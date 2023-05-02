use std::cell::{Ref, RefCell, RefMut};

use generational_arena::{Arena, Index};

use crate::entity::Entity;

pub struct EntityManager {
    entities: Arena<Entity>,
}

impl EntityManager {
    pub fn new() -> Self {
        Self {
            entities: Arena::new(),
        }
    }

    pub fn get_entity(&self, index: Index) -> Option<&Entity> {
        self.entities.get(index)
    }

    pub fn get_entity_unknown_gen(&self, index: usize) -> Option<(&Entity, Index)> {
        self.entities.get_unknown_gen(index)
    }

    pub fn get_entity_mut(&mut self, index: Index) -> Option<&mut Entity> {
        self.entities.get_mut(index)
    }

    pub fn iter(&self) -> impl Iterator<Item = (Index, &Entity)> {
        self.entities.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (Index, &mut Entity)> {
        self.entities.iter_mut()
    }

    pub fn index_iter(&self) -> impl Iterator<Item = Index> + '_ {
        self.entities.iter().map(|(i, _)| i)
    }

    pub fn len(&self) -> usize {
        self.entities.len()
    }

    pub fn create_entity(&mut self) -> Index {
        let index = self.entities.insert(Entity::default());
        let mut entity = self.get_entity_mut(index).unwrap();
        entity.index = index;
        index
    }

    pub fn remove_entity(&mut self, index: Index) -> Option<Entity> {
        self.entities.remove(index)
    }
}
