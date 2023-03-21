use flutter_rust_bridge::{SyncReturn, ZeroCopyBuffer};
pub use generational_arena::Arena;
use generational_arena::Index;
use rapier2d_f64::na::Vector2;
use std::cell::RefCell;
pub use std::{
    ops::Deref,
    sync::{Mutex, RwLock},
};

use crate::{
    api::state::State,
    bvh::{Bvh, FlatBvh},
    entity::{Entity, EntityShape, Origin},
    index::{IndexWrapper, RawIndex},
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

pub fn entities_set_position(data: ZeroCopyBuffer<Vec<u8>>) -> SyncReturn<()> {
    state_mut(|state| {
        let data = data.0.as_slice();
        // Read first 8 bytes as the length of the indices array
        let indices_len = u64::from_ne_bytes(data[0..8].try_into().unwrap()) as u64;
        // Now read the indices data (8 bytes per index, 8 bytes per generation)
        // Memory map a region of the data to read the indices
        let indices_data = unsafe {
            std::slice::from_raw_parts(data.as_ptr().add(8) as *const u64, indices_len as usize * 2)
        };
        let indices = indices_data
            .chunks_exact(2)
            .map(|chunk| Index::from_raw_parts(chunk[0] as usize, chunk[1]))
            .collect::<Vec<_>>();

        // Skip the next 8 bytes to get to the positions data
        let positions_data = unsafe {
            std::slice::from_raw_parts(
                data.as_ptr().add(8 + indices_len as usize * 16 + 8) as *const f64,
                indices_len as usize * 2,
            )
        };
        let positions = positions_data
            .chunks_exact(2)
            .map(|chunk| [chunk[0], chunk[1]])
            .collect::<Vec<_>>();

        for (index, position) in indices.iter().zip(positions.iter()) {
            if let Some(entity) = state.get_entity_mut(*index) {
                entity.set_position(Vector2::new(position[0], position[1]));
            }
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

/* BVH */
pub fn create_bvh() -> SyncReturn<RawIndex> {
    SyncReturn(state_mut(|state| state.add_bvh(Bvh::default())).into())
}

pub fn drop_bvh(index: RawIndex) -> SyncReturn<()> {
    state_mut(|state| {
        state.remove_bvh(index.into());
    });
    SyncReturn(())
}

pub fn bvh_clear_and_rebuild(
    index: RawIndex,
    entities: Vec<RawIndex>,
    dilation_factor: f64,
) -> SyncReturn<()> {
    state_mut(|state| {
        // Gather indices and AABBs of entities
        let mut indices_and_aabbs = Vec::with_capacity(entities.len());
        for entity_index in entities {
            if let Some(entity) = state.get_entity_mut(entity_index.into()) {
                indices_and_aabbs.push((
                    IndexWrapper::from(entity_index),
                    entity
                        .get_aabb()
                        .expect("Entity has no shape, it must have a shape to be added to a BVH."),
                ));
            }
        }

        // Get the BVH
        if let Some(bvh) = state.get_bvh_mut(index.into()) {
            bvh.bvh
                .clear_and_rebuild(indices_and_aabbs.into_iter(), dilation_factor);
        }
    });
    SyncReturn(())
}

pub fn bvh_clear_and_rebuild_raw(
    index: RawIndex,
    data: ZeroCopyBuffer<Vec<u8>>,
    dilation_factor: f64,
) -> SyncReturn<()> {
    state_mut(|state| {
        let data = data.0.as_slice();
        // Read first 8 bytes as the length of the indices array
        let indices_len = u64::from_ne_bytes(data[0..8].try_into().unwrap()) as u64;
        // Now read the indices data (8 bytes per index, 8 bytes per generation)
        // Memory map a region of the data to read the indices
        let indices_data = unsafe {
            std::slice::from_raw_parts(data.as_ptr().add(8) as *const u64, indices_len as usize * 2)
        };
        let indices = indices_data
            .chunks_exact(2)
            .map(|chunk| Index::from_raw_parts(chunk[0] as usize, chunk[1]))
            .collect::<Vec<_>>();

        let mut indices_and_aabbs = Vec::with_capacity(indices.len());
        for entity_index in indices {
            if let Some(entity) = state.get_entity_mut(entity_index) {
                indices_and_aabbs.push((
                    IndexWrapper(entity_index),
                    entity
                        .get_aabb()
                        .expect("Entity has no shape, it must have a shape to be added to a BVH."),
                ));
            }
        }

        // Get the BVH
        if let Some(bvh) = state.get_bvh_mut(index.into()) {
            bvh.bvh
                .clear_and_rebuild(indices_and_aabbs.into_iter(), dilation_factor);
        }
    });
    SyncReturn(())
}

pub fn bvh_flatten(index: RawIndex) -> SyncReturn<ZeroCopyBuffer<Vec<u8>>> {
    state_mut(|state| {
        let bvh = state.get_bvh(index.into()).expect("BVH not found");
        let flattened = bvh.flatten();
        let bytes = flattened.to_byte_buffer();
        SyncReturn(ZeroCopyBuffer(bytes))
    })
}
