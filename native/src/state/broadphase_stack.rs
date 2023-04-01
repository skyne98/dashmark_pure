use std::sync::mpsc::Receiver;

#[cfg(target_arch = "wasm32")]
use flutter_rust_bridge::JsValue;
use generational_arena::Index;
use rapier2d_f64::{parry::partitioning::QbvhUpdateWorkspace, prelude::Aabb};

use crate::{bvh::Bvh, index::IndexWrapper};

use super::entity_manager::EntityManager;

pub struct BroadphaseStack {
    current_bvh: Bvh,
    buffer_bvh: Option<Bvh>,
    buffer_receiver: Option<Receiver<Bvh>>,
    needs_rebuild: bool,
}

// Maintenance & Utils
impl BroadphaseStack {
    pub fn new() -> Self {
        Self {
            current_bvh: Bvh::default(),
            buffer_bvh: Some(Bvh::default()),
            buffer_receiver: None,
            needs_rebuild: false,
        }
    }

    pub fn is_building(&self) -> bool {
        self.buffer_bvh.is_none() && self.buffer_receiver.is_some()
    }

    pub fn entity_added(&mut self, index: Index) {
        self.current_bvh
            .bvh
            .pre_update_or_insert(IndexWrapper::from(index));
        self.needs_rebuild = true;
    }

    pub fn entity_removed(&mut self, index: Index) {
        self.current_bvh.bvh.remove(IndexWrapper::from(index));
        self.needs_rebuild = true;
    }

    pub fn entity_updated(&mut self, index: Index) {
        self.current_bvh
            .bvh
            .pre_update_or_insert(IndexWrapper::from(index));
        self.needs_rebuild = true;
    }

    pub fn do_maintenance(&mut self, entities: &EntityManager) {
        // Dispatching the rebuild task
        if self.needs_rebuild && self.is_building() == false {
            self.needs_rebuild = false;
            let to_rebuild = self
                .buffer_bvh
                .take()
                .expect("BVH is already being rebuilt, this should not happen");
            let new_entities = entities.index_iter();
            let new_entities_with_aabbs = new_entities
                .iter()
                .map(|index| {
                    let mut entity = entities.get_entity_mut(*index).expect("Entity not found");
                    let aabb = entity.get_aabb_iso().expect("Entity has no AABB");
                    (IndexWrapper::from(*index), aabb)
                })
                .collect::<Vec<_>>();

            // Spawn the task
            let (tx, rx) = std::sync::mpsc::channel();
            self.buffer_receiver = Some(rx);
            flutter_rust_bridge::spawn!(|| {
                let mut bvh = to_rebuild;
                let new_entities = new_entities_with_aabbs;
                bvh.bvh.clear_and_rebuild(new_entities.into_iter(), 0.0);
                tx.send(bvh)
                    .expect("Failed to send built BVH via the channel");
            });
        }

        // Listening if the built BVH is ready, otherwise
        // we check if we need to refit the current BVH
        if let Some(rx) = &self.buffer_receiver {
            if let Ok(bvh) = rx.try_recv() {
                self.buffer_receiver = None;
                let old_current = std::mem::replace(&mut self.current_bvh, bvh);
                self.buffer_bvh = Some(old_current);
            }
        }
    }
}

// Collision checking
impl BroadphaseStack {
    pub fn query_aabb(&self, aabb: &Aabb) -> Vec<Index> {
        let mut result = Vec::new();
        self.current_bvh.bvh.intersect_aabb(aabb, &mut result);
        result.into_iter().map(|index| index.0).collect()
    }
}
