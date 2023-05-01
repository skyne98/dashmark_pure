use generational_arena::Index;
use rapier2d_f64::na::Vector2;
use smallvec::SmallVec;

use crate::matrix::bulk_transform_vectors_mut;

use super::{entity_manager::EntityManager, transform_manager::TransformManager};

#[derive(Default, Debug, Clone)]
pub struct RenderingResources {
    pub vertices: Vec<SmallVec<[Vector2<f64>; 10]>>,
    pub texCoords: Vec<SmallVec<[Vector2<f64>; 10]>>,
    pub colors: Vec<i32>,
    pub indices: Vec<SmallVec<[u16; 10]>>,
    pub batches: Vec<Batch>,
}

impl RenderingResources {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn get(
        &self,
        index: Index,
    ) -> Option<(
        &SmallVec<[Vector2<f64>; 10]>,
        &SmallVec<[Vector2<f64>; 10]>,
        &i32,
        &SmallVec<[u16; 10]>,
    )> {
        let (index, generation) = index.into_raw_parts();
        if index >= self.vertices.len() {
            None
        } else {
            Some((
                &self.vertices[index],
                &self.texCoords[index],
                &self.colors[index],
                &self.indices[index],
            ))
        }
    }

    pub fn get_mut(
        &mut self,
        index: Index,
    ) -> Option<(
        &mut SmallVec<[Vector2<f64>; 10]>,
        &mut SmallVec<[Vector2<f64>; 10]>,
        &mut i32,
        &mut SmallVec<[u16; 10]>,
    )> {
        let (index, generation) = index.into_raw_parts();
        if index >= self.vertices.len() {
            None
        } else {
            Some((
                &mut self.vertices[index],
                &mut self.texCoords[index],
                &mut self.colors[index],
                &mut self.indices[index],
            ))
        }
    }

    pub fn set_vertices<V: AsRef<[Vector2<f64>]>>(&mut self, index: Index, vertices: V) {
        let (index, generation) = index.into_raw_parts();
        if index >= self.vertices.len() {
            panic!("Index out of bounds when setting vertices");
        }
        self.vertices[index].clear();
        self.vertices[index].extend_from_slice(vertices.as_ref());
    }
    pub fn set_tex_coords<V: AsRef<[Vector2<f64>]>>(&mut self, index: Index, tex_coords: V) {
        let (index, generation) = index.into_raw_parts();
        if index >= self.vertices.len() {
            panic!("Index out of bounds when setting tex_coords");
        }
        self.texCoords[index].clear();
        self.texCoords[index].extend_from_slice(tex_coords.as_ref());
    }
    pub fn set_indices<V: AsRef<[u16]>>(&mut self, index: Index, indices: V) {
        let (index, generation) = index.into_raw_parts();
        if index >= self.vertices.len() {
            panic!("Index out of bounds when setting indices");
        }
        self.indices[index].clear();
        self.indices[index].extend_from_slice(indices.as_ref());
    }

    pub fn transformed_vertices(
        &self,
        index: Index,
        transforms: &TransformManager,
    ) -> SmallVec<[Vector2<f64>; 10]> {
        let (index_index, _) = index.into_raw_parts();
        if index_index >= self.vertices.len() {
            panic!("Index out of bounds when getting transformed vertices");
        }
        let vertices = &self.vertices[index_index];
        let transform = transforms
            .transform(index)
            .expect("Entity's transform not found");
        let mut ouput = vertices.clone();
        bulk_transform_vectors_mut(&transform.matrix(), &mut ouput);
        ouput
    }

    pub fn batchify(&mut self, entities: &mut EntityManager, transforms: &TransformManager) {
        let start = std::time::Instant::now();
        self.batches.clear();

        // Get all active entities from the entity manager
        let mut active_entities = entities.iter().collect::<Vec<_>>();
        // Sort them by their priority
        active_entities.sort_by(|a, b| {
            let (_, a) = a;
            let (_, b) = b;
            a.priority.cmp(&b.priority)
        });

        // 1. Split into batches up to 8192 vertices (not indices)
        // 2. Make sure not to split a polygon (3 indices) into two batches
        // 3. Re-map indices to the new batches

        let mut finished_batches = Vec::new();
        let mut batch = Batch::default();

        for (index, entity) in active_entities {
            let (_, tex_coords, color, indices) = self.get(index).expect("Entity not found");
            let vertices = self.transformed_vertices(index, transforms);

            // Start a new batch if the current one is full
            if (batch.vertices.len() / 2) + vertices.len() > 8192 {
                finished_batches.push(batch);
                batch = Batch::default();
            }

            // Replicate color for each vertex
            for _ in 0..vertices.len() {
                batch.colors.push(*color);
            }
            let current_vertex_offset = (batch.vertices.len() / 2) as u16;
            for vertex in vertices {
                batch.vertices.push(vertex.x as f32);
                batch.vertices.push(vertex.y as f32);
            }
            for tex_coord in tex_coords {
                batch.tex_coords.push(tex_coord.x as f32);
                batch.tex_coords.push(tex_coord.y as f32);
            }
            for index in indices {
                batch.indices.push(*index as u16 + current_vertex_offset);
            }
        }

        // Push the last batch
        finished_batches.push(batch);

        // Update the batches
        self.batches = finished_batches;

        println!("Batched in {:?}", start.elapsed());
    }
}

// Entity management
impl RenderingResources {
    pub fn index_added(&mut self, index: Index) {
        let (index, generation) = index.into_raw_parts();
        if index >= self.vertices.len() {
            self.vertices.insert(index, SmallVec::new());
            self.texCoords.insert(index, SmallVec::new());
            self.colors.insert(index, 0);
            self.indices.insert(index, SmallVec::new());
        } else {
            self.vertices[index].clear();
            self.texCoords[index].clear();
            self.colors[index] = 0;
            self.indices[index].clear();
        }
    }
    pub fn index_removed(&mut self, index: Index) {
        // Might not be needed at all
    }
}

// Batch
#[derive(Default, Debug, Clone)]
pub struct Batch {
    pub vertices: Vec<f32>,
    pub tex_coords: Vec<f32>,
    pub colors: Vec<i32>,
    pub indices: Vec<u16>,
}
