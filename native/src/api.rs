use crate::flat_bvh::FlatBVH;
use crate::{aabb::AABB, bvh::BVH};
use flutter_rust_bridge::{frb, SyncReturn, ZeroCopyBuffer};
pub use generational_arena::{Arena, Index as ExternalIndex};
use std::cell::RefCell;
use std::time::Instant;
use std::{env, mem};
pub use std::{
    ops::Deref,
    sync::{Mutex, RwLock},
};

thread_local! {
    static AABB_STORE: RefCell<Arena<AABB>> = RefCell::new(Arena::new());
    static BVH_STORE: RefCell<Arena<BVH>> = RefCell::new(Arena::new());
}

// Wrappers around external types
#[derive(Debug, Clone)]
pub struct Index {
    pub index: usize,
    pub generation: u64,
}

impl Index {
    fn from_external_index(external_index: ExternalIndex) -> Self {
        let raw_parts = external_index.into_raw_parts();
        Self {
            index: raw_parts.0,
            generation: raw_parts.1,
        }
    }

    fn to_external_index(&self) -> ExternalIndex {
        ExternalIndex::from_raw_parts(self.index, self.generation)
    }
}

pub fn say_hello_async() -> String {
    "Hello from Rust!".to_string()
}

pub fn morton_codes_async(xs: Vec<f64>, ys: Vec<f64>) -> Vec<u64> {
    let mut codes = Vec::with_capacity(xs.len());
    for i in 0..xs.len() {
        let x_double = xs[i];
        let y_double = ys[i];
        let x = (x_double * 1000000.0) as u64;
        let y = (y_double * 1000000.0) as u64;

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
pub fn aabb_new(min_x: f64, min_y: f64, max_x: f64, max_y: f64) -> SyncReturn<Index> {
    let aabb = AABB::new(min_x, min_y, max_x, max_y);
    AABB_STORE.with(|store| {
        let mut store = store.borrow_mut();
        let id = store.insert(aabb);
        store[id].id = Some(id);
        SyncReturn(Index::from_external_index(id))
    })
}

pub fn aabb_new_bulk(points: Vec<f64>) -> SyncReturn<ZeroCopyBuffer<Vec<u8>>> {
    let aabbs_len = points.len() / 4;
    if points.is_empty() {
        return SyncReturn(ZeroCopyBuffer(Vec::new()));
    }

    let mut ids = Vec::with_capacity(aabbs_len * 2);
    AABB_STORE.with(|store| {
        let mut store = store.borrow_mut();

        for i in 0..aabbs_len {
            let offset = i * 4;
            let min_x = points[offset];
            let min_y = points[offset + 1];
            let max_x = points[offset + 2];
            let max_y = points[offset + 3];
            let aabb = AABB::new(min_x, min_y, max_x, max_y);
            let id = store.insert(aabb);
            let (id_index, id_gen) = id.into_raw_parts();
            store[id].id = Some(id);

            ids.push(id_index as u64);
            ids.push(id_gen as u64);
        }

        let ids = unsafe {
            let ratio = mem::size_of::<u64>() / mem::size_of::<u8>();

            let length = ids.len() * ratio;
            let capacity = ids.capacity() * ratio;
            let ptr = ids.as_mut_ptr() as *mut u8;

            // Don't run the destructor for ids
            mem::forget(ids);

            // Construct new Vec
            Vec::from_raw_parts(ptr, length, capacity)
        };
        SyncReturn(ZeroCopyBuffer(ids))
    })
}

pub fn aabb_drop_bulk(aabb_ids: Vec<Index>) -> SyncReturn<Vec<u8>> {
    let mut results = Vec::with_capacity(aabb_ids.len());
    AABB_STORE.with(|store| {
        let mut store = store.borrow_mut();
        for aabb_id in aabb_ids {
            let existing = store.remove(aabb_id.to_external_index());
            results.push(existing.is_some());
        }
        let results_u8 = results.iter().map(|x| *x as u8).collect();
        SyncReturn(results_u8)
    })
}

pub fn aabb_min(aabb_id: Index) -> SyncReturn<Vec<f64>> {
    AABB_STORE.with(|store| {
        let store = store.borrow();
        let aabb = &store[aabb_id.to_external_index()];
        SyncReturn(vec![aabb.min_x, aabb.min_y])
    })
}

pub fn aabb_max(aabb_id: Index) -> SyncReturn<Vec<f64>> {
    AABB_STORE.with(|store| {
        let store = store.borrow();
        let aabb = &store[aabb_id.to_external_index()];
        SyncReturn(vec![aabb.max_x, aabb.max_y])
    })
}

pub fn aabb_size(aabb_id: Index) -> SyncReturn<[f64; 2]> {
    AABB_STORE.with(|store| {
        let store = store.borrow();
        let aabb = &store[aabb_id.to_external_index()];
        SyncReturn(aabb.size())
    })
}

pub fn aabb_center(aabb_id: Index) -> SyncReturn<[f64; 2]> {
    AABB_STORE.with(|store| {
        let store = store.borrow();
        let aabb = &store[aabb_id.to_external_index()];
        SyncReturn(aabb.center())
    })
}

pub fn aabb_intersects_aabb(aabb_left_id: Index, aabb_right_id: Index) -> SyncReturn<bool> {
    AABB_STORE.with(|store| {
        let store = store.borrow();
        let aabb_left = &store[aabb_left_id.to_external_index()];
        let aabb_right = &store[aabb_right_id.to_external_index()];
        SyncReturn(aabb_left.intersects_aabb(aabb_right))
    })
}

pub fn aabb_contains_point(aabb_id: Index, point: [f64; 2]) -> SyncReturn<bool> {
    AABB_STORE.with(|store| {
        let store = store.borrow();
        let aabb = &store[aabb_id.to_external_index()];
        SyncReturn(aabb.contains_point(point[0], point[1]))
    })
}

pub fn aabb_contains_aabb(aabb_left_id: Index, aabb_right_id: Index) -> SyncReturn<bool> {
    AABB_STORE.with(|store| {
        let store = store.borrow();
        let aabb_left = &store[aabb_left_id.to_external_index()];
        let aabb_right = &store[aabb_right_id.to_external_index()];
        SyncReturn(aabb_left.contains_aabb(aabb_right))
    })
}

pub fn aabb_merge(aabb_left_id: Index, aabb_right_id: Index) -> SyncReturn<Index> {
    AABB_STORE.with(|store| {
        let mut store = store.borrow_mut();
        let aabb_left = &store[aabb_left_id.to_external_index()];
        let aabb_right = &store[aabb_right_id.to_external_index()];
        let aabb = aabb_left.merge(aabb_right);
        let id = store.insert(aabb);
        store[id].id = Some(id);
        SyncReturn(Index::from_external_index(id))
    })
}

pub fn aabb_merge_with(aabb: Index, other: Index) {
    AABB_STORE.with(|store| {
        let store = &mut store.borrow_mut();
        let (mut aabb_left, aabb_right) =
            store.get2_mut(aabb.to_external_index(), other.to_external_index());
        let aabb_left = aabb_left.as_mut().expect("aabb_left is None");
        let aabb_right = aabb_right.as_ref().expect("aabb_right is None");
        aabb_left.merge_with(&aabb_right);
    })
}

// BVH API
pub fn bvh_new(aabbs: Vec<Index>) -> SyncReturn<Index> {
    AABB_STORE.with(|store| {
        BVH_STORE.with(|bvh_store| {
            let aabb_store = store.borrow();
            let mut bvh_store = bvh_store.borrow_mut();

            let vec_of_aabbs = aabbs
                .iter()
                .map(|aabb_id| aabb_store[aabb_id.to_external_index()])
                .collect::<Vec<_>>();
            let bvh = BVH::build(&vec_of_aabbs[..]);
            let bvh_id = bvh_store.insert(bvh);
            SyncReturn(Index::from_external_index(bvh_id))
        })
    })
}

pub fn bvh_new_async(aabbs: Vec<Index>) -> Index {
    AABB_STORE.with(|store| {
        BVH_STORE.with(|bvh_store| {
            let aabb_store = store.borrow();
            let mut bvh_store = bvh_store.borrow_mut();

            let vec_of_aabbs = aabbs
                .iter()
                .map(|aabb_id| aabb_store[aabb_id.to_external_index()])
                .collect::<Vec<_>>();
            let bvh = BVH::build(&vec_of_aabbs[..]);
            let bvh_id = bvh_store.insert(bvh);
            Index::from_external_index(bvh_id)
        })
    })
}

pub fn bvh_drop(bvh_id: Index) -> SyncReturn<bool> {
    BVH_STORE.with(|store| {
        let mut store = store.borrow_mut();
        let existing = store.remove(bvh_id.to_external_index());
        SyncReturn(existing.is_some())
    })
}

pub fn bvh_flatten(bvh_id: Index) -> SyncReturn<FlatBVH> {
    BVH_STORE.with(|store| {
        let store = store.borrow();
        let bvh = &store[bvh_id.to_external_index()];
        let flat_bvh = bvh.flatten();
        SyncReturn(flat_bvh)
    })
}

pub fn bvh_flatten_async(bvh_id: Index) -> FlatBVH {
    BVH_STORE.with(|store| {
        let store = store.borrow();
        let bvh = &store[bvh_id.to_external_index()];
        let flat_bvh = bvh.flatten();
        flat_bvh
    })
}

pub fn bvh_depth(bvh_id: Index) -> SyncReturn<u64> {
    BVH_STORE.with(|store| {
        let store = store.borrow();
        let bvh = &store[bvh_id.to_external_index()];
        let depth = bvh.depth();
        SyncReturn(depth as u64)
    })
}

pub fn bvh_depth_async(bvh_id: Index) -> u64 {
    BVH_STORE.with(|store| {
        let store = store.borrow();
        let bvh = &store[bvh_id.to_external_index()];
        let depth = bvh.depth();
        depth as u64
    })
}

pub fn bvh_query_aabb_collisions(bvh_id: Index, aabb_id: Index) -> SyncReturn<Vec<Index>> {
    AABB_STORE.with(|store| {
        BVH_STORE.with(|bvh_store| {
            let aabb_store = store.borrow();
            let bvh_store = bvh_store.borrow();
            let aabb = &aabb_store[aabb_id.to_external_index()];
            let bvh = &bvh_store[bvh_id.to_external_index()];
            let collisions = bvh.query_aabb_collisions(aabb);
            let collisions_wrapped = collisions
                .iter()
                .map(|collision| Index::from_external_index(*collision))
                .collect::<Vec<_>>();
            SyncReturn(collisions_wrapped)
        })
    })
}

pub fn bvh_query_aabb_collisions_min_max(
    bvh_id: Index,
    min_x: f64,
    min_y: f64,
    max_x: f64,
    max_y: f64,
) -> SyncReturn<Vec<Index>> {
    BVH_STORE.with(|store| {
        let store = store.borrow();
        let bvh = &store[bvh_id.to_external_index()];
        let aabb = AABB::new(min_x, min_y, max_x, max_y);
        let collisions = bvh.query_aabb_collisions(&aabb);
        let collisions_wrapped = collisions
            .iter()
            .map(|collision| Index::from_external_index(*collision))
            .collect::<Vec<_>>();
        SyncReturn(collisions_wrapped)
    })
}

pub fn bvh_query_point_collisions(bvh_id: Index, x: f64, y: f64) -> SyncReturn<Vec<Index>> {
    BVH_STORE.with(|store| {
        let store = store.borrow();
        let bvh = &store[bvh_id.to_external_index()];
        let collisions = bvh.query_point_collisions(x, y);
        let collisions_wrapped = collisions
            .iter()
            .map(|collision| Index::from_external_index(*collision))
            .collect::<Vec<_>>();
        SyncReturn(collisions_wrapped)
    })
}

pub fn bvh_print(bvh_id: Index) -> SyncReturn<String> {
    BVH_STORE.with(|store| {
        let store = store.borrow();
        let bvh = &store[bvh_id.to_external_index()];
        let bvh_str = bvh.print_bvh();
        SyncReturn(bvh_str)
    })
}

pub fn bvh_print_async(bvh_id: Index) -> String {
    BVH_STORE.with(|store| {
        let store = store.borrow();
        let bvh = &store[bvh_id.to_external_index()];
        let bvh_str = bvh.print_bvh();
        bvh_str
    })
}

pub fn bvh_overlap_ratio(bvh_id: Index) -> SyncReturn<f64> {
    BVH_STORE.with(|store| {
        let store = store.borrow();
        let bvh = &store[bvh_id.to_external_index()];
        let overlap_ratio = bvh.overlap_ratio();
        SyncReturn(overlap_ratio)
    })
}
