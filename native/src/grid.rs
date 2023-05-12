use std::{
    collections::{hash_set::Iter, HashSet},
    fmt::Debug,
    hash::{BuildHasher, Hasher},
    iter::Peekable,
    ops::AddAssign,
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
// Based on
/*
  hashCoords(xi: number, yi: number, zi: number) {
    const hash = (xi * 92837111) ^ (yi * 689287499) ^ (zi * 283923481);
    return Math.abs(hash);
  }
*/
pub fn hash(x: i32, y: i32, l: u8, len: u64) -> u64 {
    const X: u64 = 92837111;
    const Y: u64 = 689287499;
    const Z: u64 = 283923481;

    let hash = (x as u64 * X) ^ (y as u64 * Y) ^ (l as u64 * Z);
    hash % len
}

// ==================================
// GRID
// ==================================
// Based on "Hierarchical Spatial Hashing for Real-time Collision Detection"
// and https://carmencincotti.com/2022-10-31/spatial-hash-maps-part-one/
// which is based on https://www.youtube.com/watch?v=D2M8jTtKi44
pub struct SpatialHash {
    pub capacity: usize,
    pub cell_start: Vec<usize>,
    pub cell_entries: Vec<u16>,
    pub levels: Vec<u8>,

    type_phantom: std::marker::PhantomData<u16>,
}

impl SpatialHash {
    pub fn new() -> Self {
        Self {
            capacity: 0,
            cell_start: vec![],
            cell_entries: vec![],
            levels: Vec::with_capacity(16),
            type_phantom: std::marker::PhantomData,
        }
    }
    pub fn clear_and_rebuild(&mut self, atoms: &[FastAabb]) {
        self.clear();
        self.rebuild(atoms);
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
        self.capacity = 0;
        self.levels.clear();
        self.cell_entries.clear();
        self.cell_start.clear();
    }
    pub fn rebuild(&mut self, atoms: &[FastAabb]) {
        // Recreate the cell start list
        self.capacity = atoms.len() * 3;
        self.cell_start = vec![0; self.capacity + 1];

        // Calculate the hashes for each body and store them in a list
        for (_, aabb) in atoms.iter().enumerate() {
            let side = self.longest_side(aabb);
            let level = self.level_for_side(side as u32);
            if self.levels.contains(&level) == false {
                self.levels.push(level);
            }

            let min_x = self.world_to_grid(aabb.mins.x, level);
            let min_y = self.world_to_grid(aabb.mins.y, level);
            let max_x = self.world_to_grid(aabb.maxs.x, level);
            let max_y = self.world_to_grid(aabb.maxs.y, level);

            let grid_width = max_x - min_x;
            let grid_height = max_y - min_y;
            for x in 0..grid_width + 1 {
                for y in 0..grid_height + 1 {
                    let hash = hash(min_x + x, min_y + y, level, self.capacity as u64);
                    self.cell_start[hash as usize] += 1;
                }
            }
        }

        // Do a partial sum through the list to get the start of each cell
        let mut start: usize = 0;
        for i in 0..self.cell_start.len() {
            start += self.cell_start[i];
            self.cell_start[i] = start;
        }
        let len = self.cell_start.len();
        self.cell_start[len - 1] = start; // guard

        // Fill the atom ids
        self.cell_entries = vec![0; start];
        for (i, aabb) in atoms.iter().enumerate() {
            let side = self.longest_side(aabb);
            let level = self.level_for_side(side as u32);

            let min_x = self.world_to_grid(aabb.mins.x, level);
            let min_y = self.world_to_grid(aabb.mins.y, level);
            let max_x = self.world_to_grid(aabb.maxs.x, level);
            let max_y = self.world_to_grid(aabb.maxs.y, level);

            let grid_width = max_x - min_x;
            let grid_height = max_y - min_y;
            for x in 0..grid_width + 1 {
                for y in 0..grid_height + 1 {
                    let hash = hash(min_x + x, min_y + y, level, self.capacity as u64);
                    self.cell_start[hash as usize] -= 1;
                    self.cell_entries[self.cell_start[hash as usize]] = i as u16;
                }
            }
        }

        // Print the state of the "starts" and "entries" arrays
        // log::debug!("Cell start:");
        // for i in 0..self.cell_start.len() {
        //     log::debug!("{}: {}", i, self.cell_start[i]);
        // }
        // log::debug!("Cell entries:");
        // for i in 0..self.cell_entries.len() {
        //     log::debug!("{}: {}", i, self.cell_entries[i]);
        // }
    }
    pub fn len(&self) -> usize {
        self.cell_entries.len()
    }

    // ==================================
    // Management functions
    // ==================================
    // Nothing to see here for now

    // ==================================
    // Query functions
    // ==================================
    pub fn query(&self, aabb: FastAabb) -> QueryIterator {
        QueryIterator::new(self, aabb)
    }
}

// ==================================
// QueryIterator
// ==================================
pub struct QueryIterator<'a> {
    hash_grid: &'a SpatialHash,
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

impl<'a> QueryIterator<'a> {
    pub fn new(grid: &'a SpatialHash, aabb: FastAabb) -> Self {
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

impl<'a> Iterator for QueryIterator<'a> {
    type Item = &'a [u16];

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
                        self.hash_grid.capacity as u64,
                    );

                    // Get the bucket for the current grid cell
                    let start = self.hash_grid.cell_start[hash as usize];
                    let end = self.hash_grid.cell_start[hash as usize + 1];
                    let bucket = &self.hash_grid.cell_entries[start as usize..end as usize];

                    // Move to the next grid cell
                    self.x += 1;
                    if self.x >= self.grid_width {
                        self.x = 0;
                        self.y += 1;
                    }

                    // Return the bucket
                    return Some(bucket);
                }
            }
        }
    }
}
