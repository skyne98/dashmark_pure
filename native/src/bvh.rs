use std::collections::HashSet;

use generational_arena::Index;

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
            BVHNode::Leaf(aabb) => aabb.clone(),
            BVHNode::Internal(_, _, aabb) => aabb.clone(),
        }
    }
}

pub struct BVH {
    nodes: Vec<BVHNode>,
}

impl BVH {
    pub fn build(aabbs: &[&AABB]) -> Self {
        if aabbs.len() == 0 {
            return Self { nodes: vec![] };
        }

        let mut nodes = vec![];
        Self::build_recursive(&mut nodes, aabbs.iter().map(|b| (*b).clone()).collect());

        Self { nodes }
    }

    fn build_recursive(nodes: &mut Vec<BVHNode>, aabbs: Vec<AABB>) -> usize {
        let mut current_index = nodes.len();

        if aabbs.len() == 1 {
            nodes.push(BVHNode::Leaf(aabbs[0].clone()));
        } else {
            let mut merged_aabb = AABB::empty();
            for aabb in &aabbs {
                merged_aabb.merge_with(aabb);
            }
            let split_axis = merged_aabb.longest_axis();
            let split_position = merged_aabb.center()[split_axis];

            // let (split_position, split_axis) = Self::find_split(&aabbs[..]);
            let mut left_aabbs = vec![];
            let mut right_aabbs = vec![];
            let mut left_aabb = AABB::empty();
            let mut right_aabb = AABB::empty();

            for aabb in aabbs {
                if aabb.center()[split_axis] < split_position {
                    left_aabbs.push(aabb.clone());
                    left_aabb.merge_with(&aabb);
                } else {
                    right_aabbs.push(aabb.clone());
                    right_aabb.merge_with(&aabb);
                }
            }

            // Insert a placeholder node
            nodes.push(BVHNode::empty());

            let left_index = Self::build_recursive(nodes, left_aabbs);
            let right_index = Self::build_recursive(nodes, right_aabbs);

            nodes[current_index] =
                BVHNode::Internal(left_index as u64, right_index as u64, merged_aabb);
        }

        current_index
    }

    fn find_split(aabbs: &[AABB]) -> (f64, usize) {
        // Use SAH to find the best split along the best axis
        let mut best_cost = std::f64::MAX;
        let mut best_position = 0.0;
        let mut best_axis = 0;

        let mut smallest_position = std::f64::MAX;
        let mut largest_position = std::f64::MIN;

        for axis in 0..2 {
            for aabb in aabbs {
                let position = aabb.center()[axis];
                let cost = Self::evaluate_sah(position, axis, aabbs);

                if cost < best_cost {
                    best_cost = cost;
                    best_position = position;
                    best_axis = axis;
                }

                smallest_position = smallest_position.min(position);
                largest_position = largest_position.max(position);
            }
        }

        println!("Best cost: {}", best_cost);
        println!("Best position: {}", best_position);
        println!("Best axis: {}", best_axis);
        println!("Smallest position: {}", smallest_position);
        println!("Largest position: {}", largest_position);

        (best_position, best_axis)
    }

    fn evaluate_sah(position: f64, axis: usize, aabbs: &[AABB]) -> f64 {
        let mut left_aabb = AABB::empty();
        let mut left_count = 0;
        let mut right_aabb = AABB::empty();
        let mut right_count = 0;

        for aabb in aabbs {
            let other_position = aabb.center()[axis];
            if other_position < position {
                left_aabb = left_aabb.merge(aabb);
                left_count += 1;
            } else {
                right_aabb = right_aabb.merge(aabb);
                right_count += 1;
            }
        }

        let left_area = left_aabb.area();
        let right_area = right_aabb.area();

        let left_cost = left_count as f64 * left_area;
        let right_cost = right_count as f64 * right_area;
        let cost = left_cost + right_cost;

        if cost > 0.0 {
            cost
        } else {
            std::f64::MAX
        }
    }

    // Flattening
    pub fn flatten(&self) -> FlatBVH {
        let mut min_x = Vec::new();
        let mut min_y = Vec::new();
        let mut max_x = Vec::new();
        let mut max_y = Vec::new();
        let mut depth = Vec::new();

        if self.nodes.is_empty() {
            return FlatBVH {
                min_x,
                min_y,
                max_x,
                max_y,
                depth,
            };
        }

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

    // Querying
    pub fn query_aabb_collisions(&self, query_aabb: &AABB) -> Vec<Index> {
        if self.nodes.is_empty() {
            return vec![];
        }
        let mut results = Vec::new();
        self.query_aabb_collisions_recursive(0, query_aabb, &mut results);
        results
    }

    fn query_aabb_collisions_recursive(
        &self,
        node_index: usize,
        query_aabb: &AABB,
        results: &mut Vec<Index>,
    ) {
        let node = &self.nodes[node_index];
        if query_aabb.intersects_aabb(&node.get_aabb()) {
            match node {
                BVHNode::Leaf(aabb) => {
                    if let Some(id) = aabb.id {
                        results.push(id);
                    }
                }
                BVHNode::Internal(left_child_index, right_child_index, _) => {
                    self.query_aabb_collisions_recursive(
                        *left_child_index as usize,
                        query_aabb,
                        results,
                    );
                    self.query_aabb_collisions_recursive(
                        *right_child_index as usize,
                        query_aabb,
                        results,
                    );
                }
            }
        }
    }

    pub fn query_point_collisions(&self, point: (f64, f64)) -> Vec<Index> {
        if self.nodes.is_empty() {
            return vec![];
        }
        let mut results = Vec::new();
        self.query_point_collisions_recursive(0, point, &mut results);
        results
    }

    fn query_point_collisions_recursive(
        &self,
        node_index: usize,
        point: (f64, f64),
        results: &mut Vec<Index>,
    ) {
        let node = &self.nodes[node_index];
        if node.get_aabb().contains_point(point) {
            match node {
                BVHNode::Leaf(aabb) => {
                    if let Some(id) = aabb.id {
                        results.push(id);
                    }
                }
                BVHNode::Internal(left_child_index, right_child_index, _) => {
                    self.query_point_collisions_recursive(
                        *left_child_index as usize,
                        point,
                        results,
                    );
                    self.query_point_collisions_recursive(
                        *right_child_index as usize,
                        point,
                        results,
                    );
                }
            }
        }
    }

    // Printing
    pub fn print_bvh(&self) -> String {
        if self.nodes.is_empty() {
            return String::from("EMPTY BVH");
        }
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

    // Utilities
    pub fn depth(&self) -> usize {
        if self.nodes.is_empty() {
            return 0;
        }
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

    /// Returns the average ration of overlap between the bounding boxes of the nodes in the tree,
    /// which reside at the same depth. Uses the [AABB] `overlap_ratio` method.
    pub fn overlap_ratio(&self) -> f64 {
        if self.nodes.is_empty() {
            return 0.0;
        }
        let mut min_x = Vec::new();
        let mut min_y = Vec::new();
        let mut max_x = Vec::new();
        let mut max_y = Vec::new();
        let mut depth = Vec::new();
        self.flatten_recursive(
            0, 0, &mut min_x, &mut min_y, &mut max_x, &mut max_y, &mut depth,
        );
        let unique_depths: HashSet<_> = depth.iter().collect();
        let mut overlap_sum = 0.0;
        let mut count = 0;
        for this_depth in unique_depths {
            let mut aabbs = Vec::new();
            for i in 0..min_x.len() {
                if this_depth == &depth[i as usize] {
                    aabbs.push(AABB::new(
                        (min_x[i as usize], min_y[i as usize]),
                        (max_x[i as usize], max_y[i as usize]),
                    ));
                }
            }
            for i in 0..aabbs.len() {
                for j in 0..aabbs.len() {
                    if i != j {
                        overlap_sum += aabbs[i as usize].overlap_ratio(&aabbs[j as usize]);
                        count += 1;
                    }
                }
            }
        }
        overlap_sum / count as f64
    }
}
