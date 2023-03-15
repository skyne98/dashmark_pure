use crate::{aabb::AABB, api::morton_codes_async, flat_bvh::FlatBVH};

#[derive(Debug, Clone)]
pub enum BVHNode {
    Leaf(AABB),
    Internal(Box<BVHNode>, Box<BVHNode>, AABB),
}

pub struct BVH {
    root: Option<Box<BVHNode>>,
}

impl BVH {
    pub fn new(aabbs: &[AABB]) -> Self {
        let mut index_and_codes: Vec<(usize, u64)> = Vec::with_capacity(aabbs.len());

        let centers: Vec<_> = aabbs.iter().map(|aabb| aabb.center()).collect();
        let xs = centers.iter().map(|center| center.0).collect();
        let ys = centers.iter().map(|center| center.1).collect();
        let morton_codes = morton_codes_async(xs, ys);
        for (i, code) in morton_codes.iter().enumerate() {
            index_and_codes.push((i, *code));
        }
        index_and_codes.sort_unstable_by(|a, b| a.1.cmp(&b.1));

        let sorted_aabbs: Vec<_> = index_and_codes.iter().map(|&(i, _)| aabbs[i]).collect();
        let sorted_morton_codes: Vec<_> = index_and_codes.iter().map(|&(_, code)| code).collect();

        BVH {
            root: Some(Box::new(Self::build_lbv_nodes(
                &sorted_aabbs,
                &sorted_morton_codes,
                0,
                aabbs.len() - 1,
            ))),
        }
    }
    fn build_lbv_nodes(
        primitives: &[AABB],
        morton_codes: &[u64],
        start: usize,
        end: usize,
    ) -> BVHNode {
        if start == end {
            BVHNode::Leaf(primitives[start])
        } else {
            let split = Self::find_morton_split(morton_codes, start, end);

            let left_child = Self::build_lbv_nodes(primitives, morton_codes, start, split);
            let right_child = Self::build_lbv_nodes(primitives, morton_codes, split + 1, end);

            let mut bbox = AABB::empty();
            if let BVHNode::Leaf(ref child_bbox) | BVHNode::Internal(_, _, ref child_bbox) =
                left_child
            {
                bbox.merge_with(child_bbox);
            }
            if let BVHNode::Leaf(ref child_bbox) | BVHNode::Internal(_, _, ref child_bbox) =
                right_child
            {
                bbox.merge_with(child_bbox);
            }

            BVHNode::Internal(Box::new(left_child), Box::new(right_child), bbox)
        }
    }
    fn find_morton_split(codes: &[u64], start: usize, end: usize) -> usize {
        let mut split = start;
        let mut max_diff = 0;

        for i in start..end {
            let diff = (codes[i] ^ codes[i + 1]).trailing_zeros();
            if diff > max_diff {
                max_diff = diff;
                split = i;
            }
        }

        split
    }

    // Utilities
    pub fn depth(&self) -> u64 {
        if let Some(ref root) = self.root {
            Self::depth_node(root)
        } else {
            0
        }
    }
    fn depth_node(node: &BVHNode) -> u64 {
        match node {
            BVHNode::Leaf(_) => 1,
            BVHNode::Internal(left, right, _) => {
                1 + Self::depth_node(left).max(Self::depth_node(right))
            }
        }
    }

    pub fn flatten(&self) -> FlatBVH {
        let mut flat_bvh = FlatBVH {
            min_x: Vec::new(),
            min_y: Vec::new(),
            max_x: Vec::new(),
            max_y: Vec::new(),
            depth: Vec::new(),
        };

        if let Some(ref root) = self.root {
            Self::flatten_node(root, None, &mut flat_bvh, 0);
        }

        flat_bvh
    }
    fn flatten_node(node: &BVHNode, parent: Option<u64>, flat_bvh: &mut FlatBVH, depth: u64) {
        let index = flat_bvh.min_x.len();

        match node {
            BVHNode::Leaf(aabb) => {
                flat_bvh.min_x.push(aabb.min.0);
                flat_bvh.min_y.push(aabb.min.1);
                flat_bvh.max_x.push(aabb.max.0);
                flat_bvh.max_y.push(aabb.max.1);
                flat_bvh.depth.push(depth);
            }
            BVHNode::Internal(left, right, aabb) => {
                flat_bvh.min_x.push(aabb.min.0);
                flat_bvh.min_y.push(aabb.min.1);
                flat_bvh.max_x.push(aabb.max.0);
                flat_bvh.max_y.push(aabb.max.1);
                flat_bvh.depth.push(depth);

                Self::flatten_node(left, Some(index as u64), flat_bvh, depth + 1);
                Self::flatten_node(right, Some(index as u64), flat_bvh, depth + 1);
            }
        }
    }

    // Printing
    pub fn print_bvh(&self) -> String {
        if let Some(ref root) = self.root {
            Self::print_bvh_tree(root, 0)
        } else {
            String::new()
        }
    }
    fn print_bvh_tree(node: &BVHNode, depth: usize) -> String {
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
                output.push_str(&Self::print_bvh_tree(left, depth + 1));
                output.push_str(&Self::print_bvh_tree(right, depth + 1));
            }
        }
        output
    }
}
