use flutter_rust_bridge::SyncReturn;
pub use generational_arena::Arena;
use rapier2d_f64::na::Vector2;
use std::cell::RefCell;
pub use std::{
    ops::Deref,
    sync::{Mutex, RwLock},
};

use crate::{
    api::state::State,
    entity::{Entity, EntityShape, Origin},
    index::RawIndex,
};

use crate::api::shape::Shape;

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State::new());
}

// For initialization
pub fn say_hello() -> String {
    "Hello, world!".to_string()
}

fn state<F, T>(f: F) -> T
where
    F: FnOnce(&State) -> T,
{
    STATE.with(|state| f(&*state.borrow()))
}

fn state_mut<F, T>(f: F) -> T
where
    F: FnOnce(&mut State) -> T,
{
    STATE.with(|state| f(&mut *state.borrow_mut()))
}

// Entities
pub fn create_entity() -> SyncReturn<RawIndex> {
    SyncReturn(state_mut(|state| state.add_entity(Entity::default())).into())
}

pub fn drop_entity(index: RawIndex) -> SyncReturn<()> {
    state_mut(|state| {
        state.remove_entity(index.into());
    });
    SyncReturn(())
}

pub fn entity_set_position(index: RawIndex, x: f64, y: f64) -> SyncReturn<()> {
    state_mut(|state| {
        if let Some(entity) = state.get_entity_mut(index.into()) {
            entity.set_position(Vector2::new(x, y));
        }
    });
    SyncReturn(())
}

pub fn entity_set_origin(index: RawIndex, relative: bool, x: f64, y: f64) -> SyncReturn<()> {
    state_mut(|state| {
        if let Some(entity) = state.get_entity_mut(index.into()) {
            if relative {
                entity.set_origin(Origin::Relative(Vector2::new(x, y)));
            } else {
                entity.set_origin(Origin::Absolute(Vector2::new(x, y)));
            }
        }
    });
    SyncReturn(())
}

pub fn entity_set_rotation(index: RawIndex, rotation: f64) -> SyncReturn<()> {
    state_mut(|state| {
        if let Some(entity) = state.get_entity_mut(index.into()) {
            entity.set_rotation(rotation);
        }
    });
    SyncReturn(())
}

pub fn entity_set_shape(index: RawIndex, shape: Shape) -> SyncReturn<()> {
    state_mut(|state| {
        if let Some(entity) = state.get_entity_mut(index.into()) {
            let shape: Box<dyn EntityShape> = shape.into();
            entity.set_shape_from_box(shape);
        }
    });
    SyncReturn(())
}
