use generational_arena::{Arena, Index};

use crate::{bvh::Bvh, entity::Entity};

pub struct State {
    pub entities: Arena<Entity>,
    pub bvh: Arena<Bvh>,
}

impl State {
    pub fn new() -> Self {
        Self {
            entities: Arena::new(),
            bvh: Arena::new(),
        }
    }

    pub fn get_entity(&self, index: Index) -> Option<&Entity> {
        self.entities.get(index)
    }

    pub fn get_entity_mut(&mut self, index: Index) -> Option<&mut Entity> {
        self.entities.get_mut(index)
    }

    pub fn get_bvh(&self, index: Index) -> Option<&Bvh> {
        self.bvh.get(index)
    }

    pub fn get_bvh_mut(&mut self, index: Index) -> Option<&mut Bvh> {
        self.bvh.get_mut(index)
    }

    pub fn add_entity(&mut self, entity: Entity) -> Index {
        let index = self.entities.insert(entity);
        let entity = self.entities.get_mut(index).unwrap();
        entity.index = index;
        index
    }

    pub fn add_bvh(&mut self, bvh: Bvh) -> Index {
        let index = self.bvh.insert(bvh);
        let bvh = self.bvh.get_mut(index).unwrap();
        bvh.index = index;
        index
    }

    pub fn remove_entity(&mut self, index: Index) -> Option<Entity> {
        self.entities.remove(index)
    }

    pub fn remove_bvh(&mut self, index: Index) -> Option<Bvh> {
        self.bvh.remove(index)
    }
}
