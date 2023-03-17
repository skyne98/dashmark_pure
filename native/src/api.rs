pub use generational_arena::{Arena, Index as ExternalIndex};
use std::cell::RefCell;
pub use std::{
    ops::Deref,
    sync::{Mutex, RwLock},
};

use crate::state::State;

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State::new());
}
