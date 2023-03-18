use std::ops::Deref;

use generational_arena::Index;
use rapier2d_f64::parry::partitioning::{IndexedData, Qbvh};

use crate::entity::Entity;

#[derive(Debug, Clone, Copy)]
pub struct IndexWrapper(Index);
impl Deref for IndexWrapper {
    type Target = Index;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl IndexedData for IndexWrapper {
    fn default() -> Self {
        IndexWrapper(Index::from_raw_parts(0, 0))
    }

    fn index(&self) -> usize {
        self.0.into_raw_parts().0
    }
}

pub fn bvh_from_entities<'a, I>(entities: I) -> Qbvh<IndexWrapper>
where
    I: IntoIterator<Item = &'a mut Entity>,
{
    let mut bvh = Qbvh::new();
    let mut aabbs = Vec::new();
    for entity in entities {
        aabbs.push((
            IndexWrapper(entity.index),
            entity
                .get_aabb()
                .expect("Entity has no AABB, a shape is required for an entity to be in the BVH."),
        ));
    }
    bvh.clear_and_rebuild(aabbs.into_iter(), 0.0);
    bvh
}

#[derive(Debug, Clone)]
pub struct FlatBVH {
    pub min_x: Vec<f64>,
    pub min_y: Vec<f64>,
    pub max_x: Vec<f64>,
    pub max_y: Vec<f64>,
    pub depth: Vec<u64>,
}

#[cfg(test)]
mod test_bvh {
    use generational_arena::Index;
    use rapier2d_f64::prelude::Ball;

    use crate::entity::Entity;

    #[test]
    fn can_build_an_empty_bvh() {
        let bvh = super::bvh_from_entities(Vec::new());
        let raw_nodes = bvh.raw_nodes();
        assert_eq!(raw_nodes.len(), 2);
        let root_node = raw_nodes[0];
        let recursive_build_node = raw_nodes[1];
        assert!(root_node.is_leaf() == false);
        assert!(recursive_build_node.is_leaf());
    }

    #[test]
    fn bvh_and_entity_have_same_aabb() {
        let mut entity = Entity::new(Index::from_raw_parts(0, 0));
        entity.set_shape(Ball::new(2.0));
        let bvh = super::bvh_from_entities(vec![&mut entity]);
        let raw_nodes = bvh.raw_nodes();
        assert_eq!(raw_nodes.len(), 2);
        let root_node = raw_nodes[0];
        let recursive_build_node = raw_nodes[1];
        assert!(root_node.is_leaf() == false);
        assert!(recursive_build_node.is_leaf());
        let root_aabb = root_node.simd_aabb.extract(0);
        let entity_aabb = entity.get_aabb().unwrap();
        assert_eq!(root_aabb.mins, entity_aabb.mins);
        assert_eq!(root_aabb.maxs, entity_aabb.maxs);
    }
}
