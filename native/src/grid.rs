use smallvec::SmallVec;

#[derive(Clone, Debug)]
pub struct SpatialCell {
    pub atoms: SmallVec<[usize; 2]>,
}

impl SpatialCell {
    pub fn new() -> Self {
        Self {
            atoms: SmallVec::new(),
        }
    }

    pub fn add_atom(&mut self, atom: usize) {
        self.atoms.push(atom);
    }

    pub fn remove_atom(&mut self, atom: usize) {
        self.atoms.retain(|a| *a != atom);
    }

    pub fn atoms(&self) -> &[usize] {
        &self.atoms
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
        Self {
            width,
            height,
            data: vec![SpatialCell::new(); (width * height) as usize],
            cell_size,
        }
    }

    // Cells laytout
    // 0 1 2 ->
    // 3 4 5
    pub fn add_atom(&mut self, atom: usize, x: u32, y: u32) {
        if let Some(cell) = self.get_at_mut(x, y) {
            cell.add_atom(atom);
        }
    }

    pub fn add_atom_world(&mut self, atom: usize, x: f32, y: f32) {
        let (x, y) = world_to_grid(x, y, self.cell_size);
        let x = (x as u32).min(self.width - 1);
        let y = (y as u32).min(self.height - 1);
        self.add_atom(atom, x, y);
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
        let len = self.data.len() as usize - 1;
        [
            (index as i32 - 1).clamp(0, len as i32) as usize,
            index,
            (index + 1).clamp(0, len) as usize,
            (index + self.height as usize - 1).clamp(0, len) as usize,
            (index + self.height as usize).clamp(0, len) as usize,
            (index + self.height as usize + 1).clamp(0, len) as usize,
            (index - self.height as usize - 1).clamp(0, len) as usize,
            (index - self.height as usize).clamp(0, len) as usize,
            (index - self.height as usize + 1).clamp(0, len) as usize,
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
    let x = x / cell;
    let y = y / cell;

    (x as i32, y as i32)
}

pub fn world_vector_to_grid(x: f32, y: f32, cell: f32) -> (i32, i32) {
    let x = x / cell;
    let y = y / cell;

    (x as i32, y as i32)
}
