// Column-major order (meaning at index 1 we have the second row and first column).
// Contiguous memory.
// Each cell contains a current length of stored handles.
// Each cell contains an array of 4 handles or a heap-allocated list of arbitrary length.
// ^ SmallVec

// An iterator is implemented to iterate over the grid in the most cache-friendly way possible.
// Another function is provided to subdivide the grid into smaller grids with cache-friendly iteration
// ... for use in multi-threaded iteration.

use std::{cmp::min, ops::Deref};

use rapier2d::{na::Vector2, parry::bounding_volume::Aabb};
use smallvec::SmallVec;

pub type GridHandle = u32;

pub struct SpatialGrid {
    start_x: u32, // in floor(coordinate / cell_size) units
    start_y: u32, // in floor(coordinate / cell_size) units
    width: u32,
    height: u32,
    cell_size: f32,
    data: Vec<GridCell>,
}

impl SpatialGrid {
    // Construct from an iterator of Aabbs
    pub fn from_aabbs<A: Deref<Target = [Aabb]>>(aabbs: &A) -> Self {
        // First, find the required size of the grid
        let mut min_x = f32::MAX;
        let mut min_y = f32::MAX;
        let mut max_x = f32::MIN;
        let mut max_y = f32::MIN;

        let mut min_aabb_side = f32::MAX;
        let mut max_aabb_side = f32::MIN;

        for aabb in aabbs.iter() {
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

        // Add some padding to the max values so we can subdivide the grid
        // based on the cell size
        let current_cells_width = (max_x - min_x) / cell_size;
        let current_cells_height = (max_y - min_y) / cell_size;
        let expected_cells_width = current_cells_width.ceil() as u32;
        let expected_cells_height = current_cells_height.ceil() as u32;

        let width = expected_cells_width;
        let height = expected_cells_height;

        // Now allocate the data
        let mut data = vec![
            GridCell {
                handles: SmallVec::new(),
            };
            (width * height) as usize
        ];

        // Find the starting point of the grid
        let start_x = (min_x / cell_size).floor() as u32;
        let start_y = (min_y / cell_size).floor() as u32;

        // Now insert the aabbs into the grid
        // by inserting the min and max points and then
        // inserting into all cells in between them
        for (i, aabb) in aabbs.iter().enumerate() {
            let min_x = (aabb.mins.x / cell_size).floor() as u32;
            let min_y = (aabb.mins.y / cell_size).floor() as u32;
            let max_x = (aabb.maxs.x / cell_size).floor() as u32;
            let max_y = (aabb.maxs.y / cell_size).floor() as u32;

            for x in 0..=(max_x - min_x) {
                for y in 0..=(max_y - min_y) {
                    let index = y + x * height;
                    data[index as usize].handles.push(i as u32);
                }
            }
        }

        Self {
            start_x,
            start_y,
            width,
            height,
            cell_size,
            data,
        }
    }
}

#[derive(Clone)]
pub struct GridCell {
    handles: SmallVec<[GridHandle; 4]>,
}
