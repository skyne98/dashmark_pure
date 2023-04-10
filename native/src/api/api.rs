use flutter_rust_bridge::{SyncReturn, ZeroCopyBuffer};
pub use generational_arena::Arena;
use rapier2d_f64::na::{Point2, Vector2};
pub use std::{
    ops::Deref,
    sync::{Mutex, RwLock},
};

use crate::{
    api::shape::Shape,
    entity::EntityShape,
    index::RawIndex,
    matrix::bulk_transform_vectors_mut,
    state::State,
    transform::Origin,
    typed_data::{bytes_to_indices, bytes_to_vector2s, indices_to_bytes, to_bytes, value_to_bytes},
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
pub fn create_entity() -> SyncReturn<RawIndex> {
    let index = State::acquire_mut(|state| {
        let index = state.entities.borrow_mut().create_entity();
        state.broadphase.borrow_mut().index_added(index);
        state.vertices.borrow_mut().index_added(index);
        index
    });
    SyncReturn(index.into())
}

pub fn drop_entity(index: RawIndex) -> SyncReturn<()> {
    State::acquire_mut(|state| {
        let entity = state.entities.borrow_mut().remove_entity(index.into());
        if let Some(entity) = entity {
            state.broadphase.borrow_mut().index_removed(entity.index);
            state.vertices.borrow_mut().index_removed(entity.index);
        }
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
            if let Some(mut transform) = state.transforms.borrow_mut().get_transform_mut(*index) {
                transform.set_position(*position);
                state.broadphase.borrow_mut().index_updated(*index);
            }
        }
    });
    SyncReturn(())
}

pub fn entity_set_origin(index: RawIndex, relative: bool, x: f64, y: f64) -> SyncReturn<()> {
    State::acquire_mut(|state| {
        if let Some(mut trasform) = state
            .transforms
            .borrow_mut()
            .get_transform_mut(index.into())
        {
            if let Some(entity) = state.entities.borrow_mut().get_entity_mut(index.into()) {
                if relative {
                    trasform.set_origin_relative(Vector2::new(x, y));
                } else {
                    let origin = Origin::Absolute(Vector2::new(x, y));
                    let size = entity.get_size();
                    trasform.set_origin_relative(origin.to_relative(size));
                }
                state.broadphase.borrow_mut().index_updated(entity.index);
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
        let rotations = bytes_to_vector2s(rotations.0.as_slice());
        for (index, rotation) in indices.iter().zip(rotations.iter()) {
            if let Some(mut transform) = state.transforms.borrow_mut().get_transform_mut(*index) {
                transform.set_rotation(rotation.x);
                state.broadphase.borrow_mut().index_updated(*index);
            }
        }
    });
    SyncReturn(())
}

pub fn entity_set_shape(index: RawIndex, shape: Shape) -> SyncReturn<()> {
    State::acquire_mut(|state| {
        if let Some(mut entity) = state.entities.borrow_mut().get_entity_mut(index.into()) {
            let shape: Box<dyn EntityShape> = shape.into();
            entity.set_shape_box(shape);
            state.broadphase.borrow_mut().index_updated(entity.index);
        }
    });
    SyncReturn(())
}

pub fn entity_set_vertices_raw(
    index: RawIndex,
    vertices: ZeroCopyBuffer<Vec<u8>>,
) -> SyncReturn<()> {
    State::acquire_mut(|state| {
        let vertices = bytes_to_vector2s(vertices.0.as_slice());
        state.vertices.borrow_mut().set(index.into(), vertices);
    });
    SyncReturn(())
}

// Collisions
pub fn query_aabb(x: f64, y: f64, width: f64, height: f64) -> SyncReturn<Vec<RawIndex>> {
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
pub fn transformed_vertices() -> SyncReturn<ZeroCopyBuffer<Vec<u8>>> {
    // Buffer layout:
    // 1. Entities count
    // N. Vertices count
    // N + V. Vertices
    State::acquire(|state| {
        let mut buffer = Vec::new();

        let entities = state.entities.borrow();
        let vertices = state.vertices.borrow();
        let transform = state.transforms.borrow();
        let entities_len = entities.len();
        let entities_iter = entities.iter();
        buffer.extend_from_slice(value_to_bytes(&entities_len));

        for (index, _) in entities_iter {
            let transform = transform.get_transform(index).unwrap();
            let vertices = vertices.get(index).expect("Entity has no vertices");
            let vertices_len = vertices.len();
            buffer.extend_from_slice(value_to_bytes(&vertices_len));
            let mut transformed_vertices = Vec::with_capacity(vertices_len);
            transformed_vertices.copy_from_slice(vertices);
            bulk_transform_vectors_mut(&transform.matrix(), &mut transformed_vertices);

            buffer.extend_from_slice(to_bytes(&transformed_vertices));
        }

        SyncReturn(ZeroCopyBuffer(buffer))
    })
}
