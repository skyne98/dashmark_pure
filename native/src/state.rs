use generational_arena::Arena;

use crate::entity::Entity;

pub struct State {
    pub entities: Arena<Entity>,
}

impl State {
    pub fn new() -> Self {
        Self {
            entities: Arena::new(),
        }
    }
}
