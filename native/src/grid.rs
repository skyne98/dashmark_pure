use std::hash::{BuildHasher, Hasher};

use rapier2d::parry::partitioning::Qbvh;
use smallvec::SmallVec;

use crate::{
    fast_list::{Clearable, FastHashMap, FastList},
    verlet::{FastAabb, FastVector2},
};

// ==================================
// HASHER
// ==================================
pub fn morton_code(x: i32, y: i32) -> i64 {
    let mut code = 0;
    for i in 0..32 {
        code |= ((x >> i) & 1) << (i * 2);
        code |= ((y >> i) & 1) << (i * 2 + 1);
    }
    code as i64
}

pub fn djb2_hash(x: i32, y: i32) -> i32 {
    let mut hash = 5381;
    hash = hash * 33 + x;
    hash = hash * 33 + y;
    hash
}

// ==================================
// CELL
// ==================================
#[derive(Clone, Debug)]
pub struct SpatialCell<const N: usize> {
    pub atoms: FastList<u16, N>,
}

impl<const N: usize> SpatialCell<N> {
    pub fn n() -> usize {
        N
    }

    pub fn new(aabb: FastAabb) -> Self {
        Self {
            atoms: FastList::new(),
        }
    }

    pub fn add_atom(&mut self, atom: u16, aabb: FastAabb) {
        self.atoms.push(atom);
    }

    pub fn atoms(&self) -> &[u16] {
        &self.atoms.data()
    }

    pub fn clear(&mut self) {
        self.atoms.clear();
    }

    pub fn len(&self) -> usize {
        self.atoms.len()
    }
}

impl<const N: usize> Clearable for SpatialCell<N> {
    fn clear(&mut self) {
        self.atoms.clear();
    }
}

// ==================================
// COLLISION ITERATOR
// ==================================
/// Internally iterates over all potential collisions (combinations of atoms) in all the cells.
pub struct CollisionIterator<'a, const N: usize> {
    grid: &'a SpatialGrid<N>,
    cell_index: usize,
    atom_index: usize,
}

impl<'a, const N: usize> CollisionIterator<'a, N> {
    pub fn new(grid: &'a SpatialGrid<N>) -> Self {
        Self {
            grid,
            cell_index: 0,
            atom_index: 0,
        }
    }
}

impl<'a, const N: usize> Iterator for CollisionIterator<'a, N> {
    type Item = (u16, Box<FastList<u16, N>>);
    fn next(&mut self) -> Option<Self::Item> {
        unsafe {
            while self.cell_index < self.grid.data.len() {
                let cell = self.grid.data.get_unchecked(self.cell_index);
                if self.atom_index < cell.atoms.len() {
                    let atom = cell.atoms[self.atom_index];
                    self.atom_index += 1;
                    return Some((atom, Box::new(cell.atoms.clone())));
                } else {
                    self.cell_index += 1;
                    self.atom_index = 0;
                }
            }
        }
        None
    }
}

// ==================================
// GRID
// ==================================
pub struct SpatialGrid<const CN: usize, C = SpatialCell<CN>> {
    pub data: Vec<C>,
    pub width: usize,
    pub height: usize,
    pub min_grid_x: i32,
    pub min_grid_y: i32,
    pub cell_size: f32,
}

impl<const CN: usize> SpatialGrid<CN> {
    pub fn new(cell_size: f32) -> Self {
        Self {
            data: Vec::new(),
            width: 0,
            height: 0,
            min_grid_x: 0,
            min_grid_y: 0,
            cell_size,
        }
    }

    pub fn set_cell_size(&mut self, cell_size: f32) {
        self.cell_size = cell_size;
    }
    pub fn cells(&self) -> &[SpatialCell<CN>] {
        &self.data
    }

    #[inline]
    pub fn local_grid_coords_to_index(&self, x: u32, y: u32) -> usize {
        let x = x as usize;
        let y = y as usize;
        x * self.height + y
    }
    #[inline]
    pub fn index_to_local_grid_coords(&self, index: usize) -> (i32, i32) {
        let x = index / self.height;
        let y = index % self.height;
        (x as i32, y as i32)
    }
    #[inline]
    pub fn grid_to_index(&self, x: i32, y: i32) -> usize {
        let x = x - self.min_grid_x;
        let y = y - self.min_grid_y;
        let x = x as u32;
        let y = y as u32;
        self.local_grid_coords_to_index(x, y)
    }

    pub fn clear_and_rebuild(&mut self, aabb: &[FastAabb]) {
        if aabb.is_empty() {
            self.data.clear();
            self.width = 0;
            self.height = 0;
            self.min_grid_x = 0;
            self.min_grid_y = 0;
            return;
        }

        // Find world bounds
        let mut world_min_x = f32::MAX;
        let mut world_min_y = f32::MAX;
        let mut world_max_x = f32::MIN;
        let mut world_max_y = f32::MIN;
        for aabb in aabb {
            world_min_x = world_min_x.min(aabb.mins.x);
            world_min_y = world_min_y.min(aabb.mins.y);
            world_max_x = world_max_x.max(aabb.maxs.x);
            world_max_y = world_max_y.max(aabb.maxs.y);
        }

        // Find grid bounds
        self.min_grid_x = (world_min_x / self.cell_size).floor() as i32;
        self.min_grid_y = (world_min_y / self.cell_size).floor() as i32;
        let max_grid_x = (world_max_x / self.cell_size).floor() as i32;
        let max_grid_y = (world_max_y / self.cell_size).floor() as i32;
        let grid_width = (max_grid_x - self.min_grid_x + 1) as usize;
        let grid_height = (max_grid_y - self.min_grid_y + 1) as usize;
        self.width = grid_width;
        self.height = grid_height;

        // Allocate and put the aabbs
        self.data = Vec::with_capacity(grid_width * grid_height);
        for x in 0..grid_width {
            for y in 0..grid_height {
                let mins = FastVector2::new(
                    (x as i32 + self.min_grid_x) as f32 * self.cell_size,
                    (y as i32 + self.min_grid_y) as f32 * self.cell_size,
                );
                let maxs = FastVector2::new(
                    (x as i32 + self.min_grid_x + 1) as f32 * self.cell_size,
                    (y as i32 + self.min_grid_y + 1) as f32 * self.cell_size,
                );
                let aabb = FastAabb::new(mins, maxs);
                self.data.push(SpatialCell::new(aabb));
            }
        }

        // Add the aabbs
        for (obj_index, aabb) in aabb.iter().enumerate() {
            let mins_grid = world_to_grid(aabb.mins.x, aabb.mins.y, self.cell_size);
            let maxs_grid = world_to_grid(aabb.maxs.x, aabb.maxs.y, self.cell_size);
            let obj_width = maxs_grid[0] - mins_grid[0];
            let obj_height = maxs_grid[1] - mins_grid[1];

            for y in 0..obj_height + 1 {
                for x in 0..obj_width + 1 {
                    let index = self.grid_to_index(mins_grid[0] + x, mins_grid[1] + y);
                    self.data[index].add_atom(obj_index as u16, *aabb);
                }
            }
        }
    }

    pub fn iter_collisions(&self) -> CollisionIterator<CN> {
        CollisionIterator::new(&self)
    }

    pub fn clear(&mut self) {
        self.data.clear();
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
}

pub fn world_to_grid(x: f32, y: f32, cell_size: f32) -> [i32; 2] {
    let x = x / cell_size;
    let y = y / cell_size;

    [x as i32, y as i32]
}

// ==================================
// CIRCLE QBVH
// ==================================
pub struct CircleQBVH {
    pub data: Qbvh<usize>,
}

impl CircleQBVH {
    pub fn new() -> Self {
        Self { data: Qbvh::new() }
    }

    pub fn clear_and_rebuild(&mut self, positions: &[FastVector2], radii: &[f32]) {
        self.data.clear_and_rebuild(
            positions
                .iter()
                .enumerate()
                .zip(radii.iter())
                .map(|((i, p), r)| {
                    let aabb: rapier2d::parry::bounding_volume::Aabb =
                        rapier2d::geometry::Aabb::new(
                            rapier2d::na::Vector2::new(p.x - r, p.y - r).into(),
                            rapier2d::na::Vector2::new(p.x + r, p.y + r).into(),
                        );
                    (i, aabb)
                }),
            0.0,
        );
    }

    pub fn query(&self, x: f32, y: f32, radius: f32) -> Vec<usize> {
        let min = rapier2d::na::Vector2::new(x - radius, y - radius);
        let max = rapier2d::na::Vector2::new(x + radius, y + radius);

        let aabb = rapier2d::geometry::Aabb::new(min.into(), max.into());

        let mut result = Vec::with_capacity(8);
        self.data.intersect_aabb(&aabb, &mut result);
        result
    }
}
