use std::sync::mpsc::Receiver;

#[cfg(target_arch = "wasm32")]
use flutter_rust_bridge::JsValue;
use generational_arena::Index;
use rapier2d_f64::{parry::partitioning::QbvhUpdateWorkspace, prelude::Aabb};

use crate::{bvh::Bvh, index::IndexWrapper};

use super::entity_manager::EntityManager;

pub struct BroadphaseStack {
    active_bvh_present: Bvh,
    active_bvh_to_build: Option<Bvh>,
    active_bvh_built_receiver: Option<Receiver<Bvh>>,
    active_needs_rebuild: bool,
    active_needs_refit: bool,
    active_update_workspace: QbvhUpdateWorkspace,
}

// Maintenance & Utils
impl BroadphaseStack {
    pub fn new() -> Self {
        Self {
            active_bvh_present: Bvh::default(),
            active_bvh_to_build: Some(Bvh::default()),
            active_bvh_built_receiver: None,
            active_needs_rebuild: false,
            active_needs_refit: false,
            active_update_workspace: QbvhUpdateWorkspace::default(),
        }
    }

    pub fn is_building(&self) -> bool {
        self.active_bvh_to_build.is_none() && self.active_bvh_built_receiver.is_some()
    }

    pub fn entity_added(&mut self, index: Index) {
        self.active_bvh_present
            .bvh
            .pre_update_or_insert(IndexWrapper::from(index));
        self.active_needs_rebuild = true;
    }

    pub fn entity_removed(&mut self, index: Index) {
        self.active_bvh_present
            .bvh
            .remove(IndexWrapper::from(index));
        self.active_needs_rebuild = true;
    }

    pub fn entity_updated(&mut self, index: Index) {
        self.active_bvh_present
            .bvh
            .pre_update_or_insert(IndexWrapper::from(index));
        self.active_needs_rebuild = true;
        self.active_needs_refit = true;
    }

    pub fn do_maintenance(&mut self, entities: &EntityManager) {
        // Dispatching the rebuild task
        if self.active_needs_rebuild && self.is_building() == false {
            self.active_needs_rebuild = false;
            let to_rebuild = self
                .active_bvh_to_build
                .take()
                .expect("BVH is already being rebuilt, this should not happen");
            let new_entities = entities.index_iter();
            let new_entities_with_aabbs = new_entities
                .iter()
                .map(|index| {
                    let mut entity = entities.get_entity_mut(*index).expect("Entity not found");
                    let aabb = entity.get_aabb().expect("Entity has no AABB");
                    (IndexWrapper::from(*index), aabb)
                })
                .collect::<Vec<_>>();

            // Spawn the task
            let (tx, rx) = std::sync::mpsc::channel();
            self.active_bvh_built_receiver = Some(rx);
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
        if let Some(rx) = &self.active_bvh_built_receiver {
            if let Ok(bvh) = rx.try_recv() {
                self.active_bvh_built_receiver = None;
                self.active_needs_refit = false;
                let old_present = std::mem::replace(&mut self.active_bvh_present, bvh);
                self.active_bvh_to_build = Some(old_present);
            } else if self.active_needs_refit {
                // Synchronously refit the current BVH
                self.active_needs_refit = false;
                let to_refit = &mut self.active_bvh_present;
                let update_workspace = &mut self.active_update_workspace;
                to_refit.bvh.refit(0.0, update_workspace, |index| {
                    let mut entity = entities.get_entity_mut(index.0).unwrap();
                    entity.get_aabb().expect("Entity has no AABB")
                });
            }
        }
    }
}

// Collision checking
impl BroadphaseStack {
    pub fn query_aabb(&self, aabb: &Aabb) -> Vec<Index> {
        let mut result = Vec::new();
        self.active_bvh_present
            .bvh
            .intersect_aabb(aabb, &mut result);
        result.into_iter().map(|index| index.0).collect()
    }
}
