pub mod broadphase_stack;
pub mod entity_manager;
pub mod transform_manager;
pub mod vertex_manager;

use std::cell::RefCell;

use self::{
    broadphase_stack::BroadphaseStack, entity_manager::EntityManager,
    transform_manager::TransformManager, vertex_manager::VertexManager,
};

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State::new());
}

pub struct State {
    pub entities: RefCell<EntityManager>,
    pub broadphase: RefCell<BroadphaseStack>,
    pub transforms: RefCell<TransformManager>,
    pub vertices: RefCell<VertexManager>,
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
        let transforms = RefCell::new(TransformManager::new());
        let vertices = RefCell::new(VertexManager::new());

        Self {
            entities,
            broadphase,
            transforms,
            vertices,
        }
    }

    pub fn update(&mut self, dt: f64) {
        let mut entities = self.entities.borrow_mut();
        let mut broadphase = self.broadphase.borrow_mut();
        let mut transforms = self.transforms.borrow_mut();
        let mut vertices = self.vertices.borrow_mut();

        broadphase.do_maintenance(&entities, &transforms);
    }
}
