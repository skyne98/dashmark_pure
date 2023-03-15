use crate::flat_bvh::FlatBVH;
pub use crate::{aabb::AABB, bvh::BVH};
use flutter_rust_bridge::{support::lazy_static, RustOpaque, SyncReturn};
pub use std::{
    ops::Deref,
    sync::{Mutex, RwLock},
};

lazy_static! {
    static ref AABB_STORE: RwLock<Vec<RustOpaque<RwLock<AABB>>>> = RwLock::new(Vec::new());
    static ref BVH_STORE: RwLock<Vec<RustOpaque<RwLock<BVH>>>> = RwLock::new(Vec::new());
}

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
pub fn aabb_new(min_x: f64, min_y: f64, max_x: f64, max_y: f64) -> SyncReturn<u64> {
    let aabb = AABB::new((min_x, min_y), (max_x, max_y));
    let aabb = RustOpaque::new(RwLock::new(aabb));
    let mut store = AABB_STORE.write().unwrap();
    store.push(aabb);
    SyncReturn((store.len() - 1) as u64)
}

pub fn aabb_new_bulk(
    min_xs: Vec<f64>,
    min_ys: Vec<f64>,
    max_xs: Vec<f64>,
    max_ys: Vec<f64>,
) -> SyncReturn<Vec<u64>> {
    let mut store = AABB_STORE.write().unwrap();
    let mut aabbs = Vec::with_capacity(min_xs.len());
    let mut ids = Vec::with_capacity(min_xs.len());
    let current_len = store.len();
    for i in 0..min_xs.len() {
        let aabb = AABB::new((min_xs[i], min_ys[i]), (max_xs[i], max_ys[i]));
        aabbs.push(RustOpaque::new(RwLock::new(aabb)));
        ids.push((current_len + i) as u64);
    }
    store.append(&mut aabbs);
    SyncReturn(ids)
}

pub fn aabb_min(aabb_id: u64) -> SyncReturn<Vec<f64>> {
    let store_lock = AABB_STORE.read().unwrap();
    let aabb = store_lock[aabb_id as usize].read().unwrap();
    SyncReturn(vec![aabb.min.0, aabb.min.1])
}

pub fn aabb_max(aabb_id: u64) -> SyncReturn<Vec<f64>> {
    let store_lock = AABB_STORE.read().unwrap();
    let aabb = store_lock[aabb_id as usize].read().unwrap();
    SyncReturn(vec![aabb.max.0, aabb.max.1])
}

pub fn aabb_size(aabb_id: u64) -> SyncReturn<Vec<f64>> {
    let store_lock = AABB_STORE.read().unwrap();
    let aabb = store_lock[aabb_id as usize].read().unwrap();
    let size = aabb.size();
    SyncReturn(vec![size.0, size.1])
}

pub fn aabb_center(aabb_id: u64) -> SyncReturn<Vec<f64>> {
    let store_lock = AABB_STORE.read().unwrap();
    let aabb = store_lock[aabb_id as usize].read().unwrap();
    let center = aabb.center();
    SyncReturn(vec![center.0, center.1])
}

pub fn aabb_intersects_point(aabb_left_id: u64, aabb_right_id: u64) -> SyncReturn<bool> {
    let store_lock = AABB_STORE.read().unwrap();
    let aabb_left = store_lock[aabb_left_id as usize].read().unwrap();
    let aabb_right = store_lock[aabb_right_id as usize].read().unwrap();
    SyncReturn(aabb_left.intersects_point(&aabb_right))
}

pub fn aabb_contains(aabb_id: u64, point: Vec<f64>) -> SyncReturn<bool> {
    let store_lock = AABB_STORE.read().unwrap();
    let aabb = store_lock[aabb_id as usize].read().unwrap();
    let point = (point[0], point[1]);
    SyncReturn(aabb.contains(point))
}

pub fn aabb_contains_aabb(aabb_left_id: u64, aabb_right_id: u64) -> SyncReturn<bool> {
    let store_lock = AABB_STORE.read().unwrap();
    let aabb_left = store_lock[aabb_left_id as usize].read().unwrap();
    let aabb_right = store_lock[aabb_right_id as usize].read().unwrap();
    SyncReturn(aabb_left.contains_aabb(&aabb_right))
}

pub fn aabb_merge(aabb_left_id: u64, aabb_right_id: u64) -> SyncReturn<RustOpaque<RwLock<AABB>>> {
    let store_lock = AABB_STORE.read().unwrap();
    let aabb_left = store_lock[aabb_left_id as usize].read().unwrap();
    let aabb_right = store_lock[aabb_right_id as usize].read().unwrap();
    let aabb = aabb_left.merge(&aabb_right);
    SyncReturn(RustOpaque::new(RwLock::new(aabb)))
}

pub fn aabb_merge_with(aabb_id: u64, other_id: u64) {
    let store_lock = AABB_STORE.read().unwrap();
    let mut aabb_guard = store_lock[aabb_id as usize].write().unwrap();
    let other = store_lock[other_id as usize].read().unwrap();
    aabb_guard.merge_with(&other);
    drop(aabb_guard);
}

// BVH API
pub fn bvh_new(aabbs: Vec<u64>) -> SyncReturn<u64> {
    let store_lock = AABB_STORE.read().unwrap();
    let cloned_aabbs: Vec<_> = aabbs
        .iter()
        .map(|aabb_id| store_lock[*aabb_id as usize].read().unwrap().clone())
        .collect();
    let bvh = BVH::new(cloned_aabbs.as_slice());
    let mut bvh_store = BVH_STORE.write().unwrap();
    bvh_store.push(RustOpaque::new(RwLock::new(bvh)));
    SyncReturn((bvh_store.len() - 1) as u64)
}

pub fn bvh_new_async(aabbs: Vec<u64>) -> u64 {
    let store_lock = AABB_STORE.read().unwrap();
    let cloned_aabbs: Vec<_> = aabbs
        .iter()
        .map(|aabb_id| store_lock[*aabb_id as usize].read().unwrap().clone())
        .collect();
    let bvh = BVH::new(cloned_aabbs.as_slice());
    let mut bvh_store = BVH_STORE.write().unwrap();
    bvh_store.push(RustOpaque::new(RwLock::new(bvh)));
    (bvh_store.len() - 1) as u64
}

pub fn bvh_flatten(bvh_id: u64) -> SyncReturn<FlatBVH> {
    let store_lock = BVH_STORE.read().unwrap();
    let bvh = store_lock[bvh_id as usize].read().unwrap();
    let flat_bvh = bvh.flatten();
    SyncReturn(flat_bvh)
}

pub fn bvh_flatten_async(bvh_id: u64) -> FlatBVH {
    let store_lock = BVH_STORE.read().unwrap();
    let bvh = store_lock[bvh_id as usize].read().unwrap();
    let flat_bvh = bvh.flatten();
    flat_bvh
}

pub fn bvh_depth(bvh_id: u64) -> SyncReturn<u64> {
    let store_lock = BVH_STORE.read().unwrap();
    let bvh = store_lock[bvh_id as usize].read().unwrap();
    let depth = bvh.depth();
    SyncReturn(depth)
}

pub fn bvh_depth_async(bvh_id: u64) -> u64 {
    let store_lock = BVH_STORE.read().unwrap();
    let bvh = store_lock[bvh_id as usize].read().unwrap();
    let depth = bvh.depth();
    depth
}

pub fn bvh_print(bvh_id: u64) -> SyncReturn<String> {
    let store_lock = BVH_STORE.read().unwrap();
    let bvh = store_lock[bvh_id as usize].read().unwrap();
    let s = bvh.print_bvh();
    SyncReturn(s)
}

pub fn bvh_print_async(bvh_id: u64) -> String {
    let store_lock = BVH_STORE.read().unwrap();
    let bvh = store_lock[bvh_id as usize].read().unwrap();
    bvh.print_bvh()
}
