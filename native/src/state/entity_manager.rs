use std::{
    cell::{Ref, RefCell, RefMut},
    rc::Rc,
};

use generational_arena::{Arena, Index};

use crate::entity::Entity;

use super::State;

pub struct EntityManager {
    entities: Arena<RefCell<Entity>>,
}

impl EntityManager {
    pub fn new() -> Self {
        Self {
            entities: Arena::new(),
        }
    }

    pub fn get_entity(&self, index: Index) -> Option<Ref<Entity>> {
        self.entities.get(index).map(|e| e.borrow())
    }

    pub fn get_entity_mut(&self, index: Index) -> Option<RefMut<Entity>> {
        self.entities.get(index).map(|e| e.borrow_mut())
    }

    pub fn iter(&self) -> impl Iterator<Item = (Index, Ref<Entity>)> {
        self.entities.iter().map(|(i, e)| (i, e.borrow()))
    }

    pub fn index_iter(&self) -> Vec<Index> {
        self.entities.iter().map(|(i, _)| i).collect()
    }

    pub fn create_entity(&mut self) -> Index {
        let index = self.entities.insert(RefCell::new(Entity::default()));
        let mut entity = self.get_entity_mut(index).unwrap();
        entity.index = index;
        index
    }

    pub fn remove_entity(&mut self, index: Index) -> Option<Entity> {
        self.entities.remove(index).map(|e| e.into_inner())
    }
}
