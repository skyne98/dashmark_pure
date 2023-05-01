use flutter_rust_bridge::{SyncReturn, ZeroCopyBuffer};
pub use generational_arena::Arena;
use rapier2d_f64::na::{Point2, Vector2};
use std::time::Instant;
pub use std::{
    ops::Deref,
    sync::{Mutex, RwLock},
};

use crate::{
    api::shape::Shape,
    entity::EntityShape,
    index::GenerationalIndex,
    matrix::bulk_transform_vectors_mut,
    state::State,
    transform::Origin,
    typed_data::{
        bytes_to, bytes_to_indices, bytes_to_vector2s, indices_to_bytes, to_bytes, value_to_bytes,
    },
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
    indices: ZeroCopyBuffer<Vec<u8>>,
    positions: ZeroCopyBuffer<Vec<u8>>,
    origins: ZeroCopyBuffer<Vec<u8>>,
    rotations: ZeroCopyBuffer<Vec<u8>>,
    scales: ZeroCopyBuffer<Vec<u8>>,
) -> SyncReturn<()> {
    State::acquire_mut(|state| {
        let start = Instant::now();
        let indices = bytes_to_indices(indices.0.as_slice());
        let positions = bytes_to_vector2s(positions.0.as_slice());
        let origins = bytes_to_vector2s(origins.0.as_slice());
        let rotations = bytes_to(rotations.0.as_slice());
        let scales = bytes_to_vector2s(scales.0.as_slice());
        for ((((index, position), origin), rotation), scale) in indices
            .iter()
            .zip(positions.iter())
            .zip(origins.iter())
            .zip(rotations.iter())
            .zip(scales.iter())
        {
            if let Some(mut transform) = state.transforms.borrow_mut().transform_mut(*index) {
                transform.set_all(*position, *origin, *rotation, *scale);
                state.broadphase.borrow_mut().index_updated(*index);
            }
        }
        println!("set_transform_raw: {:?}", start.elapsed());
    });
    SyncReturn(())
}

pub fn entities_set_position_raw(
    indices: ZeroCopyBuffer<Vec<u8>>,
    positions: ZeroCopyBuffer<Vec<u8>>,
) -> SyncReturn<()> {
    State::acquire_mut(|state| {
        let indices = bytes_to_indices(indices.0.as_slice());
        let positions = bytes_to_vector2s(positions.0.as_slice());
        for (index, position) in indices.iter().zip(positions.iter()) {
            if let Some(mut transform) = state.transforms.borrow_mut().transform_mut(*index) {
                transform.set_position(*position);
                state.broadphase.borrow_mut().index_updated(*index);
            }
        }
    });
    SyncReturn(())
}

pub fn entities_set_origin_raw(
    indices: ZeroCopyBuffer<Vec<u8>>,
    origins: ZeroCopyBuffer<Vec<u8>>,
) -> SyncReturn<()> {
    State::acquire_mut(|state| {
        let indices = bytes_to_indices(indices.0.as_slice());
        let origins = bytes_to_vector2s(origins.0.as_slice());
        for (index, origin) in indices.iter().zip(origins.iter()) {
            if let Some(mut transform) = state.transforms.borrow_mut().transform_mut(*index) {
                transform.set_origin_absolute(*origin);
                state.broadphase.borrow_mut().index_updated(*index);
            }
        }
    });
    SyncReturn(())
}

pub fn entities_set_rotation_raw(
    indices: ZeroCopyBuffer<Vec<u8>>,
    rotations: ZeroCopyBuffer<Vec<u8>>,
) -> SyncReturn<()> {
    State::acquire_mut(|state| {
        let indices = bytes_to_indices(indices.0.as_slice());
        let rotations = bytes_to(rotations.0.as_slice());
        for (index, rotation) in indices.iter().zip(rotations.iter()) {
            if let Some(mut transform) = state.transforms.borrow_mut().transform_mut(*index) {
                transform.set_rotation(*rotation);
                state.broadphase.borrow_mut().index_updated(*index);
            }
        }
    });
    SyncReturn(())
}

pub fn entities_set_scale_raw(
    indices: ZeroCopyBuffer<Vec<u8>>,
    scales: ZeroCopyBuffer<Vec<u8>>,
) -> SyncReturn<()> {
    State::acquire_mut(|state| {
        let indices = bytes_to_indices(indices.0.as_slice());
        let scales = bytes_to_vector2s(scales.0.as_slice());
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
pub fn query_aabb(x: f64, y: f64, width: f64, height: f64) -> SyncReturn<Vec<GenerationalIndex>> {
    let result = State::acquire(|state| {
        let aabb = rapier2d_f64::parry::bounding_volume::Aabb::new(
            Point2::new(x, y),
            Point2::new(x + width, y + height),
        );
        state.broadphase.borrow().query_aabb(&aabb)
    });
    let result = result.into_iter().map(|index| index.into()).collect();
    SyncReturn(result)
}

pub fn query_aabb_raw(
    x: f64,
    y: f64,
    width: f64,
    height: f64,
) -> SyncReturn<ZeroCopyBuffer<Vec<u8>>> {
    let result = State::acquire(|state| {
        let aabb = rapier2d_f64::parry::bounding_volume::Aabb::new(
            Point2::new(x, y),
            Point2::new(x + width, y + height),
        );
        state.broadphase.borrow().query_aabb(&aabb)
    });
    let result_bytes = indices_to_bytes(&result[..]);
    SyncReturn(ZeroCopyBuffer(result_bytes))
}

// Rendering
pub fn entity_set_vertices_raw(
    index: GenerationalIndex,
    vertices: ZeroCopyBuffer<Vec<u8>>,
) -> SyncReturn<()> {
    State::acquire_mut(|state| {
        let vertices = bytes_to_vector2s(vertices.0.as_slice());
        state
            .rendering
            .borrow_mut()
            .set_vertices(index.into(), vertices);
    });
    SyncReturn(())
}

pub fn entity_set_tex_coords_raw(
    index: GenerationalIndex,
    tex_coords: ZeroCopyBuffer<Vec<u8>>,
) -> SyncReturn<()> {
    State::acquire_mut(|state| {
        let tex_coords = bytes_to_vector2s(tex_coords.0.as_slice());
        state
            .rendering
            .borrow_mut()
            .set_tex_coords(index.into(), tex_coords);
    });
    SyncReturn(())
}

pub fn entity_set_indices_raw(
    index: GenerationalIndex,
    indices: ZeroCopyBuffer<Vec<u8>>,
) -> SyncReturn<()> {
    State::acquire_mut(|state| {
        let indices = bytes_to(indices.0.as_slice());
        state
            .rendering
            .borrow_mut()
            .set_indices(index.into(), indices);
    });
    SyncReturn(())
}

pub fn entities_set_priority_raw(
    indices: ZeroCopyBuffer<Vec<u8>>,
    priorities: ZeroCopyBuffer<Vec<u8>>,
) -> SyncReturn<()> {
    State::acquire_mut(|state| {
        let indices = bytes_to_indices(indices.0.as_slice());
        let priorities = bytes_to(priorities.0.as_slice());
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

pub fn vertices(batch_index: u16) -> SyncReturn<ZeroCopyBuffer<Vec<u8>>> {
    State::acquire(|state| {
        let rendering = state.rendering.borrow();
        let batch = &rendering.batches[batch_index as usize];
        let vertices = &batch.vertices;
        let bytes = to_bytes(vertices).to_vec();
        SyncReturn(ZeroCopyBuffer(bytes))
    })
}

pub fn tex_coords(batch_index: u16) -> SyncReturn<ZeroCopyBuffer<Vec<u8>>> {
    State::acquire(|state| {
        let rendering = state.rendering.borrow();
        let batch = &rendering.batches[batch_index as usize];
        let tex_coords = &batch.tex_coords;
        let bytes = to_bytes(tex_coords).to_vec();
        SyncReturn(ZeroCopyBuffer(bytes))
    })
}

pub fn indices(batch_index: u16) -> SyncReturn<ZeroCopyBuffer<Vec<u8>>> {
    State::acquire(|state| {
        let rendering = state.rendering.borrow();
        let batch = &rendering.batches[batch_index as usize];
        let indices = &batch.indices;
        let bytes = to_bytes(indices).to_vec();
        SyncReturn(ZeroCopyBuffer(bytes))
    })
}

pub fn colors(batch_index: u16) -> SyncReturn<ZeroCopyBuffer<Vec<u8>>> {
    State::acquire(|state| {
        let rendering = state.rendering.borrow();
        let batch = &rendering.batches[batch_index as usize];
        let colors = &batch.colors;
        let bytes = to_bytes(colors).to_vec();
        SyncReturn(ZeroCopyBuffer(bytes))
    })
}
