pub mod broadphase_stack;
pub mod entity_manager;
pub mod rendering_resources;
pub mod transform_manager;
pub mod verlet_system;

use std::cell::RefCell;

use self::{
    broadphase_stack::BroadphaseStack, entity_manager::EntityManager,
    rendering_resources::RenderingResources, transform_manager::TransformManager,
    verlet_system::VerletSystem,
};

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State::new());
}

pub struct State {
    pub entities: RefCell<EntityManager>,
    pub broadphase: RefCell<BroadphaseStack>,
    pub transforms: RefCell<TransformManager>,
    pub rendering: RefCell<RenderingResources>,
    pub verlet: RefCell<VerletSystem<8>>,
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
        // Set up the logging
        #[cfg(target_arch = "wasm32")]
        {
            console_log::init_with_level(log::Level::Debug).unwrap();
            console_error_panic_hook::set_once();
        }
        #[cfg(target_os = "android")]
        {
            use android_logger::{Config, FilterBuilder};
            android_logger::init_once(Config::default().with_max_level(log::LevelFilter::Debug));
        }
        #[cfg(not(any(target_arch = "wasm32", target_os = "android")))]
        {
            env_logger::init_from_env(env_logger::Env::default().default_filter_or("debug"));
        }
        log::debug!("Logger is initialized!");

        let entities = RefCell::new(EntityManager::new());
        let broadphase = RefCell::new(BroadphaseStack::new());
        let transforms = RefCell::new(TransformManager::new());
        let vertices = RefCell::new(RenderingResources::new());
        let verlet = RefCell::new(VerletSystem::new());

        Self {
            entities,
            broadphase,
            transforms,
            rendering: vertices,
            verlet,
        }
    }

    pub fn update(&mut self, dt: f64) {
        let mut entities = self.entities.borrow_mut();
        let mut broadphase = self.broadphase.borrow_mut();
        let mut transforms = self.transforms.borrow_mut();
        let mut rendering = self.rendering.borrow_mut();
        let mut verlet = self.verlet.borrow_mut();

        verlet.simulate(dt);
        verlet.apply_to_transforms(&mut transforms);
        transforms.sweep(&entities);
        broadphase.do_maintenance(&entities, &transforms);
        rendering.batchify(&mut entities, &transforms);
    }
}
