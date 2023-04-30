use crate::matrix::{bulk_transform_vectors_mut, bulk_transform_vectors_mut_n, TransformMatrix};
use rapier2d_f64::na::Vector2;
use rapier2d_f64::parry::bounding_volume::Aabb as ParryAabb;
use rapier2d_f64::parry::na::Point2;
use rapier2d_f64::parry::partitioning::Qbvh as ParryQbvh;

mod api;
mod bvh;
mod entity;
mod index;
mod matrix;
mod state;
mod transform;
mod typed_data;

struct Data {
    a: i64,
    b: i64,
    flag: bool,
}

fn main() {
    // Test cache misses
    // Prepare the data
    let amount = 100_000_000 as usize;
    let mut data = vec![];
    for i in 0..amount {
        data.push(Data {
            a: i as i64,
            b: i as i64 * 2,
            flag: i % 2 == 0,
        });
    }

    // Process the data
    let mut result = vec![0; amount as usize];
    let now = std::time::Instant::now();
    for i in 0..amount {
        let data = &data[i];
        if data.flag {
            result[i] = data.a + data.b;
        } else {
            result[i] = data.a * data.b;
        }
    }
    println!("Cache-optimized version took {:?}", now.elapsed());

    // Setup cache-miss version
    let mut result = vec![0; amount as usize];
    let mut a = vec![];
    let mut b = vec![];
    let mut flag = vec![];
    for (index, data) in data.iter().enumerate() {
        a.push(data.a);
        b.push(data.b);
        flag.push(data.flag);
    }
    let now = std::time::Instant::now();
    for i in 0..amount {
        let a = a[i];
        let b = b[i];
        let flag = flag[i];
        if flag {
            result[i] = a + b;
        } else {
            result[i] = a * b;
        }
    }
    println!("Cache-miss version took {:?}", now.elapsed());

    const iterations: u32 = 1000000;

    // Make a parry_2d_f64 test
    let mut aabbs = Vec::new();

    let mut start = std::time::Instant::now();
    for _ in 0..iterations {
        let x = fastrand::f64() * 1000.0;
        let y = fastrand::f64() * 1000.0;
        let one_aabb = ParryAabb::new(Point2::new(x, y), Point2::new(x + 20.0, y + 20.0));
        aabbs.push((0 as u64, one_aabb));
    }
    println!(
        "Time to generate iterations ParryAABBs: {:?}",
        start.elapsed()
    );

    // Now generate a BVH with it
    start = std::time::Instant::now();
    let mut bvh = ParryQbvh::new();
    bvh.clear_and_rebuild(aabbs.into_iter(), 0.0);
    println!("Time to generate ParryQbvh: {:?}", start.elapsed());

    // Now do iterations / 10 intersection tests
    start = std::time::Instant::now();
    for _ in 0..iterations / 10 {
        let x = fastrand::f64() * 1000.0;
        let y = fastrand::f64() * 1000.0;
        let one_aabb = ParryAabb::new(Point2::new(x, y), Point2::new(x + 20.0, y + 20.0));
        let mut results = Vec::new();
        bvh.intersect_aabb(&one_aabb, &mut results);
    }
    let time = start.elapsed();
    println!(
        "Time to do {} ParryQbvh intersection tests: {:?}",
        iterations / 10,
        time
    );
    let time_for_per_query_per_aabb =
        time.as_millis() as f64 / iterations as f64 / iterations as f64 * 10.0;
    println!(
        "Time for per query per aabb: {:?}",
        time_for_per_query_per_aabb
    );
    let time_for_500_queries_2000_aabbs = time_for_per_query_per_aabb * 500 as f64 * 2000 as f64;
    println!(
        "Time for 500 queries 2000 aabbs: {:?}",
        time_for_500_queries_2000_aabbs
    );

    // Do 1000000 matrix and vector allocations, then do
    // 1000000 matrix and vector transformations
    start = std::time::Instant::now();
    let mut matrices = Vec::with_capacity(iterations as usize);
    matrices.resize(iterations as usize, TransformMatrix::new());
    println!(
        "Time to generate iterations matrices: {:?}",
        start.elapsed()
    );

    start = std::time::Instant::now();
    let mut vectors = Vec::with_capacity(iterations as usize);
    vectors.resize(iterations as usize, Vector2::new(0.0, 0.0));
    println!("Time to generate iterations vectors: {:?}", start.elapsed());

    start = std::time::Instant::now();
    let mut results = Vec::with_capacity(iterations as usize);
    for _ in 0..iterations {
        let matrix = matrices.pop().unwrap();
        let vector = vectors.pop().unwrap();
        results.push(matrix.transform_vector(&vector));
    }
    println!(
        "Time to do iterations matrix and vector transformations: {:?}",
        start.elapsed()
    );

    start = std::time::Instant::now();
    let matrix = TransformMatrix::new();
    bulk_transform_vectors_mut(&matrix.matrix, &mut vectors);
    println!(
        "Time to do iterations bulk matrix and vector transformations: {:?}",
        start.elapsed()
    );

    let transform_matrices = matrices.iter().map(|m| m.matrix).collect::<Vec<_>>();
    start = std::time::Instant::now();
    bulk_transform_vectors_mut_n(&transform_matrices[..], &mut vectors, 1);
    println!(
        "Time to do iterations bulk matrix and vector transformations (n=1): {:?}",
        start.elapsed()
    );
}
