use std::{
    cell::{Ref, RefCell, RefMut},
    collections::HashSet,
};

use generational_arena::Index;

use crate::transform::Transform;

pub struct TransformManager {
    transforms: Vec<RefCell<Transform>>,
    dirty: HashSet<usize>,
}

impl TransformManager {
    pub fn new() -> Self {
        Self {
            transforms: Vec::new(),
            dirty: HashSet::new(),
        }
    }

    pub fn transform(&self, index: Index) -> Option<Ref<Transform>> {
        let index = index.into_raw_parts().0;
        self.transforms.get(index).map(|t| t.borrow())
    }

    pub fn transform_mut(&self, index: Index) -> Option<RefMut<Transform>> {
        let index = index.into_raw_parts().0;
        self.transforms.get(index).map(|t| t.borrow_mut())
    }

    pub fn iter(&self) -> impl Iterator<Item = (usize, Ref<Transform>)> {
        self.transforms
            .iter()
            .enumerate()
            .map(|(i, t)| (i, t.borrow()))
    }

    pub fn index_iter(&self) -> Vec<usize> {
        self.transforms.iter().enumerate().map(|(i, _)| i).collect()
    }

    pub fn upsert(&mut self, index: Index, transform: Transform) {
        let index = index.into_raw_parts().0;
        if index >= self.transforms.len() {
            // Insert
            self.transforms.insert(index, RefCell::new(transform));
        } else {
            // Update
            let mut t = self.transforms.get_mut(index).unwrap();
            *t.borrow_mut() = transform;
        }
    }

    pub fn add_dirty(&mut self, index: Index) {
        let index = index.into_raw_parts().0;
        self.dirty.insert(index);
    }

    pub fn remove_dirty(&mut self, index: Index) {
        let index = index.into_raw_parts().0;
        self.dirty.remove(&index);
    }
}

impl TransformManager {
    pub fn index_added(&mut self, index: Index) {
        let index = index.into_raw_parts().0;
        self.transforms
            .insert(index, RefCell::new(Transform::default()));
    }

    pub fn index_removed(&mut self, index: Index) {
        // Do nothing here for now, keep as cache
    }
}
