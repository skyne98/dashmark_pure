// Column-major order (meaning at index 1 we have the second row and first column).
// Contiguous memory.
// Each cell contains a current length of stored handles.
// Each cell contains an array of 4 handles or a heap-allocated list of arbitrary length.
// ^ SmallVec

// An iterator is implemented to iterate over the grid in the most cache-friendly way possible.
// Another function is provided to subdivide the grid into smaller grids with cache-friendly iteration
// ... for use in multi-threaded iteration.

use std::{i8::MIN, ops::Deref};

use rapier2d::parry::bounding_volume::Aabb;
use smallvec::SmallVec;

use crate::verlet::IntoAabb;

#[derive(Debug)]
pub struct GridHandle {
    pub object: usize,
}

pub struct SpatialGrid {
    start_x: u32, // in floor(coordinate / cell_size) units
    start_y: u32, // in floor(coordinate / cell_size) units
    width: u32,
    height: u32,
    cell_size: f32,
    objects_len: usize,
    cells: Vec<GridCell>,
}

impl SpatialGrid {
    // Construct from an iterator of Aabbs
    pub fn from_aabbs<O: IntoAabb>(objects: &[O]) -> Self {
        // First, find the required size of the grid
        let mut min_x = f32::MAX;
        let mut min_y = f32::MAX;
        let mut max_x = f32::MIN;
        let mut max_y = f32::MIN;

        let mut min_aabb_side = f32::MAX;
        let mut max_aabb_side = f32::MIN;

        for aabb in objects.iter() {
            let aabb: Aabb = aabb.into_aabb();
            let aabb_width = aabb.maxs.x - aabb.mins.x;
            let aabb_height = aabb.maxs.y - aabb.mins.y;

            if aabb_width < min_aabb_side {
                min_aabb_side = aabb_width;
            }
            if aabb_height < min_aabb_side {
                min_aabb_side = aabb_height;
            }
            if aabb_width > max_aabb_side {
                max_aabb_side = aabb_width;
            }
            if aabb_height > max_aabb_side {
                max_aabb_side = aabb_height;
            }

            if aabb.mins.x < min_x {
                min_x = aabb.mins.x;
            }
            if aabb.mins.y < min_y {
                min_y = aabb.mins.y;
            }

            if aabb.maxs.x > max_x {
                max_x = aabb.maxs.x;
            }
            if aabb.maxs.y > max_y {
                max_y = aabb.maxs.y;
            }
        }

        // Find an average size for the grid cells
        let cell_size = (min_aabb_side + max_aabb_side) / 2.0;
        let cell_size = cell_size * 1.5; // add some padding

        let width = ((max_x - min_x) / cell_size).ceil() as u32 + 1;
        let height = ((max_y - min_y) / cell_size).ceil() as u32 + 1;

        // Now allocate the data
        let mut cells = Vec::with_capacity((width * height) as usize);
        for _ in 0..(width * height) {
            cells.push(GridCell {
                handles: SmallVec::new(),
            });
        }

        // Find the starting point of the grid
        let start_x = (min_x / cell_size).floor() as u32;
        let start_y = (min_y / cell_size).floor() as u32;

        // Now insert the aabbs into the grid
        // by inserting the min and max points and then
        // inserting into all cells in between them
        for (i, aabb) in objects.iter().enumerate() {
            let aabb: Aabb = aabb.into_aabb();
            let min_x = (aabb.mins.x / cell_size).floor() as u32;
            let min_y = (aabb.mins.y / cell_size).floor() as u32;
            let max_x = (aabb.maxs.x / cell_size).floor() as u32;
            let max_y = (aabb.maxs.y / cell_size).floor() as u32;

            let min_x = min_x - start_x;
            let min_y = min_y - start_y;
            let max_x = max_x - start_x;
            let max_y = max_y - start_y;

            for x in min_x..=max_x {
                for y in min_y..=max_y {
                    let index = y + x * height;
                    cells[index as usize].handles.push(GridHandle { object: i });
                }
            }
        }

        Self {
            start_x,
            start_y,
            width,
            height,
            cell_size,
            cells,
            objects_len: objects.len(),
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (usize, &GridCell)> {
        self.cells.iter().enumerate()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (usize, &mut GridCell)> {
        self.cells.iter_mut().enumerate()
    }

    pub fn process_collisions<O, F: FnMut(&mut O, &mut O)>(&mut self, objects: &mut [O], mut f: F) {
        let mut processed_collisions = vec![vec![]; objects.len()];
        let objects_ptr = objects.as_mut_ptr();
        for (i, cell) in self.iter() {
            for (a, b) in cell.potential_collisions() {
                if processed_collisions[a].contains(&b) {
                    continue;
                }
                processed_collisions[a].push(b);
                unsafe {
                    let a = &mut *objects_ptr.add(a);
                    let b = &mut *objects_ptr.add(b);
                    f(a, b);
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct GridCell {
    handles: SmallVec<[GridHandle; 4]>,
}

impl GridCell {
    pub fn potential_collisions(&self) -> impl Iterator<Item = (usize, usize)> + '_ {
        let mut handles = self.handles.iter();
        std::iter::from_fn(move || {
            let a = handles.next()?;
            for b in handles.clone() {
                return Some((a.object, b.object));
            }
            None
        })
    }
}
