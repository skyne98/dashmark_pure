use super::*;
// Section: wire functions

#[no_mangle]
pub extern "C" fn wire_say_hello_async(port_: i64) {
    wire_say_hello_async_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_morton_codes_async(
    port_: i64,
    xs: *mut wire_float_64_list,
    ys: *mut wire_float_64_list,
) {
    wire_morton_codes_async_impl(port_, xs, ys)
}

#[no_mangle]
pub extern "C" fn wire_morton_codes(
    xs: *mut wire_float_64_list,
    ys: *mut wire_float_64_list,
) -> support::WireSyncReturn {
    wire_morton_codes_impl(xs, ys)
}

#[no_mangle]
pub extern "C" fn wire_aabb_new(
    min_x: f64,
    min_y: f64,
    max_x: f64,
    max_y: f64,
) -> support::WireSyncReturn {
    wire_aabb_new_impl(min_x, min_y, max_x, max_y)
}

#[no_mangle]
pub extern "C" fn wire_aabb_new_bulk(
    min_xs: *mut wire_float_64_list,
    min_ys: *mut wire_float_64_list,
    max_xs: *mut wire_float_64_list,
    max_ys: *mut wire_float_64_list,
) -> support::WireSyncReturn {
    wire_aabb_new_bulk_impl(min_xs, min_ys, max_xs, max_ys)
}

#[no_mangle]
pub extern "C" fn wire_aabb_drop(aabb_id: *mut wire_Index) -> support::WireSyncReturn {
    wire_aabb_drop_impl(aabb_id)
}

#[no_mangle]
pub extern "C" fn wire_aabb_min(aabb_id: *mut wire_Index) -> support::WireSyncReturn {
    wire_aabb_min_impl(aabb_id)
}

#[no_mangle]
pub extern "C" fn wire_aabb_max(aabb_id: *mut wire_Index) -> support::WireSyncReturn {
    wire_aabb_max_impl(aabb_id)
}

#[no_mangle]
pub extern "C" fn wire_aabb_size(aabb_id: *mut wire_Index) -> support::WireSyncReturn {
    wire_aabb_size_impl(aabb_id)
}

#[no_mangle]
pub extern "C" fn wire_aabb_center(aabb_id: *mut wire_Index) -> support::WireSyncReturn {
    wire_aabb_center_impl(aabb_id)
}

#[no_mangle]
pub extern "C" fn wire_aabb_intersects_aabb(
    aabb_left_id: *mut wire_Index,
    aabb_right_id: *mut wire_Index,
) -> support::WireSyncReturn {
    wire_aabb_intersects_aabb_impl(aabb_left_id, aabb_right_id)
}

#[no_mangle]
pub extern "C" fn wire_aabb_contains_point(
    aabb_id: *mut wire_Index,
    point: *mut wire_float_64_list,
) -> support::WireSyncReturn {
    wire_aabb_contains_point_impl(aabb_id, point)
}

#[no_mangle]
pub extern "C" fn wire_aabb_contains_aabb(
    aabb_left_id: *mut wire_Index,
    aabb_right_id: *mut wire_Index,
) -> support::WireSyncReturn {
    wire_aabb_contains_aabb_impl(aabb_left_id, aabb_right_id)
}

#[no_mangle]
pub extern "C" fn wire_aabb_merge(
    aabb_left_id: *mut wire_Index,
    aabb_right_id: *mut wire_Index,
) -> support::WireSyncReturn {
    wire_aabb_merge_impl(aabb_left_id, aabb_right_id)
}

#[no_mangle]
pub extern "C" fn wire_aabb_merge_with(port_: i64, aabb: *mut wire_Index, other: *mut wire_Index) {
    wire_aabb_merge_with_impl(port_, aabb, other)
}

#[no_mangle]
pub extern "C" fn wire_bvh_new(aabbs: *mut wire_list_index) -> support::WireSyncReturn {
    wire_bvh_new_impl(aabbs)
}

#[no_mangle]
pub extern "C" fn wire_bvh_new_async(port_: i64, aabbs: *mut wire_list_index) {
    wire_bvh_new_async_impl(port_, aabbs)
}

#[no_mangle]
pub extern "C" fn wire_bvh_drop(bvh_id: *mut wire_Index) -> support::WireSyncReturn {
    wire_bvh_drop_impl(bvh_id)
}

#[no_mangle]
pub extern "C" fn wire_bvh_flatten(bvh_id: *mut wire_Index) -> support::WireSyncReturn {
    wire_bvh_flatten_impl(bvh_id)
}

#[no_mangle]
pub extern "C" fn wire_bvh_flatten_async(port_: i64, bvh_id: *mut wire_Index) {
    wire_bvh_flatten_async_impl(port_, bvh_id)
}

#[no_mangle]
pub extern "C" fn wire_bvh_depth(bvh_id: *mut wire_Index) -> support::WireSyncReturn {
    wire_bvh_depth_impl(bvh_id)
}

#[no_mangle]
pub extern "C" fn wire_bvh_depth_async(port_: i64, bvh_id: *mut wire_Index) {
    wire_bvh_depth_async_impl(port_, bvh_id)
}

#[no_mangle]
pub extern "C" fn wire_bvh_query_aabb_collisions(
    bvh_id: *mut wire_Index,
    aabb_id: *mut wire_Index,
) -> support::WireSyncReturn {
    wire_bvh_query_aabb_collisions_impl(bvh_id, aabb_id)
}

#[no_mangle]
pub extern "C" fn wire_bvh_query_aabb_collisions_min_max(
    bvh_id: *mut wire_Index,
    min_x: f64,
    min_y: f64,
    max_x: f64,
    max_y: f64,
) -> support::WireSyncReturn {
    wire_bvh_query_aabb_collisions_min_max_impl(bvh_id, min_x, min_y, max_x, max_y)
}

#[no_mangle]
pub extern "C" fn wire_bvh_query_point_collisions(
    bvh_id: *mut wire_Index,
    x: f64,
    y: f64,
) -> support::WireSyncReturn {
    wire_bvh_query_point_collisions_impl(bvh_id, x, y)
}

#[no_mangle]
pub extern "C" fn wire_bvh_print(bvh_id: *mut wire_Index) -> support::WireSyncReturn {
    wire_bvh_print_impl(bvh_id)
}

#[no_mangle]
pub extern "C" fn wire_bvh_print_async(port_: i64, bvh_id: *mut wire_Index) {
    wire_bvh_print_async_impl(port_, bvh_id)
}

#[no_mangle]
pub extern "C" fn wire_bvh_overlap_ratio(bvh_id: *mut wire_Index) -> support::WireSyncReturn {
    wire_bvh_overlap_ratio_impl(bvh_id)
}

// Section: allocate functions

#[no_mangle]
pub extern "C" fn new_box_autoadd_index_0() -> *mut wire_Index {
    support::new_leak_box_ptr(wire_Index::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_float_64_list_0(len: i32) -> *mut wire_float_64_list {
    let ans = wire_float_64_list {
        ptr: support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    support::new_leak_box_ptr(ans)
}

#[no_mangle]
pub extern "C" fn new_list_index_0(len: i32) -> *mut wire_list_index {
    let wrap = wire_list_index {
        ptr: support::new_leak_vec_ptr(<wire_Index>::new_with_null_ptr(), len),
        len,
    };
    support::new_leak_box_ptr(wrap)
}

// Section: related functions

// Section: impl Wire2Api

impl Wire2Api<Index> for *mut wire_Index {
    fn wire2api(self) -> Index {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<Index>::wire2api(*wrap).into()
    }
}

impl Wire2Api<Vec<f64>> for *mut wire_float_64_list {
    fn wire2api(self) -> Vec<f64> {
        unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}
impl Wire2Api<Index> for wire_Index {
    fn wire2api(self) -> Index {
        Index {
            index: self.index.wire2api(),
            generation: self.generation.wire2api(),
        }
    }
}
impl Wire2Api<Vec<Index>> for *mut wire_list_index {
    fn wire2api(self) -> Vec<Index> {
        let vec = unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}

// Section: wire structs

#[repr(C)]
#[derive(Clone)]
pub struct wire_float_64_list {
    ptr: *mut f64,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_Index {
    index: usize,
    generation: u64,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_index {
    ptr: *mut wire_Index,
    len: i32,
}

// Section: impl NewWithNullPtr

pub trait NewWithNullPtr {
    fn new_with_null_ptr() -> Self;
}

impl<T> NewWithNullPtr for *mut T {
    fn new_with_null_ptr() -> Self {
        std::ptr::null_mut()
    }
}

impl NewWithNullPtr for wire_Index {
    fn new_with_null_ptr() -> Self {
        Self {
            index: Default::default(),
            generation: Default::default(),
        }
    }
}

impl Default for wire_Index {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

// Section: sync execution mode utility

#[no_mangle]
pub extern "C" fn free_WireSyncReturn(ptr: support::WireSyncReturn) {
    unsafe {
        let _ = support::box_from_leak_ptr(ptr);
    };
}
