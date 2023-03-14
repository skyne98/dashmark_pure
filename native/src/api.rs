pub use crate::{aabb::AABB, bvh::BVH};
use flutter_rust_bridge::{RustOpaque, SyncReturn};
pub use std::{
    ops::Deref,
    sync::{Mutex, RwLock},
};

pub fn say_hello_async() -> String {
    "Hello from Rust!".to_string()
}

pub fn morton_codes_async(xs: Vec<f64>, ys: Vec<f64>) -> Vec<u64> {
    let mut codes = Vec::with_capacity(xs.len());
    for i in 0..xs.len() {
        let x_double = xs[i];
        let y_double = ys[i];
        // let x = (x_double * 1000000.0) as u64;
        // let y = (y_double * 1000000.0) as u64;
        let x = x_double as u64;
        let y = y_double as u64;

        // Naive method
        let x = (x | (x << 32)) & 0x00000000FFFFFFFF;
        let y = (y | (y << 32)) & 0x00000000FFFFFFFF;
        let x = (x | (x << 16)) & 0x0000FFFF0000FFFF;
        let y = (y | (y << 16)) & 0x0000FFFF0000FFFF;
        let x = (x | (x << 8)) & 0x00FF00FF00FF00FF;
        let y = (y | (y << 8)) & 0x00FF00FF00FF00FF;
        let x = (x | (x << 4)) & 0x0F0F0F0F0F0F0F0F;
        let y = (y | (y << 4)) & 0x0F0F0F0F0F0F0F0F;
        let x = (x | (x << 2)) & 0x3333333333333333;
        let y = (y | (y << 2)) & 0x3333333333333333;
        let x = (x | (x << 1)) & 0x5555555555555555;
        let y = (y | (y << 1)) & 0x5555555555555555;

        let code = x | (y << 1);
        codes.push(code as u64);
    }
    codes
}

pub fn morton_codes(xs: Vec<f64>, ys: Vec<f64>) -> SyncReturn<Vec<u64>> {
    SyncReturn(morton_codes_async(xs, ys))
}

// AABB API
pub fn aabb_new(
    min_x: f64,
    min_y: f64,
    max_x: f64,
    max_y: f64,
) -> SyncReturn<RustOpaque<RwLock<AABB>>> {
    SyncReturn(RustOpaque::new(RwLock::new(AABB::new(
        (min_x, min_y),
        (max_x, max_y),
    ))))
}

pub fn aabb_new_bulk(
    min_xs: Vec<f64>,
    min_ys: Vec<f64>,
    max_xs: Vec<f64>,
    max_ys: Vec<f64>,
) -> SyncReturn<Vec<RustOpaque<RwLock<AABB>>>> {
    let mut aabbs = Vec::with_capacity(min_xs.len());
    for i in 0..min_xs.len() {
        let aabb = AABB::new((min_xs[i], min_ys[i]), (max_xs[i], max_ys[i]));
        aabbs.push(RustOpaque::new(RwLock::new(aabb)));
    }
    SyncReturn(aabbs)
}

pub fn aabb_new_bulk_benchmark(
    min_xs: Vec<f64>,
    min_ys: Vec<f64>,
    max_xs: Vec<f64>,
    max_ys: Vec<f64>,
) -> SyncReturn<u64> {
    let instant = std::time::Instant::now();
    let mut aabbs = Vec::with_capacity(min_xs.len());
    for i in 0..min_xs.len() {
        let aabb = AABB::new((min_xs[i], min_ys[i]), (max_xs[i], max_ys[i]));
        aabbs.push(RustOpaque::new(RwLock::new(aabb)));
    }
    SyncReturn(instant.elapsed().as_millis() as u64)
}

pub fn aabb_min(aabb: RustOpaque<RwLock<AABB>>) -> SyncReturn<Vec<f64>> {
    let aabb = aabb.read().unwrap();
    SyncReturn(vec![aabb.min.0, aabb.min.1])
}

pub fn aabb_max(aabb: RustOpaque<RwLock<AABB>>) -> SyncReturn<Vec<f64>> {
    let aabb = aabb.read().unwrap();
    SyncReturn(vec![aabb.max.0, aabb.max.1])
}

pub fn aabb_size(aabb: RustOpaque<RwLock<AABB>>) -> SyncReturn<Vec<f64>> {
    let aabb = aabb.read().unwrap();
    let size = aabb.size();
    SyncReturn(vec![size.0, size.1])
}

pub fn aabb_center(aabb: RustOpaque<RwLock<AABB>>) -> SyncReturn<Vec<f64>> {
    let aabb = aabb.read().unwrap();
    let center = aabb.center();
    SyncReturn(vec![center.0, center.1])
}

pub fn aabb_intersects(
    aabb_left: RustOpaque<RwLock<AABB>>,
    aabb_right: RustOpaque<RwLock<AABB>>,
) -> SyncReturn<bool> {
    let aabb_left = aabb_left.read().unwrap();
    let aabb_right = aabb_right.read().unwrap();
    SyncReturn(aabb_left.intersects(&aabb_right))
}

pub fn aabb_contains(aabb: RustOpaque<RwLock<AABB>>, point: Vec<f64>) -> SyncReturn<bool> {
    let aabb = aabb.read().unwrap();
    let point = (point[0], point[1]);
    SyncReturn(aabb.contains(point))
}

pub fn aabb_contains_aabb(
    aabb_left: RustOpaque<RwLock<AABB>>,
    aabb_right: RustOpaque<RwLock<AABB>>,
) -> SyncReturn<bool> {
    let aabb_left = aabb_left.read().unwrap();
    let aabb_right = aabb_right.read().unwrap();
    SyncReturn(aabb_left.contains_aabb(&aabb_right))
}

pub fn aabb_merge(
    aabb_left: RustOpaque<RwLock<AABB>>,
    aabb_right: RustOpaque<RwLock<AABB>>,
) -> SyncReturn<RustOpaque<RwLock<AABB>>> {
    let aabb_left = aabb_left.read().unwrap();
    let aabb_right = aabb_right.read().unwrap();
    let aabb = aabb_left.merge(&aabb_right);
    SyncReturn(RustOpaque::new(RwLock::new(aabb)))
}

pub fn aabb_merge_with(
    aabb: RustOpaque<RwLock<AABB>>,
    other: RustOpaque<RwLock<AABB>>,
) -> SyncReturn<RustOpaque<RwLock<AABB>>> {
    let mut aabb_guard = aabb.write().unwrap();
    let other = other.read().unwrap();
    aabb_guard.merge_with(&other);
    drop(aabb_guard);
    SyncReturn(aabb)
}

// BVH API
pub fn bvh_new<'a>(aabbs: Vec<RustOpaque<RwLock<AABB>>>) -> SyncReturn<RustOpaque<RwLock<BVH>>> {
    let cloned_aabbs: Vec<_> = aabbs
        .iter()
        .map(|aabb| aabb.read().unwrap().clone())
        .collect();
    let bvh = BVH::new(cloned_aabbs.as_slice());
    SyncReturn(RustOpaque::new(RwLock::new(bvh)))
}

pub fn bvh_new_async<'a>(aabbs: Vec<RustOpaque<RwLock<AABB>>>) -> RustOpaque<RwLock<BVH>> {
    let cloned_aabbs: Vec<_> = aabbs
        .iter()
        .map(|aabb| aabb.read().unwrap().clone())
        .collect();
    let bvh = BVH::new(cloned_aabbs.as_slice());
    RustOpaque::new(RwLock::new(bvh))
}
