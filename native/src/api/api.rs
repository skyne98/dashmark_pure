use crate::{
    time::Instant,
    typed_data::{f32s_to_vec2_arrays, f32s_to_vec2s, indices_to_u32s, u32s_to_indices},
};
use flutter_rust_bridge::{SyncReturn, ZeroCopyBuffer};
pub use generational_arena::Arena;
use rapier2d::na::Point2;
pub use std::{
    ops::Deref,
    sync::{Mutex, RwLock},
};

use crate::{
    api::shape::Shape, entity::EntityShape, index::GenerationalIndex, state::State,
    typed_data::from_to,
};

// For initialization
pub fn say_hello() -> String {
    "Hello, world!".to_string()
}

// Main loop
pub fn update(dt: f64) -> SyncReturn<()> {
    State::acquire_mut(|state| state.update(dt));
    SyncReturn(())
}

// Entities
pub fn create_entity() -> SyncReturn<GenerationalIndex> {
    let index = State::acquire_mut(|state| {
        let index = state.entities.borrow_mut().create_entity();
        state.broadphase.borrow_mut().index_added(index);
        state.rendering.borrow_mut().index_added(index);
        state.transforms.borrow_mut().index_added(index);
        index
    });
    SyncReturn(index.into())
}

pub fn drop_entity(index: GenerationalIndex) -> SyncReturn<()> {
    State::acquire_mut(|state| {
        let entity = state.entities.borrow_mut().remove_entity(index.into());
        if let Some(entity) = entity {
            state.broadphase.borrow_mut().index_removed(entity.index);
            state.rendering.borrow_mut().index_removed(entity.index);
            state.transforms.borrow_mut().index_removed(entity.index);
        }
    });
    SyncReturn(())
}

// Transform
pub fn entities_set_transform_raw(
    indices: Vec<u32>,
    positions: Vec<f32>,
    origins: Vec<f32>,
    rotations: Vec<f32>,
    scales: Vec<f32>,
) -> SyncReturn<()> {
    let start = Instant::now();
    State::acquire_mut(|state| {
        let indices = u32s_to_indices(indices.as_slice());
        let positions = f32s_to_vec2_arrays(positions.as_slice());
        let origins = f32s_to_vec2_arrays(origins.as_slice());
        let rotations = rotations.as_slice();
        let scales = f32s_to_vec2_arrays(scales.as_slice());

        let transforms = state.transforms.borrow_mut();
        let mut broadphase = state.broadphase.borrow_mut();
        for (i, index) in indices.iter().enumerate() {
            if let Some(mut transform) = transforms.transform_mut(*index) {
                transform.set_all(positions[i], origins[i], rotations[i], scales[i]);
                broadphase.index_updated(*index);
            }
        }
    });
    println!("set_transform_raw: {:?}", start.elapsed());
    SyncReturn(())
}

pub fn entities_set_position_raw(indices: Vec<u32>, positions: Vec<f32>) -> SyncReturn<()> {
    State::acquire_mut(|state| {
        let indices = u32s_to_indices(indices.as_slice());
        let positions = f32s_to_vec2_arrays(positions.as_slice());
        for (index, position) in indices.iter().zip(positions.iter()) {
            if let Some(mut transform) = state.transforms.borrow_mut().transform_mut(*index) {
                transform.set_position(*position);
                state.broadphase.borrow_mut().index_updated(*index);
            }
        }
    });
    SyncReturn(())
}

pub fn entities_set_origin_raw(indices: Vec<u32>, origins: Vec<f32>) -> SyncReturn<()> {
    State::acquire_mut(|state| {
        let indices = u32s_to_indices(indices.as_slice());
        let origins = f32s_to_vec2_arrays(origins.as_slice());
        for (index, origin) in indices.iter().zip(origins.iter()) {
            if let Some(mut transform) = state.transforms.borrow_mut().transform_mut(*index) {
                transform.set_origin_absolute(*origin);
                state.broadphase.borrow_mut().index_updated(*index);
            }
        }
    });
    SyncReturn(())
}

pub fn entities_set_rotation_raw(indices: Vec<u32>, rotations: Vec<f32>) -> SyncReturn<()> {
    State::acquire_mut(|state| {
        let indices = u32s_to_indices(indices.as_slice());
        for (index, rotation) in indices.iter().zip(rotations.iter()) {
            if let Some(mut transform) = state.transforms.borrow_mut().transform_mut(*index) {
                transform.set_rotation(*rotation);
                state.broadphase.borrow_mut().index_updated(*index);
            }
        }
    });
    SyncReturn(())
}

pub fn entities_set_scale_raw(indices: Vec<u32>, scales: Vec<f32>) -> SyncReturn<()> {
    State::acquire_mut(|state| {
        let indices = u32s_to_indices(indices.as_slice());
        let scales = f32s_to_vec2_arrays(scales.as_slice());
        for (index, scale) in indices.iter().zip(scales.iter()) {
            if let Some(mut transform) = state.transforms.borrow_mut().transform_mut(*index) {
                transform.set_scale(*scale);
                state.broadphase.borrow_mut().index_updated(*index);
            }
        }
    });
    SyncReturn(())
}

// Collisions
pub fn query_aabb(x: f32, y: f32, width: f32, height: f32) -> SyncReturn<Vec<GenerationalIndex>> {
    let result = State::acquire(|state| {
        let aabb = rapier2d::parry::bounding_volume::Aabb::new(
            Point2::new(x, y),
            Point2::new(x + width, y + height),
        );
        state.broadphase.borrow().query_aabb(&aabb)
    });
    let result = result.into_iter().map(|index| index.into()).collect();
    SyncReturn(result)
}

pub fn query_aabb_raw(x: f32, y: f32, width: f32, height: f32) -> SyncReturn<Vec<u32>> {
    let result = State::acquire(|state| {
        let aabb = rapier2d::parry::bounding_volume::Aabb::new(
            Point2::new(x, y),
            Point2::new(x + width, y + height),
        );
        state.broadphase.borrow().query_aabb(&aabb)
    });
    let result_bytes = indices_to_u32s(&result[..]);
    SyncReturn(result_bytes)
}

// Rendering
pub fn entity_set_vertices_raw(index: GenerationalIndex, vertices: Vec<f32>) -> SyncReturn<()> {
    State::acquire_mut(|state: &mut State| {
        let vertices = f32s_to_vec2s(vertices.as_slice());
        state
            .rendering
            .borrow_mut()
            .set_vertices(index.into(), vertices);
    });
    SyncReturn(())
}

pub fn entity_set_tex_coords_raw(index: GenerationalIndex, tex_coords: Vec<f32>) -> SyncReturn<()> {
    State::acquire_mut(|state| {
        let tex_coords = f32s_to_vec2s(tex_coords.as_slice());
        state
            .rendering
            .borrow_mut()
            .set_tex_coords(index.into(), tex_coords);
    });
    SyncReturn(())
}

pub fn entity_set_indices_raw(index: GenerationalIndex, indices: Vec<u16>) -> SyncReturn<()> {
    State::acquire_mut(|state| {
        let indices = from_to(indices.as_slice());
        state
            .rendering
            .borrow_mut()
            .set_indices(index.into(), indices);
    });
    SyncReturn(())
}

pub fn entities_set_priority_raw(indices: Vec<u32>, priorities: Vec<i32>) -> SyncReturn<()> {
    State::acquire_mut(|state| {
        let indices = u32s_to_indices(indices.as_slice());
        for (index, priority) in indices.iter().zip(priorities.iter()) {
            if let Some(mut entity) = state.entities.borrow_mut().get_entity_mut(*index) {
                entity.priority = *priority;
            }
        }
    });
    SyncReturn(())
}

pub fn entity_set_shape(index: GenerationalIndex, shape: Shape) -> SyncReturn<()> {
    State::acquire_mut(|state| {
        if let Some(mut entity) = state.entities.borrow_mut().get_entity_mut(index.into()) {
            let shape: Box<dyn EntityShape> = shape.into();
            entity.set_shape_box(shape);
            state.broadphase.borrow_mut().index_updated(entity.index);
        }
    });
    SyncReturn(())
}

pub fn entity_set_color(index: GenerationalIndex, color: i32) -> SyncReturn<()> {
    State::acquire_mut(|state| {
        if let Some((_, _, mut entity_color, _)) =
            state.rendering.borrow_mut().get_mut(index.into())
        {
            *entity_color = color;
        }
    });
    SyncReturn(())
}

pub fn batches_count() -> SyncReturn<u64> {
    State::acquire(|state| {
        let rendering = state.rendering.borrow();
        SyncReturn(rendering.batches.len() as u64)
    })
}

pub fn vertices(batch_index: u16) -> SyncReturn<Vec<f32>> {
    State::acquire(|state| {
        let rendering = state.rendering.borrow();
        let batch = &rendering.batches[batch_index as usize];
        let vertices = &batch.vertices;
        SyncReturn(vertices.clone())
    })
}

pub fn tex_coords(batch_index: u16) -> SyncReturn<Vec<f32>> {
    State::acquire(|state| {
        let rendering = state.rendering.borrow();
        let batch = &rendering.batches[batch_index as usize];
        let tex_coords = &batch.tex_coords;
        SyncReturn(tex_coords.clone())
    })
}

pub fn indices(batch_index: u16) -> SyncReturn<Vec<u16>> {
    State::acquire(|state| {
        let rendering = state.rendering.borrow();
        let batch = &rendering.batches[batch_index as usize];
        let indices = &batch.indices;
        SyncReturn(indices.clone())
    })
}

pub fn colors(batch_index: u16) -> SyncReturn<Vec<i32>> {
    State::acquire(|state| {
        let rendering = state.rendering.borrow();
        let batch = &rendering.batches[batch_index as usize];
        let colors = &batch.colors;
        SyncReturn(colors.clone())
    })
}
