pub mod broadphase_stack;
pub mod entity_manager;

use std::{
    cell::{Ref, RefCell, RefMut},
    rc::Rc,
};

use self::{broadphase_stack::BroadphaseStack, entity_manager::EntityManager};

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State::new());
}

pub struct State {
    pub entities: RefCell<EntityManager>,
    pub broadphase: RefCell<BroadphaseStack>,
}

// Static methods
impl State {
    pub fn acquire<F, T>(f: F) -> T
    where
        F: FnOnce(&State) -> T,
    {
        STATE.with(|state| f(&*state.borrow()))
    }

    pub fn acquire_mut<F, T>(f: F) -> T
    where
        F: FnOnce(&mut State) -> T,
    {
        STATE.with(|state| f(&mut *state.borrow_mut()))
    }
}

impl State {
    pub fn new() -> Self {
        let entities = RefCell::new(EntityManager::new());
        let broadphase = RefCell::new(BroadphaseStack::new());

        Self {
            entities,
            broadphase,
        }
    }

    pub fn update(&mut self, dt: f64) {
        let mut entities = self.entities.borrow_mut();
        let mut broadphase = self.broadphase.borrow_mut();

        broadphase.do_maintenance(&entities);
    }
}
