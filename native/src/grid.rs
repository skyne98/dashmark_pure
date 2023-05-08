use ahash::AHashMap;
use rapier2d::na::Vector2;

use crate::fast_list::FastList;

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

pub struct SpatialGrid {
    pub data: AHashMap<u32, SpatialCell>,
    pub cell_size: f32,
}

impl SpatialGrid {
    pub fn new(cell_size: f32) -> Self {
        Self {
            data: AHashMap::default(),
            cell_size,
        }
    }

    pub fn djb2_hash(x: u32, y: u32) -> u32 {
        let mut hash = 5381;
        hash = hash * 33 + x;
        hash = hash * 33 + y;
        hash
    }

    // Cells laytout
    // 0 2 4
    // 1 3 5
    pub fn add_atom(&mut self, atom: usize, x: f32, y: f32) {
        let grid_pos = world_to_grid(x, y, self.cell_size);
        let hash = Self::djb2_hash(grid_pos.0, grid_pos.1);
        let cell = self.data.entry(hash).or_insert_with(SpatialCell::new);
        cell.add_atom(atom);
    }

    pub fn add_atom_aabb(&mut self, atom: usize, x: f32, y: f32, radius: f32) {
        let min = Vector2::new(x - radius, y - radius);
        let max = Vector2::new(x + radius, y + radius);

        let min_grid = world_to_grid(min.x, min.y, self.cell_size);
        let max_grid = world_to_grid(max.x, max.y, self.cell_size);

        let width = max_grid.0 - min_grid.0;
        let height = max_grid.1 - min_grid.1;

        for x in 0..width + 1 {
            for y in 0..height + 1 {
                let hash = Self::djb2_hash(min_grid.0 + x, min_grid.1 + y);
                let cell = self.data.entry(hash).or_insert_with(SpatialCell::new);
                cell.add_atom(atom);
            }
        }
    }

    pub fn get_at(&self, x: f32, y: f32) -> Option<&SpatialCell> {
        let grid_pos = world_to_grid(x, y, self.cell_size);
        let hash = Self::djb2_hash(grid_pos.0, grid_pos.1);
        self.data.get(&hash)
    }

    pub fn query(&self, x: f32, y: f32, radius: f32) -> Vec<&FastList<usize, 32>> {
        let min = Vector2::new(x - radius, y - radius);
        let max = Vector2::new(x + radius, y + radius);

        let min_grid = world_to_grid(min.x, min.y, self.cell_size);
        let max_grid = world_to_grid(max.x, max.y, self.cell_size);

        let width = max_grid.0 - min_grid.0;
        let height = max_grid.1 - min_grid.1;

        let mut result = Vec::with_capacity(4);
        for x in 0..width + 1 {
            for y in 0..height + 1 {
                let hash = Self::djb2_hash(min_grid.0 + x, min_grid.1 + y);
                if let Some(cell) = self.data.get(&hash) {
                    result.push(&cell.atoms);
                }
            }
        }

        result
    }

    pub fn clear(&mut self) {
        for (_, cell) in self.data.iter_mut() {
            cell.atoms.clear();
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
}

pub fn world_to_grid(x: f32, y: f32, cell_size: f32) -> (u32, u32) {
    let x = x / cell_size;
    let y = y / cell_size;

    (x as u32, y as u32)
}
