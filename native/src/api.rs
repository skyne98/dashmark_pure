use flutter_rust_bridge::SyncReturn;
pub use generational_arena::Arena;
use std::cell::RefCell;
pub use std::{
    ops::Deref,
    sync::{Mutex, RwLock},
};

use crate::{entity::Entity, index::RawIndex, state::State};

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
