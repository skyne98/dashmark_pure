use crate::fast_list::FastList;

#[derive(Clone, Debug)]
pub struct SpatialCell {
    pub atoms: FastList<usize, 16>,
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
    pub width: u32,
    pub height: u32,
    pub data: Vec<SpatialCell>,
    pub cell_size: f32,
}

impl SpatialGrid {
    pub fn new(width: u32, height: u32, cell_size: f32) -> Self {
        let mut data = Vec::with_capacity((width * height) as usize);
        for _ in 0..(width * height) {
            data.push(SpatialCell::new());
        }

        Self {
            width,
            height,
            data: data,
            cell_size,
        }
    }

    // Cells laytout
    // 0 2 4
    // 1 3 5
    pub fn add_atom(&mut self, atom: usize, x: u32, y: u32) {
        let x = x.min(self.width - 1);
        let y = y.min(self.height - 1);
        let index = x * self.height + y;
        self.data[index as usize].add_atom(atom);
    }

    pub fn add_atom_world(&mut self, atom: usize, x: f32, y: f32) {
        let (x, y) = world_to_grid(x, y, self.cell_size);
        self.add_atom(atom, x as u32, y as u32);
    }

    pub fn get_at(&self, x: u32, y: u32) -> Option<&SpatialCell> {
        self.data.get((x * self.height + y) as usize)
    }

    pub fn get_at_mut(&mut self, x: u32, y: u32) -> Option<&mut SpatialCell> {
        self.data.get_mut((x * self.height + y) as usize)
    }

    pub fn get_at_wrap(&self, x: i32, y: i32) -> Option<&SpatialCell> {
        let x = if x < 0 { 0 } else { x as u32 };
        let y = if y < 0 { 0 } else { y as u32 };

        self.get_at(x % self.width, y % self.height)
    }

    pub fn get(&self, index: usize) -> Option<&SpatialCell> {
        self.data.get(index as usize)
    }

    pub fn get_neighbours(&self, index: usize) -> [usize; 9] {
        let len = self.data.len() - 1;
        let height = self.height as usize;
        let idx_up = index.saturating_add(height);
        let idx_down = index.saturating_sub(height);

        let clamp = |value: usize| value.min(len);

        [
            clamp(index.saturating_sub(1)),
            index,
            clamp(index + 1),
            clamp(idx_up.saturating_sub(1)),
            clamp(idx_up),
            clamp(idx_up + 1),
            clamp(idx_down.saturating_sub(1)),
            clamp(idx_down),
            clamp(idx_down + 1),
        ]
    }

    pub fn clear(&mut self) {
        for cell in self.data.iter_mut() {
            cell.atoms.clear();
        }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }
}

pub fn world_to_grid(x: f32, y: f32, cell: f32) -> (i32, i32) {
    ((x / cell) as i32, (y / cell) as i32)
}
