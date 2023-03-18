use generational_arena::Index;
use rapier2d_f64::parry::partitioning::Qbvh;

use crate::{entity::Entity, index::IndexWrapper};

// Bvh
pub struct Bvh {
    pub index: Index,
    pub bvh: Qbvh<IndexWrapper>,
}

impl Bvh {
    pub fn new(index: Index) -> Self {
        Self {
            bvh: Qbvh::new(),
            index,
        }
    }

    pub fn from_entities<'a, I>(entities: I) -> Self
    where
        I: IntoIterator<Item = &'a mut Entity>,
    {
        let mut bvh = Qbvh::new();
        let mut aabbs = Vec::new();
        for entity in entities {
            aabbs.push((
                IndexWrapper(entity.index),
                entity.get_aabb().expect(
                    "Entity has no AABB, a shape is required for an entity to be in the BVH.",
                ),
            ));
        }
        bvh.clear_and_rebuild(aabbs.into_iter(), 0.0);
        Bvh {
            bvh: bvh,
            index: Index::from_raw_parts(0, 0),
        }
    }

    pub fn from_entities_and_index<'a, I>(entities: I, index: Index) -> Self
    where
        I: IntoIterator<Item = &'a mut Entity>,
    {
        let mut bvh = Self::from_entities(entities);
        bvh.index = index;
        bvh
    }

    pub fn flatten(&self) -> FlatBvh {
        FlatBvh::new(&self.bvh)
    }
}

// Flattenned
#[derive(Debug, Clone)]
pub struct FlatBvh {
    pub min_x: Vec<f64>,
    pub min_y: Vec<f64>,
    pub max_x: Vec<f64>,
    pub max_y: Vec<f64>,
    pub depth: Vec<u64>,
    pub is_leaf: Vec<bool>,
}

impl FlatBvh {
    pub fn new(bvh: &Qbvh<IndexWrapper>) -> FlatBvh {
        let mut flat_bvh = FlatBvh {
            min_x: Vec::new(),
            min_y: Vec::new(),
            max_x: Vec::new(),
            max_y: Vec::new(),
            depth: Vec::new(),
            is_leaf: Vec::new(),
        };

        let nodes = bvh.raw_nodes();
        if nodes.is_empty() {
            return flat_bvh;
        }

        let mut stack = vec![(0u32, 0u64)];

        while let Some((inode, current_depth)) = stack.pop() {
            let node = &nodes[inode as usize];
            let simd_aabb = &node.simd_aabb;

            for ii in 0..rapier2d_f64::parry::math::SIMD_WIDTH {
                let aabb = simd_aabb.extract(ii);

                println!("aabb: {:?}", aabb);
                if aabb.mins.x == f64::MAX {
                    continue;
                }
                flat_bvh.min_x.push(aabb.mins.x);
                flat_bvh.min_y.push(aabb.mins.y);
                flat_bvh.max_x.push(aabb.maxs.x);
                flat_bvh.max_y.push(aabb.maxs.y);
                flat_bvh.depth.push(current_depth);

                if node.is_leaf() == false {
                    // Internal node, visit the child.
                    let child_index = node.children[ii];

                    // Check if child index is valid
                    if child_index as usize <= nodes.len() {
                        stack.push((child_index, current_depth + 1));
                    }

                    flat_bvh.is_leaf.push(false);
                } else {
                    flat_bvh.is_leaf.push(true);
                }
            }
        }

        flat_bvh
    }
}

// ===== Tests =====
#[cfg(test)]
mod test_bvh {
    use generational_arena::Index;
    use rapier2d_f64::prelude::Ball;

    use crate::entity::Entity;

    #[test]
    fn can_build_an_empty_bvh() {
        let bvh = super::Bvh::from_entities(vec![]);
        let raw_nodes = bvh.bvh.raw_nodes();
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
        let bvh = super::Bvh::from_entities(vec![&mut entity]);
        let raw_nodes = bvh.bvh.raw_nodes();
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

    #[test]
    fn bvh_flatten() {
        let mut entity = Entity::new(Index::from_raw_parts(0, 0));
        entity.set_shape(Ball::new(2.0));
        let bvh = super::Bvh::from_entities(vec![&mut entity]);
        let flat_bvh = bvh.flatten();
        assert_eq!(flat_bvh.min_x.len(), 2);
        assert_eq!(flat_bvh.min_y.len(), 2);
        assert_eq!(flat_bvh.max_x.len(), 2);
        assert_eq!(flat_bvh.max_y.len(), 2);
        assert_eq!(flat_bvh.depth.len(), 2);
        assert_eq!(flat_bvh.is_leaf.len(), 2);

        // Check both nodes have a proper size and depth
        assert_eq!(flat_bvh.min_x[0], 0.0);
        assert_eq!(flat_bvh.min_y[0], 0.0);
        assert_eq!(flat_bvh.max_x[0], 4.0);
        assert_eq!(flat_bvh.max_y[0], 4.0);
        assert_eq!(flat_bvh.depth[0], 0);

        assert_eq!(flat_bvh.min_x[1], 0.0);
        assert_eq!(flat_bvh.min_y[1], 0.0);
        assert_eq!(flat_bvh.max_x[1], 4.0);
        assert_eq!(flat_bvh.max_y[1], 4.0);
        assert_eq!(flat_bvh.depth[1], 1);
    }
}
