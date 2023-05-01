use std::{
    cell::{Ref, RefCell, RefMut},
    collections::HashSet,
};

use generational_arena::Index;

use crate::transform::Transform;

use super::entity_manager;

pub struct TransformManager {
    transforms: Vec<Transform>,
}

impl TransformManager {
    pub fn new() -> Self {
        Self {
            transforms: Vec::new(),
        }
    }

    pub fn transform(&self, index: Index) -> Option<&Transform> {
        let index = index.into_raw_parts().0;
        self.transforms.get(index)
    }

    pub fn transform_mut(&mut self, index: Index) -> Option<&mut Transform> {
        let index = index.into_raw_parts().0;
        self.transforms.get_mut(index)
    }

    pub fn sweep(&mut self, entity_manager: &entity_manager::EntityManager) {
        // Sweep the dirty matrices
        for transform in self.transforms.iter_mut() {
            if transform.dirty_matrix {
                transform.update_matrix();
            }
        }
        // Sweep the dirty isometries matrices
        for (index, transform) in self.transforms.iter_mut().enumerate() {
            if transform.dirty_isometry {
                let entity = entity_manager.get_entity_unknown_gen(index).unwrap();
                transform.update_isometry(entity.get_shape_natural_offset());
            }
        }
    }
}

impl TransformManager {
    pub fn index_added(&mut self, index: Index) {
        let index = index.into_raw_parts().0;
        self.transforms.insert(index, Transform::default());
    }

    pub fn index_removed(&mut self, index: Index) {
        // Do nothing here for now, keep as cache
    }
}
