use std::hash::{BuildHasher, Hasher};

use rapier2d::na::Vector2;
use rapier2d::parry::partitioning::Qbvh;

use crate::{
    fast_list::{Clearable, FastHashMap, FastList},
    verlet::FastVector2,
};

// ==================================
// HASHER
// ==================================
pub fn morton_code(x: i32, y: i32) -> i32 {
    let mut hash = 0;
    for i in 0..32 {
        hash |= (x & (1 << i)) << i | (y & (1 << i)) << (i + 1);
    }
    hash
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
pub struct SpatialCell {
    pub atoms: FastList<usize, 32>,
}

impl SpatialCell {
    pub fn new() -> Self {
        Self {
            atoms: FastList::new(),
        }
    }

    pub fn add_atom(&mut self, atom: usize) {
        self.atoms.push(atom);
    }

    pub fn atoms(&self) -> &[usize] {
        &self.atoms.data()
    }

    pub fn clear(&mut self) {
        self.atoms.clear();
    }
}

impl Clearable for SpatialCell {
    fn clear(&mut self) {
        self.atoms.clear();
    }
}

// ==================================
// GRID
// ==================================
pub struct SpatialGrid {
    pub data: FastHashMap<SpatialCell, 32000>,
    pub cell_size: f32,
}

impl SpatialGrid {
    pub fn new(cell_size: f32) -> Self {
        Self {
            data: FastHashMap::new(|| SpatialCell::new()),
            cell_size,
        }
    }

    pub fn hash(x: i32, y: i32) -> i32 {
        morton_code(x, y)
    }

    // Cells laytout
    // 0 2 4
    // 1 3 5
    pub fn add_atom(&mut self, atom: usize, x: f32, y: f32) {
        let pos = world_to_grid(x, y, self.cell_size);
        let cell = self.data.get_mut(Self::hash(pos[0], pos[1]));
        cell.add_atom(atom);
    }

    pub fn add_atom_aabb(&mut self, atom: usize, x: f32, y: f32, radius: f32) {
        let min = Vector2::new(x - radius, y - radius);
        let max = Vector2::new(x + radius, y + radius);

        let min_grid = world_to_grid(min.x, min.y, self.cell_size);
        let max_grid = world_to_grid(max.x, max.y, self.cell_size);

        let width = max_grid[0] - min_grid[0];
        let height = max_grid[1] - min_grid[1];

        for x in 0..width + 1 {
            for y in 0..height + 1 {
                let pos = [min_grid[0] + x, min_grid[1] + y];
                let cell = self.data.get_mut(Self::hash(pos[0], pos[1]));
                cell.add_atom(atom);
            }
        }
    }

    pub fn get_at(&self, x: f32, y: f32) -> &SpatialCell {
        let grid_pos = world_to_grid(x, y, self.cell_size);
        self.data.get(Self::hash(grid_pos[0], grid_pos[1]))
    }

    pub fn query(&self, x: f32, y: f32, radius: f32) -> Vec<&FastList<usize, 32>> {
        let min = Vector2::new(x - radius, y - radius);
        let max = Vector2::new(x + radius, y + radius);

        let min_grid = world_to_grid(min.x, min.y, self.cell_size);
        let max_grid = world_to_grid(max.x, max.y, self.cell_size);

        let width = max_grid[0] - min_grid[0];
        let height = max_grid[1] - min_grid[1];

        let mut result = Vec::with_capacity(4);
        for x in 0..width + 1 {
            for y in 0..height + 1 {
                let pos = [min_grid[0] + x, min_grid[1] + y];
                let cell = self.data.get(Self::hash(pos[0], pos[1]));
                result.push(&cell.atoms);
            }
        }

        result
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
                    let aabb = rapier2d::geometry::Aabb::new(
                        Vector2::new(p.x - r, p.y - r).into(),
                        Vector2::new(p.x + r, p.y + r).into(),
                    );
                    (i, aabb)
                }),
            0.0,
        );
    }

    pub fn query(&self, x: f32, y: f32, radius: f32) -> Vec<usize> {
        let min = Vector2::new(x - radius, y - radius);
        let max = Vector2::new(x + radius, y + radius);

        let aabb = rapier2d::geometry::Aabb::new(min.into(), max.into());

        let mut result = Vec::with_capacity(8);
        self.data.intersect_aabb(&aabb, &mut result);
        result
    }
}
