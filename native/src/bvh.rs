use std::{cell::RefCell, collections::HashMap, rc::Rc};

use generational_arena::Index;
use rapier2d_f64::{
    parry::partitioning::{Qbvh, QbvhUpdateWorkspace},
    prelude::Aabb,
};

use crate::{
    entity::Entity,
    index::IndexWrapper,
    state::{
        entity_manager::{self, EntityManager},
        transform_manager::{self, TransformManager},
    },
};

// Bvh
pub struct Bvh {
    pub bvh: Qbvh<IndexWrapper>,
}

impl Default for Bvh {
    fn default() -> Self {
        Self { bvh: Qbvh::new() }
    }
}

impl Bvh {
    pub fn new() -> Self {
        Self { bvh: Qbvh::new() }
    }

    pub fn from_indices<'a, I>(
        indices: I,
        entity_manager: &EntityManager,
        transform_manager: &TransformManager,
    ) -> Self
    where
        I: IntoIterator<Item = &'a Index>,
    {
        let mut bvh = Self::new();
        let mut index_and_aabb = vec![];
        for index in indices {
            let entity = entity_manager.get_entity(*index).unwrap();
            bvh.bvh.pre_update_or_insert(IndexWrapper(*index));
            let transform = transform_manager.get_transform(*index).unwrap();
            let aabb = entity.get_global_aabb(&transform);
            let aabb = aabb.expect("Entity has no AABB");
            index_and_aabb.push((IndexWrapper::from(index), aabb));
        }

        // Rebuild
        bvh.bvh.clear_and_rebuild(index_and_aabb.into_iter(), 0.0);

        bvh
    }

    // Updates
    pub fn prepare_upsert(&mut self, entity: &Entity) {
        self.bvh.pre_update_or_insert(IndexWrapper(entity.index));
    }
    pub fn remove(&mut self, entity: &Entity) {
        self.bvh.remove(IndexWrapper(entity.index));
    }

    pub fn refit<'a, F>(&'a mut self, aabb_builder: F)
    where
        F: Fn(Index) -> Option<Aabb>,
    {
        let mut workspace = QbvhUpdateWorkspace::default();
        self.refit_in_workspace(&mut workspace, aabb_builder);
    }
    pub fn refit_in_workspace<'a, F>(
        &'a mut self,
        workspace: &mut QbvhUpdateWorkspace,
        aabb_builder: F,
    ) where
        F: Fn(Index) -> Option<Aabb>,
    {
        let for_removal = RefCell::new(Vec::new());
        self.bvh.refit(0.0, workspace, |index| {
            let aabb = aabb_builder(index.0);
            if let Some(aabb) = aabb {
                aabb.clone()
            } else {
                for_removal.borrow_mut().push(index.clone());
                Aabb::new_invalid()
            }
        });
        for index in for_removal.into_inner() {
            self.bvh.remove(index);
        }
    }

    pub fn rebalance(&mut self) {
        let mut workspace = QbvhUpdateWorkspace::default();
        self.rebalance_in_workspace(&mut workspace);
    }
    pub fn rebalance_in_workspace(&mut self, workspace: &mut QbvhUpdateWorkspace) {
        self.bvh.rebalance(0.0, workspace);
    }

    // Utils
    pub fn flatten(&self) -> FlatBvh {
        FlatBvh::new(&self.bvh)
    }
}

// Flattened
#[derive(Debug, Clone)]
pub struct FlatBvh {
    pub min_x: Vec<f64>,
    pub min_y: Vec<f64>,
    pub max_x: Vec<f64>,
    pub max_y: Vec<f64>,
    pub depth: Vec<u8>,
    pub is_leaf: Vec<u8>,
}

impl Default for FlatBvh {
    fn default() -> Self {
        Self {
            min_x: Vec::new(),
            min_y: Vec::new(),
            max_x: Vec::new(),
            max_y: Vec::new(),
            depth: Vec::new(),
            is_leaf: Vec::new(),
        }
    }
}

impl FlatBvh {
    pub fn new(bvh: &Qbvh<IndexWrapper>) -> FlatBvh {
        let nodes = bvh.raw_nodes();
        if nodes.is_empty() {
            return FlatBvh::default();
        }
        let nodes_len = nodes.len();

        let mut flat_bvh = FlatBvh {
            min_x: Vec::with_capacity(nodes_len),
            min_y: Vec::with_capacity(nodes_len),
            max_x: Vec::with_capacity(nodes_len),
            max_y: Vec::with_capacity(nodes_len),
            depth: Vec::with_capacity(nodes_len),
            is_leaf: Vec::with_capacity(nodes_len),
        };

        let mut stack = vec![(0u32, 0u64)];

        while let Some((inode, current_depth)) = stack.pop() {
            let node = &nodes[inode as usize];
            let simd_aabb = &node.simd_aabb;

            for ii in 0..rapier2d_f64::parry::math::SIMD_WIDTH {
                let aabb = simd_aabb.extract(ii);

                if aabb.mins.x == f64::MAX {
                    continue;
                }
                flat_bvh.min_x.push(aabb.mins.x);
                flat_bvh.min_y.push(aabb.mins.y);
                flat_bvh.max_x.push(aabb.maxs.x);
                flat_bvh.max_y.push(aabb.maxs.y);
                flat_bvh.depth.push(current_depth as u8);

                if node.is_leaf() == false {
                    // Internal node, visit the child.
                    let child_index = node.children[ii];

                    // Check if child index is valid
                    if child_index as usize <= nodes.len() {
                        stack.push((child_index, current_depth + 1));
                    }

                    flat_bvh.is_leaf.push(0);
                } else {
                    flat_bvh.is_leaf.push(1);
                }
            }
        }

        flat_bvh
    }

    pub fn to_byte_buffer(self) -> Vec<u8> {
        // Use some unsafe operations to turn each Vec into a byte buffer (without copying)
        // and then concatenate them all together.
        let minxs_len = self.min_x.len() as u64;
        let minxs_buffer = unsafe {
            let minxs = self.min_x;
            let minxs_ptr = minxs.as_ptr() as *const u8;
            let minxs_len = minxs.len() * std::mem::size_of::<f64>();
            let raw_minxs = std::slice::from_raw_parts(minxs_ptr, minxs_len).to_vec();
            // Make sure the data doesn't get dropped
            std::mem::forget(minxs);
            raw_minxs
        };

        let minys_len = self.min_y.len() as u64;
        let minys_buffer = unsafe {
            let minys = self.min_y;
            let minys_ptr = minys.as_ptr() as *const u8;
            let minys_len = minys.len() * std::mem::size_of::<f64>();
            let raw_minys = std::slice::from_raw_parts(minys_ptr, minys_len).to_vec();
            // Make sure the data doesn't get dropped
            std::mem::forget(minys);
            raw_minys
        };

        let maxxs_len = self.max_x.len() as u64;
        let maxxs_buffer = unsafe {
            let maxxs = self.max_x;
            let maxxs_ptr = maxxs.as_ptr() as *const u8;
            let maxxs_len = maxxs.len() * std::mem::size_of::<f64>();
            let raw_maxxs = std::slice::from_raw_parts(maxxs_ptr, maxxs_len).to_vec();
            // Make sure the data doesn't get dropped
            std::mem::forget(maxxs);
            raw_maxxs
        };

        let maxys_len = self.max_y.len() as u64;
        let maxys_buffer = unsafe {
            let maxys = self.max_y;
            let maxys_ptr = maxys.as_ptr() as *const u8;
            let maxys_len = maxys.len() * std::mem::size_of::<f64>();
            let raw_maxys = std::slice::from_raw_parts(maxys_ptr, maxys_len).to_vec();
            // Make sure the data doesn't get dropped
            std::mem::forget(maxys);
            raw_maxys
        };

        let depths_len = self.depth.len() as u64;
        let depths_buffer = unsafe {
            let depths = self.depth;
            let depths_ptr = depths.as_ptr() as *const u8;
            let depths_len = depths.len();
            let raw_depths = std::slice::from_raw_parts(depths_ptr, depths_len).to_vec();
            // Make sure the data doesn't get dropped
            std::mem::forget(depths);
            raw_depths
        };

        let is_leafs_len = self.is_leaf.len() as u64;
        let is_leafs_buffer = unsafe {
            let is_leafs = self.is_leaf;
            let is_leafs_ptr = is_leafs.as_ptr() as *const u8;
            let is_leafs_len = is_leafs.len();
            let raw_is_leafs = std::slice::from_raw_parts(is_leafs_ptr, is_leafs_len).to_vec();
            // Make sure the data doesn't get dropped
            std::mem::forget(is_leafs);
            raw_is_leafs
        };

        let mut byte_buffer = Vec::new();
        let len_bytes = minxs_len.to_ne_bytes();
        byte_buffer.extend(len_bytes);
        byte_buffer.extend(minxs_buffer);
        let len_bytes = minys_len.to_ne_bytes();
        byte_buffer.extend(len_bytes);
        byte_buffer.extend(minys_buffer);
        let len_bytes = maxxs_len.to_ne_bytes();
        byte_buffer.extend(len_bytes);
        byte_buffer.extend(maxxs_buffer);
        let len_bytes = maxys_len.to_ne_bytes();
        byte_buffer.extend(len_bytes);
        byte_buffer.extend(maxys_buffer);
        let len_bytes = depths_len.to_ne_bytes();
        byte_buffer.extend(len_bytes);
        byte_buffer.extend(depths_buffer);
        let len_bytes = is_leafs_len.to_ne_bytes();
        byte_buffer.extend(len_bytes);
        byte_buffer.extend(is_leafs_buffer);

        byte_buffer
    }
}
