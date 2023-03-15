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
pub extern "C" fn wire_aabb_min(aabb_id: u64) -> support::WireSyncReturn {
    wire_aabb_min_impl(aabb_id)
}

#[no_mangle]
pub extern "C" fn wire_aabb_max(aabb_id: u64) -> support::WireSyncReturn {
    wire_aabb_max_impl(aabb_id)
}

#[no_mangle]
pub extern "C" fn wire_aabb_size(aabb_id: u64) -> support::WireSyncReturn {
    wire_aabb_size_impl(aabb_id)
}

#[no_mangle]
pub extern "C" fn wire_aabb_center(aabb_id: u64) -> support::WireSyncReturn {
    wire_aabb_center_impl(aabb_id)
}

#[no_mangle]
pub extern "C" fn wire_aabb_intersects_point(
    aabb_left_id: u64,
    aabb_right_id: u64,
) -> support::WireSyncReturn {
    wire_aabb_intersects_point_impl(aabb_left_id, aabb_right_id)
}

#[no_mangle]
pub extern "C" fn wire_aabb_contains(
    aabb_id: u64,
    point: *mut wire_float_64_list,
) -> support::WireSyncReturn {
    wire_aabb_contains_impl(aabb_id, point)
}

#[no_mangle]
pub extern "C" fn wire_aabb_contains_aabb(
    aabb_left_id: u64,
    aabb_right_id: u64,
) -> support::WireSyncReturn {
    wire_aabb_contains_aabb_impl(aabb_left_id, aabb_right_id)
}

#[no_mangle]
pub extern "C" fn wire_aabb_merge(
    aabb_left_id: u64,
    aabb_right_id: u64,
) -> support::WireSyncReturn {
    wire_aabb_merge_impl(aabb_left_id, aabb_right_id)
}

#[no_mangle]
pub extern "C" fn wire_aabb_merge_with(port_: i64, aabb_id: u64, other_id: u64) {
    wire_aabb_merge_with_impl(port_, aabb_id, other_id)
}

#[no_mangle]
pub extern "C" fn wire_bvh_new(aabbs: *mut wire_uint_64_list) -> support::WireSyncReturn {
    wire_bvh_new_impl(aabbs)
}

#[no_mangle]
pub extern "C" fn wire_bvh_new_async(port_: i64, aabbs: *mut wire_uint_64_list) {
    wire_bvh_new_async_impl(port_, aabbs)
}

#[no_mangle]
pub extern "C" fn wire_bvh_flatten(bvh_id: u64) -> support::WireSyncReturn {
    wire_bvh_flatten_impl(bvh_id)
}

#[no_mangle]
pub extern "C" fn wire_bvh_flatten_async(port_: i64, bvh_id: u64) {
    wire_bvh_flatten_async_impl(port_, bvh_id)
}

#[no_mangle]
pub extern "C" fn wire_bvh_depth(bvh_id: u64) -> support::WireSyncReturn {
    wire_bvh_depth_impl(bvh_id)
}

#[no_mangle]
pub extern "C" fn wire_bvh_depth_async(port_: i64, bvh_id: u64) {
    wire_bvh_depth_async_impl(port_, bvh_id)
}

#[no_mangle]
pub extern "C" fn wire_bvh_print(bvh_id: u64) -> support::WireSyncReturn {
    wire_bvh_print_impl(bvh_id)
}

#[no_mangle]
pub extern "C" fn wire_bvh_print_async(port_: i64, bvh_id: u64) {
    wire_bvh_print_async_impl(port_, bvh_id)
}

// Section: allocate functions

#[no_mangle]
pub extern "C" fn new_float_64_list_0(len: i32) -> *mut wire_float_64_list {
    let ans = wire_float_64_list {
        ptr: support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    support::new_leak_box_ptr(ans)
}

#[no_mangle]
pub extern "C" fn new_uint_64_list_0(len: i32) -> *mut wire_uint_64_list {
    let ans = wire_uint_64_list {
        ptr: support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    support::new_leak_box_ptr(ans)
}

// Section: related functions

#[no_mangle]
pub extern "C" fn drop_opaque_RwLockAabb(ptr: *const c_void) {
    unsafe {
        Arc::<RwLock<AABB>>::decrement_strong_count(ptr as _);
    }
}

#[no_mangle]
pub extern "C" fn share_opaque_RwLockAabb(ptr: *const c_void) -> *const c_void {
    unsafe {
        Arc::<RwLock<AABB>>::increment_strong_count(ptr as _);
        ptr
    }
}

// Section: impl Wire2Api

impl Wire2Api<Vec<f64>> for *mut wire_float_64_list {
    fn wire2api(self) -> Vec<f64> {
        unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}

impl Wire2Api<Vec<u64>> for *mut wire_uint_64_list {
    fn wire2api(self) -> Vec<u64> {
        unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
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
pub struct wire_uint_64_list {
    ptr: *mut u64,
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

// Section: sync execution mode utility

#[no_mangle]
pub extern "C" fn free_WireSyncReturn(ptr: support::WireSyncReturn) {
    unsafe {
        let _ = support::box_from_leak_ptr(ptr);
    };
}
