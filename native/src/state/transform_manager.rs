use std::{
    cell::{Ref, RefCell, RefMut},
    collections::HashSet,
};

use generational_arena::{Arena, Index};

use crate::transform::Transform;

pub struct TransformManager {
    transforms: Arena<RefCell<Transform>>,
    dirty: HashSet<Index>,
}

impl TransformManager {
    pub fn new() -> Self {
        Self {
            transforms: Arena::new(),
            dirty: HashSet::new(),
        }
    }

    pub fn get_transform(&self, index: Index) -> Option<Ref<Transform>> {
        self.transforms.get(index).map(|t| t.borrow())
    }

    pub fn get_transform_mut(&self, index: Index) -> Option<RefMut<Transform>> {
        self.transforms.get(index).map(|t| t.borrow_mut())
    }

    pub fn iter(&self) -> impl Iterator<Item = (Index, Ref<Transform>)> {
        self.transforms.iter().map(|(i, t)| (i, t.borrow()))
    }

    pub fn index_iter(&self) -> Vec<Index> {
        self.transforms.iter().map(|(i, _)| i).collect()
    }

    pub fn upsert_transform(&mut self, index: Index, transform: Transform) {
        if let Some(t) = self.transforms.get_mut(index) {
            *t.borrow_mut() = transform;
        } else {
            self.transforms.insert(RefCell::new(transform));
        }
    }

    pub fn add_dirty(&mut self, index: Index) {
        self.dirty.insert(index);
    }

    pub fn remove_dirty(&mut self, index: Index) {
        self.dirty.remove(&index);
    }
}
