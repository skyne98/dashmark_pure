use std::clone;

use crate::{
    aabb::AABB,
    api::{morton_codes, morton_codes_async},
};

#[derive(Debug, Clone)]
pub enum BVHNode {
    Leaf(AABB),
    Node(Box<BVHNode>, Box<BVHNode>, AABB),
}

pub struct BVH {
    root: Option<Box<BVHNode>>,
}

impl BVH {
    pub fn new(aabbs: &[AABB]) -> Self {
        let cloned_aabbs = aabbs.iter().cloned().collect::<Vec<_>>();
        if aabbs.is_empty() {
            return BVH { root: None };
        }

        let centers: Vec<_> = aabbs.iter().map(|aabb| aabb.center()).collect();
        let xs = centers.iter().map(|center| center.0).collect();
        let ys = centers.iter().map(|center| center.1).collect();
        let morton_codes: Vec<u64> = morton_codes_async(xs, ys);

        let mut indices: Vec<usize> = (0..aabbs.len()).collect();

        indices.sort_unstable_by(|&a, &b| morton_codes[a].cmp(&morton_codes[b]));

        let mut nodes: Vec<BVHNode> = Vec::new();
        let mut leaves: Vec<BVHNode> = indices
            .iter()
            .map(|&i| BVHNode::Leaf(cloned_aabbs[i]))
            .collect();

        while leaves.len() + nodes.len() > 1 {
            let mut new_nodes = Vec::new();

            let mut left = if let Some(_) = nodes.last() {
                nodes.pop().unwrap()
            } else {
                leaves.pop().unwrap()
            };

            while let Some(right) = if nodes.is_empty() {
                leaves.pop()
            } else {
                nodes.last().cloned()
            } {
                let aabb = match (&left, &right) {
                    (BVHNode::Leaf(left_aabb), BVHNode::Leaf(right_aabb)) => {
                        left_aabb.merge(right_aabb)
                    }
                    (BVHNode::Node(_, _, left_aabb), BVHNode::Node(_, _, right_aabb)) => {
                        left_aabb.merge(right_aabb)
                    }
                    (BVHNode::Node(_, _, left_aabb), BVHNode::Leaf(right_aabb)) => {
                        left_aabb.merge(right_aabb)
                    }
                    (BVHNode::Leaf(left_aabb), BVHNode::Node(_, _, right_aabb)) => {
                        left_aabb.merge(right_aabb)
                    }
                };

                let node = BVHNode::Node(Box::new(left), Box::new(right), aabb);
                new_nodes.push(node);

                if new_nodes.len() >= 2 && nodes.is_empty() {
                    break;
                }

                left = nodes.pop().unwrap_or_else(|| leaves.pop().unwrap());
            }

            nodes.extend(new_nodes.drain(..));
        }

        let root = leaves.pop().or_else(|| nodes.pop()).map(Box::new);
        BVH { root }
    }
}
