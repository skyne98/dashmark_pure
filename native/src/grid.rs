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
#[inline]
pub fn djb2_hash(x: i32, y: i32, l: u8, len: usize) -> usize {
    let mut hash = 5381;
    hash = hash * 33 + x;
    hash = hash * 33 + y;
    hash = hash * 33 + l as i32;
    hash as usize % len
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
    pub atoms: SmallVec<[T; N]>,
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
            atoms: SmallVec::new(),
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
pub struct SpatialHash<T, const CN: usize, C = Bucket<T, CN>>
where
    [T; CN]: Array<Item = T>,
    T: Clone + Copy + Debug + PartialEq + PartialOrd,
{
    pub data: Vec<C>,
    pub levels: HashSet<u8>,
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
            levels: HashSet::new(),
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
    #[inline]
    pub fn longest_side(&self, aabb: &FastAabb) -> f32 {
        let min = aabb.mins;
        let max = aabb.maxs;
        let x = max.x - min.x;
        let y = max.y - min.y;
        x.max(y)
    }
    #[inline]
    pub fn level_for_side(&self, side: f32) -> u8 {
        f32::log2(side).floor() as u8
    }
    #[inline]
    pub fn cell_size_for_level(&self, level: u8) -> u32 {
        2u32.pow(level as u32)
    }
    #[inline]
    pub fn world_to_grid(&self, world: f32, level: u8) -> i32 {
        let world = world as i32;
        world / self.cell_size_for_level(level) as i32
    }
    pub fn clear(&mut self) {
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
        let level = self.level_for_side(longest_side);

        let mins = aabb.mins;
        let mins_grid_x = self.world_to_grid(mins.x, level);
        let mins_grid_y = self.world_to_grid(mins.y, level);
        let maxs_grid_x = self.world_to_grid(aabb.maxs.x, level);
        let maxs_grid_y = self.world_to_grid(aabb.maxs.y, level);

        let grid_width = maxs_grid_x - mins_grid_x + 1;
        let grid_height = maxs_grid_y - mins_grid_y + 1;
        for x in 0..grid_width {
            for y in 0..grid_height {
                let hash = djb2_hash(mins_grid_x + x, mins_grid_y + y, level, self.data.len());
                let bucket = &mut self.data[hash];
                bucket.atoms.retain(|a| *a != atom);
            }
        }
    }
    pub fn add(&mut self, atom: T, aabb: FastAabb) {
        let longest_side = self.longest_side(&aabb);
        let level = self.level_for_side(longest_side);
        self.levels.insert(level);

        let mins = aabb.mins;
        let mins_grid_x = self.world_to_grid(mins.x, level);
        let mins_grid_y = self.world_to_grid(mins.y, level);
        let maxs_grid_x = self.world_to_grid(aabb.maxs.x, level);
        let maxs_grid_y = self.world_to_grid(aabb.maxs.y, level);

        let grid_width = maxs_grid_x - mins_grid_x + 1;
        let grid_height = maxs_grid_y - mins_grid_y + 1;
        for x in 0..grid_width {
            for y in 0..grid_height {
                let hash = djb2_hash(mins_grid_x + x, mins_grid_y + y, level, self.data.len());
                let bucket = &mut self.data[hash];
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
    level_iter: Peekable<Iter<'a, u8>>,
    x: i32,
    y: i32,
}

impl<'a, T, const CN: usize> QueryIterator<'a, T, CN>
where
    [T; CN]: Array<Item = T>,
    T: Clone + Copy + Debug + PartialEq + PartialOrd,
{
    pub fn new(hash_grid: &'a SpatialHash<T, CN>, aabb: FastAabb) -> Self {
        Self {
            hash_grid,
            aabb,
            level_iter: hash_grid.levels.iter().peekable(),
            x: 0,
            y: 0,
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
        let level = **self.level_iter.peek()?;

        // Get the grid coordinates for the current level
        let mins_grid_x = self.hash_grid.world_to_grid(self.aabb.mins.x, level);
        let mins_grid_y = self.hash_grid.world_to_grid(self.aabb.mins.y, level);
        let maxs_grid_x = self.hash_grid.world_to_grid(self.aabb.maxs.x, level);
        let maxs_grid_y = self.hash_grid.world_to_grid(self.aabb.maxs.y, level);

        // Get the grid width and height
        let grid_width = maxs_grid_x - mins_grid_x + 1;
        let grid_height = maxs_grid_y - mins_grid_y + 1;

        // If we're out of bounds, move to the next level
        if self.x >= grid_width || self.y >= grid_height {
            self.level_iter.next();
            self.x = 0;
            self.y = 0;
            return self.next();
        }

        // Get the hash for the current grid cell
        let hash = djb2_hash(
            mins_grid_x + self.x,
            mins_grid_y + self.y,
            level,
            self.hash_grid.data.len(),
        );

        // Get the bucket for the current grid cell
        let bucket = &self.hash_grid.data[hash];

        // Move to the next grid cell
        self.x += 1;
        if self.x >= grid_width {
            self.x = 0;
            self.y += 1;
        }

        // Return the bucket
        Some(&bucket.atoms[..])
    }
}
