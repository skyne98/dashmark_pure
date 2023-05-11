use std::{
    collections::{hash_set::Iter, HashSet},
    fmt::Debug,
    hash::{BuildHasher, Hasher},
    iter::Peekable,
};

use rapier2d::parry::partitioning::Qbvh;
use smallvec::{Array, SmallVec};

use crate::{
    fast_list::Clearable,
    verlet::{FastAabb, FastVector2},
};

// ==================================
// HASHER
// ==================================
pub fn hash(x: i32, y: i32, l: u8, len: u64) -> u64 {
    let x = x as u64;
    let y = y as u64;
    let l = l as u64;

    // Encode x and y using the morton encoding
    let x = (x | (x << 16)) & 0x0000ffff0000ffff;
    let x = (x | (x << 8)) & 0x00ff00ff00ff00ff;
    let x = (x | (x << 4)) & 0x0f0f0f0f0f0f0f0f;
    let x = (x | (x << 2)) & 0x3333333333333333;
    let x = (x | (x << 1)) & 0x5555555555555555;

    let y = (y | (y << 16)) & 0x0000ffff0000ffff;
    let y = (y | (y << 8)) & 0x00ff00ff00ff00ff;
    let y = (y | (y << 4)) & 0x0f0f0f0f0f0f0f0f;
    let y = (y | (y << 2)) & 0x3333333333333333;
    let y = (y | (y << 1)) & 0x5555555555555555;

    let l = l << 62;

    let hash = x | (y << 1) | l;
    hash % len
}

// ==================================
// CELL
// ==================================
#[derive(Clone, Debug)]
pub struct Bucket<T, const N: usize>
where
    [T; N]: Array<Item = T>,
    T: Clone + Copy + Debug + PartialEq + PartialOrd,
{
    pub atoms: Vec<T>,
}

impl<T, const N: usize> Bucket<T, N>
where
    [T; N]: Array<Item = T>,
    T: Clone + Copy + Debug + PartialEq + PartialOrd,
{
    pub fn n() -> usize {
        N
    }

    pub fn new() -> Self {
        Self {
            atoms: Vec::with_capacity(N),
        }
    }

    pub fn add_atom(&mut self, atom: T) {
        self.atoms.push(atom);
    }

    pub fn atoms(&self) -> &[T] {
        &self.atoms
    }

    pub fn clear(&mut self) {
        self.atoms.clear();
    }

    pub fn len(&self) -> usize {
        self.atoms.len()
    }
}

// ==================================
// GRID
// ==================================
// Based on "Hierarchical Spatial Hashing for Real-time Collision Detection"
pub struct SpatialHash<T, const CN: usize, C = Bucket<T, CN>>
where
    [T; CN]: Array<Item = T>,
    T: Clone + Copy + Debug + PartialEq + PartialOrd,
{
    pub data: Vec<C>,
    pub levels: Vec<u8>,
    type_phantom: std::marker::PhantomData<T>,
}

impl<T, const CN: usize> SpatialHash<T, CN>
where
    [T; CN]: Array<Item = T>,
    T: Clone + Copy + Debug + PartialEq + PartialOrd,
{
    pub fn new(capacity: usize) -> Self {
        Self {
            data: vec![Bucket::new(); capacity],
            levels: Vec::with_capacity(16),
            type_phantom: std::marker::PhantomData,
        }
    }
    pub fn clear_and_rebuild<E>(&mut self, atoms: &[FastAabb])
    where
        E: std::error::Error,
        T: TryFrom<usize, Error = E>,
    {
        self.clear();
        self.rebuild(atoms);
    }

    pub fn buckets(&self) -> &[Bucket<T, CN>] {
        &self.data
    }

    // Utilities

    pub fn longest_side(&self, aabb: &FastAabb) -> f32 {
        let min = aabb.mins;
        let max = aabb.maxs;
        let x = max.x - min.x;
        let y = max.y - min.y;
        x.max(y)
    }

    pub fn level_for_side(&self, side: u32) -> u8 {
        u32::ilog2(side) as u8
    }

    pub fn cell_size_for_level(&self, level: u8) -> u32 {
        2u32.pow(level as u32)
    }

    pub fn world_to_grid(&self, world: f32, level: u8) -> i32 {
        let world = world as i32;
        world / self.cell_size_for_level(level) as i32
    }
    pub fn clear(&mut self) {
        // Give analysis of the current buckets
        let bucket_count = self.data.len();
        let mut max_bucket_len = 0;
        let mut total_atoms = 0;
        for bucket in &self.data {
            let len = bucket.len();
            total_atoms += len;
            max_bucket_len = max_bucket_len.max(len);
        }
        let avg_bucket_len = total_atoms / bucket_count;
        log::debug!(
            "SpatialHash::clear() - bucket_count: {}, max_bucket_len: {}, avg_bucket_len: {}",
            bucket_count,
            max_bucket_len,
            avg_bucket_len
        );

        self.levels.clear();
        for bucket in &mut self.data {
            bucket.clear();
        }
    }
    pub fn rebuild<E>(&mut self, atoms: &[FastAabb])
    where
        E: std::error::Error,
        T: TryFrom<usize, Error = E>,
    {
        for (atom, aabb) in atoms.iter().enumerate() {
            let atom = atom.try_into().unwrap();
            self.add(atom, *aabb);
        }
    }
    pub fn len(&self) -> usize {
        self.data.len()
    }

    // ==================================
    // Management functions
    // ==================================
    pub fn remove(&mut self, atom: T) {
        for bucket in &mut self.data {
            bucket.atoms.retain(|a| *a != atom);
        }
    }
    pub fn remove_with_aabb(&mut self, atom: T, aabb: FastAabb) {
        let longest_side = self.longest_side(&aabb);
        let level = self.level_for_side(longest_side as u32);

        let mins = aabb.mins;
        let mins_grid_x = self.world_to_grid(mins.x, level);
        let mins_grid_y = self.world_to_grid(mins.y, level);
        let maxs_grid_x = self.world_to_grid(aabb.maxs.x, level);
        let maxs_grid_y = self.world_to_grid(aabb.maxs.y, level);

        let grid_width = maxs_grid_x - mins_grid_x + 1;
        let grid_height = maxs_grid_y - mins_grid_y + 1;
        for x in 0..grid_width {
            for y in 0..grid_height {
                let hash = hash(
                    mins_grid_x + x,
                    mins_grid_y + y,
                    level,
                    self.data.len() as u64,
                );
                let bucket = &mut self.data[hash as usize];
                bucket.atoms.retain(|a| *a != atom);
            }
        }
    }
    pub fn add(&mut self, atom: T, aabb: FastAabb) {
        let longest_side = self.longest_side(&aabb);
        let level = self.level_for_side(longest_side as u32);
        if self.levels.contains(&level) == false {
            self.levels.push(level);
        }

        let mins = aabb.mins;
        let mins_grid_x = self.world_to_grid(mins.x, level);
        let mins_grid_y = self.world_to_grid(mins.y, level);
        let maxs_grid_x = self.world_to_grid(aabb.maxs.x, level);
        let maxs_grid_y = self.world_to_grid(aabb.maxs.y, level);

        let grid_width = maxs_grid_x - mins_grid_x + 1;
        let grid_height = maxs_grid_y - mins_grid_y + 1;
        for x in 0..grid_width {
            for y in 0..grid_height {
                let hash = hash(
                    mins_grid_x + x,
                    mins_grid_y + y,
                    level,
                    self.data.len() as u64,
                );
                let bucket = &mut self.data[hash as usize];
                bucket.add_atom(atom.clone());
            }
        }
    }
    pub fn update(&mut self, atom: T, aabb: FastAabb) {
        self.remove(atom.clone());
        self.add(atom, aabb);
    }

    // ==================================
    // Query functions
    // ==================================
    pub fn query(&self, aabb: FastAabb) -> QueryIterator<T, CN> {
        QueryIterator::new(self, aabb)
    }
}

// ==================================
// Query Iterator
// ==================================
pub struct QueryIterator<'a, T, const CN: usize>
where
    [T; CN]: Array<Item = T>,
    T: Clone + Copy + Debug + PartialEq + PartialOrd,
{
    hash_grid: &'a SpatialHash<T, CN>,
    aabb: FastAabb,

    // Current state
    level_iterator: std::slice::Iter<'a, u8>,
    current_level: Option<&'a u8>,
    grid_width: i32,
    grid_height: i32,
    mins_grid_x: i32,
    mins_grid_y: i32,
    maxs_grid_x: i32,
    maxs_grid_y: i32,
    x: i32,
    y: i32,
}

impl<'a, T, const CN: usize> QueryIterator<'a, T, CN>
where
    [T; CN]: Array<Item = T>,
    T: Clone + Copy + Debug + PartialEq + PartialOrd,
{
    pub fn new(grid: &'a SpatialHash<T, CN>, aabb: FastAabb) -> Self {
        let mut iterator = Self {
            hash_grid: grid,
            aabb,
            level_iterator: grid.levels.iter(),
            current_level: None,
            grid_width: 0,
            grid_height: 0,
            mins_grid_x: 0,
            mins_grid_y: 0,
            maxs_grid_x: 0,
            maxs_grid_y: 0,
            x: 0,
            y: 0,
        };
        iterator.advance_level();
        iterator
    }

    fn advance_level(&mut self) {
        self.current_level = self.level_iterator.next();
        if let Some(level) = self.current_level {
            // Get the grid coordinates for the current level
            self.mins_grid_x = self.hash_grid.world_to_grid(self.aabb.mins.x, *level);
            self.mins_grid_y = self.hash_grid.world_to_grid(self.aabb.mins.y, *level);
            self.maxs_grid_x = self.hash_grid.world_to_grid(self.aabb.maxs.x, *level);
            self.maxs_grid_y = self.hash_grid.world_to_grid(self.aabb.maxs.y, *level);
            self.grid_width = self.maxs_grid_x - self.mins_grid_x + 1;
            self.grid_height = self.maxs_grid_y - self.mins_grid_y + 1;
            self.x = 0;
            self.y = 0;
        }
    }
}

impl<'a, T, const CN: usize> Iterator for QueryIterator<'a, T, CN>
where
    [T; CN]: Array<Item = T>,
    T: Clone + Copy + Debug + PartialEq + PartialOrd,
{
    type Item = &'a [T];

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.current_level {
                None => return None,
                Some(level) => {
                    // If we're out of bounds, move to the next level
                    if self.x >= self.grid_width || self.y >= self.grid_height {
                        self.advance_level();
                        continue;
                    }

                    // Get the hash for the current grid cell
                    let hash = hash(
                        self.mins_grid_x + self.x,
                        self.mins_grid_y + self.y,
                        *level,
                        self.hash_grid.data.len() as u64,
                    );

                    // Get the bucket for the current grid cell
                    let bucket = &self.hash_grid.data[hash as usize];

                    // Move to the next grid cell
                    self.x += 1;
                    if self.x >= self.grid_width {
                        self.x = 0;
                        self.y += 1;
                    }

                    // Return the bucket
                    return Some(&bucket.atoms[..]);
                }
            }
        }
    }
}
