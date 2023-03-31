use flutter_rust_bridge::{SyncReturn, ZeroCopyBuffer};
pub use generational_arena::Arena;
use rapier2d_f64::na::{Point2, Vector2};
pub use std::{
    ops::Deref,
    sync::{Mutex, RwLock},
};

use crate::{
    api::shape::Shape,
    entity::{EntityShape, Origin},
    index::RawIndex,
    state::State,
    typed_data::{bytes_to_indices, bytes_to_vector2s, indices_to_bytes},
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
        state.broadphase.borrow_mut().entity_added(index);
        index
    });
    SyncReturn(index.into())
}

pub fn drop_entity(index: RawIndex) -> SyncReturn<()> {
    State::acquire_mut(|state| {
        let entity = state.entities.borrow_mut().remove_entity(index.into());
        if let Some(entity) = entity {
            state.broadphase.borrow_mut().entity_removed(entity.index);
        }
    });
    SyncReturn(())
}

pub fn entity_set_position(index: RawIndex, x: f64, y: f64) -> SyncReturn<()> {
    State::acquire_mut(|state| {
        if let Some(mut entity) = state.entities.borrow_mut().get_entity_mut(index.into()) {
            entity.set_position(Vector2::new(x, y));
            state.broadphase.borrow_mut().entity_updated(entity.index);
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
            if let Some(mut entity) = state.entities.borrow_mut().get_entity_mut(*index) {
                entity.set_position(Vector2::new(position[0], position[1]));
                state.broadphase.borrow_mut().entity_updated(entity.index);
            }
        }
    });
    SyncReturn(())
}

pub fn entity_set_origin(index: RawIndex, relative: bool, x: f64, y: f64) -> SyncReturn<()> {
    State::acquire_mut(|state| {
        if let Some(mut entity) = state.entities.borrow_mut().get_entity_mut(index.into()) {
            if relative {
                entity.set_origin(Origin::Relative(Vector2::new(x, y)));
            } else {
                entity.set_origin(Origin::Absolute(Vector2::new(x, y)));
            }
            state.broadphase.borrow_mut().entity_updated(entity.index);
        }
    });
    SyncReturn(())
}

pub fn entity_set_rotation(index: RawIndex, rotation: f64) -> SyncReturn<()> {
    State::acquire_mut(|state| {
        if let Some(mut entity) = state.entities.borrow_mut().get_entity_mut(index.into()) {
            entity.set_rotation(rotation);
            state.broadphase.borrow_mut().entity_updated(entity.index);
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
            if let Some(mut entity) = state.entities.borrow_mut().get_entity_mut(*index) {
                entity.set_rotation(rotation[0]);
                state.broadphase.borrow_mut().entity_updated(entity.index);
            }
        }
    });
    SyncReturn(())
}

pub fn entity_set_shape(index: RawIndex, shape: Shape) -> SyncReturn<()> {
    State::acquire_mut(|state| {
        if let Some(mut entity) = state.entities.borrow_mut().get_entity_mut(index.into()) {
            let shape: Box<dyn EntityShape> = shape.into();
            entity.set_shape_from_box(shape);
            state.broadphase.borrow_mut().entity_updated(entity.index);
        }
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
