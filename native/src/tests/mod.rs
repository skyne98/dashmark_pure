use crate::{aabb::AABB, bvh::BVH};

#[test]
fn hello_world_test() {
    assert_eq!(true, true);
}

// Test out the BVH
#[test]
fn bvh_splits() {
    let left_aabb = AABB::new((0.0, 0.0), (0.5, 0.5));
    let right_aabb = AABB::new((1.0, 0.0), (1.5, 0.5));
    let vec = vec![left_aabb, right_aabb];
    let vec_of_refs = vec.iter().collect::<Vec<&AABB>>();
    let bvh = BVH::build(&vec_of_refs[..]);
    assert!(bvh.depth() == 2);
}

#[test]
fn bvh_splits_two_boxes_in_one_spot() {
    let left_aabb = AABB::new((0.0, 0.0), (0.5, 0.5));
    let right_aabb = AABB::new((0.0, 0.0), (0.5, 0.5));
    let vec = vec![left_aabb, right_aabb];
    let vec_of_refs = vec.iter().collect::<Vec<&AABB>>();
    let bvh = BVH::build(&vec_of_refs[..]);
    assert!(bvh.depth() == 2);
}
