use std::time::Instant;

use crate::{aabb::AABB, api::morton_codes_async, flat_bvh::FlatBVH};

#[derive(Debug, Clone)]
pub enum BVHNode {
    Leaf(AABB),
    Internal(u64, u64, AABB),
}

impl BVHNode {
    pub fn empty() -> Self {
        BVHNode::Leaf(AABB::empty())
    }

    pub fn get_aabb(&self) -> AABB {
        match self {
            BVHNode::Leaf(aabb) => *aabb,
            BVHNode::Internal(_, _, aabb) => *aabb,
        }
    }
}

pub struct BVH {
    nodes: Vec<BVHNode>,
}

impl BVH {
    pub fn build(aabbs: &[AABB]) -> Self {
        let n = aabbs.len();
        if n == 0 {
            return Self { nodes: vec![] };
        }

        let (xs, ys): (Vec<f64>, Vec<f64>) = aabbs.iter().map(|aabb| aabb.center()).unzip();

        let morton_time = Instant::now();
        let morton_codes = morton_codes_async(xs.clone(), ys.clone());
        println!("Morton codes computed in {:?}", morton_time.elapsed());

        let mut aabb_indices: Vec<usize> = (0..n).collect();
        aabb_indices.sort_unstable_by(|&a, &b| morton_codes[a].cmp(&morton_codes[b]));

        let sorted_aabbs: Vec<AABB> = aabb_indices.iter().map(|&idx| aabbs[idx]).collect();

        let mut nodes = vec![];
        Self::build_recursive(&mut nodes, &sorted_aabbs, 0, n);

        Self { nodes }
    }

    fn build_recursive(
        nodes: &mut Vec<BVHNode>,
        aabbs: &[AABB],
        start: usize,
        end: usize,
    ) -> usize {
        let n = end - start;
        if n == 1 {
            let current_index = nodes.len();
            nodes.push(BVHNode::Leaf(aabbs[start]));
            return current_index;
        }

        let split = start + n / 2;

        let current_index = nodes.len();
        nodes.push(BVHNode::empty()); // Placeholder for the internal node

        let left_child_index = Self::build_recursive(nodes, aabbs, start, split);
        let right_child_index = Self::build_recursive(nodes, aabbs, split, end);

        let left_aabb = nodes[left_child_index].get_aabb();
        let right_aabb = nodes[right_child_index].get_aabb();
        let merged_aabb = left_aabb.merge(&right_aabb);

        nodes[current_index] = BVHNode::Internal(
            left_child_index as u64,
            right_child_index as u64,
            merged_aabb,
        );

        current_index
    }

    // Utilities
    pub fn depth(&self) -> usize {
        self.depth_recursive(0)
    }

    fn depth_recursive(&self, current_idx: u64) -> usize {
        match self.nodes[current_idx as usize] {
            BVHNode::Leaf(_) => 1,
            BVHNode::Internal(left_idx, right_idx, _) => {
                1 + std::cmp::max(
                    self.depth_recursive(left_idx),
                    self.depth_recursive(right_idx),
                )
            }
        }
    }

    // Flattening
    pub fn flatten(&self) -> FlatBVH {
        let mut min_x = Vec::new();
        let mut min_y = Vec::new();
        let mut max_x = Vec::new();
        let mut max_y = Vec::new();
        let mut depth = Vec::new();

        self.flatten_recursive(
            0, 0, &mut min_x, &mut min_y, &mut max_x, &mut max_y, &mut depth,
        );

        FlatBVH {
            min_x,
            min_y,
            max_x,
            max_y,
            depth,
        }
    }

    fn flatten_recursive(
        &self,
        current_idx: u64,
        current_depth: u64,
        min_x: &mut Vec<f64>,
        min_y: &mut Vec<f64>,
        max_x: &mut Vec<f64>,
        max_y: &mut Vec<f64>,
        depth: &mut Vec<u64>,
    ) {
        let node = &self.nodes[current_idx as usize];
        let aabb = node.get_aabb();
        min_x.push(aabb.min.0);
        min_y.push(aabb.min.1);
        max_x.push(aabb.max.0);
        max_y.push(aabb.max.1);
        depth.push(current_depth);

        if let BVHNode::Internal(left_idx, right_idx, _) = node {
            self.flatten_recursive(
                *left_idx,
                current_depth + 1,
                min_x,
                min_y,
                max_x,
                max_y,
                depth,
            );
            self.flatten_recursive(
                *right_idx,
                current_depth + 1,
                min_x,
                min_y,
                max_x,
                max_y,
                depth,
            );
        }
    }

    // Printing
    pub fn print_bvh(&self) -> String {
        self.print_bvh_tree(0, 0)
    }
    fn print_bvh_tree(&self, node: u64, depth: usize) -> String {
        let node = &self.nodes[node as usize];
        let mut output = String::new();
        let indent = "│ ".repeat(depth);
        let prefix = if depth == 0 {
            String::new()
        } else {
            format!("{}├─", indent)
        };
        match node {
            BVHNode::Leaf(aabb) => {
                output.push_str(&format!("{}{} LEAF {:?}\n", prefix, indent, aabb));
            }
            BVHNode::Internal(left, right, aabb) => {
                output.push_str(&format!("{}{} INTERNAL {:?}\n", prefix, indent, aabb));
                output.push_str(&self.print_bvh_tree(*left, depth + 1));
                output.push_str(&self.print_bvh_tree(*right, depth + 1));
            }
        }
        output
    }
}
