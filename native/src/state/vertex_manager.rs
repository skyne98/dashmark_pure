use generational_arena::Index;
use rapier2d_f64::na::Vector2;

pub struct VertexManager {
    pub vertices: Vec<Vec<Vector2<f64>>>,
}

impl VertexManager {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
        }
    }
    pub fn iter(&self) -> impl Iterator<Item = (usize, &Vec<Vector2<f64>>)> {
        self.vertices.iter().enumerate().map(|(i, v)| (i, v))
    }
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (usize, &mut Vec<Vector2<f64>>)> {
        self.vertices.iter_mut().enumerate().map(|(i, v)| (i, v))
    }
    pub fn add(&mut self, index: Index, vertices: Vec<Vector2<f64>>) {
        let index = index.into_raw_parts().0;
        if index >= self.vertices.len() {
            self.vertices.insert(index, vertices);
        } else {
            self.vertices[index] = vertices;
        }
    }
    pub fn set(&mut self, index: Index, vertices: Vec<Vector2<f64>>) {
        let index = index.into_raw_parts().0;
        self.vertices[index] = vertices;
    }
    pub fn get(&self, index: Index) -> Option<&Vec<Vector2<f64>>> {
        let index = index.into_raw_parts().0;
        self.vertices.get(index)
    }
    pub fn get_mut(&mut self, index: Index) -> Option<&mut Vec<Vector2<f64>>> {
        let index = index.into_raw_parts().0;
        self.vertices.get_mut(index)
    }
    pub fn get_len(&self, index: Index) -> Option<usize> {
        let index = index.into_raw_parts().0;
        self.vertices.get(index).map(|v| v.len())
    }
}

// Entity management
impl VertexManager {
    pub fn index_added(&mut self, index: Index) {
        self.add(index, Vec::new());
    }
    pub fn index_removed(&mut self, index: Index) {
        // Might not be needed at all
    }
}
