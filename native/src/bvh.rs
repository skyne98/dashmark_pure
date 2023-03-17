use std::collections::HashSet;

use generational_arena::Index;

use crate::{aabb::AABB, api::morton_codes_async, flat_bvh::FlatBVH};

#[derive(Debug, Clone)]
struct IntervalBin {
    aabb: AABB,
    primitive_count: u64,
}

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
    pub fn build(aabbs: &[AABB]) -> Self {
        if aabbs.len() == 0 {
            return Self { nodes: vec![] };
        }

        let mut nodes = vec![];
        Self::build_recursive(&mut nodes, aabbs);

        Self { nodes }
    }

    fn build_recursive(nodes: &mut Vec<BVHNode>, aabbs: &[AABB]) -> usize {
        let current_index = nodes.len();

        if aabbs.len() == 1 {
            nodes.push(BVHNode::Leaf(aabbs[0].clone()));
        } else {
            // let mut merged_aabb = AABB::empty();
            // for aabb in &aabbs {
            //     merged_aabb.merge_with(aabb);
            // }
            // let split_axis = merged_aabb.longest_axis();
            // let split_position = merged_aabb.center()[split_axis];

            // let (split_position, split_axis) = Self::find_split_best(&aabbs[..]);
            let (split_position, split_axis) = Self::find_split_uniform(&aabbs[..]);
            let mut left_aabbs = vec![];
            let mut right_aabbs = vec![];
            let mut left_aabb = AABB::empty();
            let mut right_aabb = AABB::empty();

            for aabb in aabbs {
                if aabb.center()[split_axis] < split_position {
                    left_aabbs.push(*aabb);
                    left_aabb.merge_with(&aabb);
                } else {
                    right_aabbs.push(*aabb);
                    right_aabb.merge_with(&aabb);
                }
            }

            // Special measure for when all the boxes are in the same spot
            if (left_aabbs.len() == 0) || (right_aabbs.len() == 0) {
                // Doesn't matter, split the group directly in half
                let half = aabbs.len() / 2;
                left_aabbs = aabbs[..half].to_vec();
                right_aabbs = aabbs[half..].to_vec();

                left_aabb = AABB::empty();
                right_aabb = AABB::empty();

                for aabb in left_aabbs.iter() {
                    left_aabb.merge_with(&aabb);
                }

                for aabb in right_aabbs.iter() {
                    right_aabb.merge_with(&aabb);
                }
            }

            // Insert a placeholder node
            nodes.push(BVHNode::empty());

            let left_index = Self::build_recursive(nodes, &left_aabbs[..]);
            let right_index = Self::build_recursive(nodes, &right_aabbs[..]);
            let merged_aabb = left_aabb.merge(&right_aabb);
            nodes[current_index] =
                BVHNode::Internal(left_index as u64, right_index as u64, merged_aabb);
        }

        current_index
    }

    fn find_split_best(aabbs: &[AABB]) -> (f64, usize) {
        // Use SAH to find the best split along the best axis
        let mut best_cost = std::f64::MAX;
        let mut best_position = 0.0;
        let mut best_axis = 0;

        for axis in 0..2 {
            for aabb in aabbs {
                let position = aabb.center()[axis];
                let cost = Self::evaluate_sah(position, axis, aabbs);

                if cost < best_cost {
                    best_cost = cost;
                    best_position = position;
                    best_axis = axis;
                }
            }
        }

        (best_position, best_axis)
    }

    fn find_split_uniform(aabbs: &[AABB]) -> (f64, usize) {
        const NUM_SPLITS: u8 = 16;

        // Use SAH to find the best split along the best axis
        let mut best_cost = std::f64::MAX;
        let mut best_position = 0.0;
        let mut best_axis = 0;

        for axis in 0..2 {
            let mut bounds_min = f64::INFINITY;
            let mut bounds_max = f64::NEG_INFINITY;

            // Simple improvement to BVH quality
            // https://jacco.ompf2.com/2022/04/21/how-to-build-a-bvh-part-3-quick-builds/
            // this effectively allows us to make split intervals smaller
            for aabb in aabbs {
                let position = aabb.center()[axis];
                if position < bounds_min {
                    bounds_min = position;
                }
                if position > bounds_max {
                    bounds_max = position;
                }
            }
            if bounds_min == bounds_max {
                continue;
            }

            // Populate the bins
            let mut bins = vec![
                IntervalBin {
                    aabb: AABB::empty(),
                    primitive_count: 0
                };
                NUM_SPLITS as usize
            ];
            let bounds_range = bounds_max - bounds_min;
            let split_size = bounds_range / NUM_SPLITS as f64;
            for aabb in aabbs {
                let bin_id = usize::min(
                    ((aabb.center()[axis] - bounds_min) / split_size) as usize,
                    NUM_SPLITS as usize - 1,
                );
                bins[bin_id].primitive_count += 1;
                bins[bin_id].aabb.merge_with(aabb);
            }
            // Gather data for the planes (bins - 1) between the bins
            let mut left_area = vec![0.0; NUM_SPLITS as usize - 1];
            let mut left_count = vec![0; NUM_SPLITS as usize - 1];
            let mut right_area = vec![0.0; NUM_SPLITS as usize - 1];
            let mut right_count = vec![0; NUM_SPLITS as usize - 1];
            let mut left_box = AABB::empty();
            let mut right_box = AABB::empty();
            let mut left_sum = 0;
            let mut right_sum = 0;
            for i in 0..NUM_SPLITS as usize - 1 {
                left_sum += bins[i].primitive_count;
                left_count[i] = left_sum;
                left_box.merge_with(&bins[i].aabb);
                left_area[i] = left_box.area();

                right_sum += bins[NUM_SPLITS as usize - 1 - i].primitive_count;
                right_count[NUM_SPLITS as usize - 2 - i] = right_sum;
                right_box.merge_with(&bins[NUM_SPLITS as usize - 1 - i].aabb);
                right_area[NUM_SPLITS as usize - 2 - i] = right_box.area();
            }

            // Calculate SAH cost for each plane
            for i in 0..NUM_SPLITS as usize - 1 {
                let cost =
                    left_area[i] * left_count[i] as f64 + right_area[i] * right_count[i] as f64;
                if cost < best_cost {
                    best_cost = cost;
                    best_position = bounds_min + (i as f64 + 0.5) * split_size;
                    best_axis = axis;
                }
            }
        }

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
                left_aabb.merge_with(aabb);
                left_count += 1;
            } else {
                right_aabb.merge_with(aabb);
                right_count += 1;
            }
        }

        if left_count == 0 || right_count == 0 {
            return std::f64::MAX;
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
        min_x.push(aabb.min_x);
        min_y.push(aabb.min_y);
        max_x.push(aabb.max_x);
        max_y.push(aabb.max_y);
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

    pub fn query_point_collisions(&self, point_x: f64, point_y: f64) -> Vec<Index> {
        if self.nodes.is_empty() {
            return vec![];
        }
        let mut results = Vec::new();
        self.query_point_collisions_recursive(0, point_x, point_y, &mut results);
        results
    }

    fn query_point_collisions_recursive(
        &self,
        node_index: usize,
        point_x: f64,
        point_y: f64,
        results: &mut Vec<Index>,
    ) {
        let node = &self.nodes[node_index];
        if node.get_aabb().contains_point(point_x, point_y) {
            match node {
                BVHNode::Leaf(aabb) => {
                    if let Some(id) = aabb.id {
                        results.push(id);
                    }
                }
                BVHNode::Internal(left_child_index, right_child_index, _) => {
                    self.query_point_collisions_recursive(
                        *left_child_index as usize,
                        point_x,
                        point_y,
                        results,
                    );
                    self.query_point_collisions_recursive(
                        *right_child_index as usize,
                        point_x,
                        point_y,
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
                        min_x[i as usize],
                        min_y[i as usize],
                        max_x[i as usize],
                        max_y[i as usize],
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
