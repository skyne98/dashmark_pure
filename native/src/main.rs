use aabb::AABB;

mod aabb;
mod api;
mod bridge_generated;
mod bvh;
mod flat_bvh;

fn main() {
    // Generate 1000000 random AABBs between 0 and 1000 with size of 20
    let mut aabbs = Vec::new();

    let mut start = std::time::Instant::now();
    for _ in 0..1000000 {
        let x = fastrand::f64() * 1000.0;
        let y = fastrand::f64() * 1000.0;
        let aabb = AABB::new(x, y, x + 20.0, y + 20.0);
        aabbs.push(aabb);
    }
    println!("Time to generate 1000000 AABBs: {:?}", start.elapsed());

    // Now generate a BVH with it
    start = std::time::Instant::now();
    let bvh = bvh::BVH::build(&aabbs);
    println!("Time to generate BVH: {:?}", start.elapsed());

    // Now generate a FlatBVH with it
    start = std::time::Instant::now();
    let flat_bvh = bvh.flatten();
    println!("Time to generate FlatBVH: {:?}", start.elapsed());
}
