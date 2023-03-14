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
pub extern "C" fn wire_aabb_new_bulk_benchmark(
    min_xs: *mut wire_float_64_list,
    min_ys: *mut wire_float_64_list,
    max_xs: *mut wire_float_64_list,
    max_ys: *mut wire_float_64_list,
) -> support::WireSyncReturn {
    wire_aabb_new_bulk_benchmark_impl(min_xs, min_ys, max_xs, max_ys)
}

#[no_mangle]
pub extern "C" fn wire_aabb_min(aabb: wire_RwLockAabb) -> support::WireSyncReturn {
    wire_aabb_min_impl(aabb)
}

#[no_mangle]
pub extern "C" fn wire_aabb_max(aabb: wire_RwLockAabb) -> support::WireSyncReturn {
    wire_aabb_max_impl(aabb)
}

#[no_mangle]
pub extern "C" fn wire_aabb_size(aabb: wire_RwLockAabb) -> support::WireSyncReturn {
    wire_aabb_size_impl(aabb)
}

#[no_mangle]
pub extern "C" fn wire_aabb_center(aabb: wire_RwLockAabb) -> support::WireSyncReturn {
    wire_aabb_center_impl(aabb)
}

#[no_mangle]
pub extern "C" fn wire_aabb_intersects(
    aabb_left: wire_RwLockAabb,
    aabb_right: wire_RwLockAabb,
) -> support::WireSyncReturn {
    wire_aabb_intersects_impl(aabb_left, aabb_right)
}

#[no_mangle]
pub extern "C" fn wire_aabb_contains(
    aabb: wire_RwLockAabb,
    point: *mut wire_float_64_list,
) -> support::WireSyncReturn {
    wire_aabb_contains_impl(aabb, point)
}

#[no_mangle]
pub extern "C" fn wire_aabb_contains_aabb(
    aabb_left: wire_RwLockAabb,
    aabb_right: wire_RwLockAabb,
) -> support::WireSyncReturn {
    wire_aabb_contains_aabb_impl(aabb_left, aabb_right)
}

#[no_mangle]
pub extern "C" fn wire_aabb_merge(
    aabb_left: wire_RwLockAabb,
    aabb_right: wire_RwLockAabb,
) -> support::WireSyncReturn {
    wire_aabb_merge_impl(aabb_left, aabb_right)
}

#[no_mangle]
pub extern "C" fn wire_aabb_merge_with(
    aabb: wire_RwLockAabb,
    other: wire_RwLockAabb,
) -> support::WireSyncReturn {
    wire_aabb_merge_with_impl(aabb, other)
}

#[no_mangle]
pub extern "C" fn wire_bvh_new(aabbs: *mut wire_list_RwLockAabb) -> support::WireSyncReturn {
    wire_bvh_new_impl(aabbs)
}

#[no_mangle]
pub extern "C" fn wire_bvh_new_async(port_: i64, aabbs: *mut wire_list_RwLockAabb) {
    wire_bvh_new_async_impl(port_, aabbs)
}

// Section: allocate functions

#[no_mangle]
pub extern "C" fn new_RwLockAabb() -> wire_RwLockAabb {
    wire_RwLockAabb::new_with_null_ptr()
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
pub extern "C" fn new_list_RwLockAabb_0(len: i32) -> *mut wire_list_RwLockAabb {
    let wrap = wire_list_RwLockAabb {
        ptr: support::new_leak_vec_ptr(<wire_RwLockAabb>::new_with_null_ptr(), len),
        len,
    };
    support::new_leak_box_ptr(wrap)
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

#[no_mangle]
pub extern "C" fn drop_opaque_RwLockBvh(ptr: *const c_void) {
    unsafe {
        Arc::<RwLock<BVH>>::decrement_strong_count(ptr as _);
    }
}

#[no_mangle]
pub extern "C" fn share_opaque_RwLockBvh(ptr: *const c_void) -> *const c_void {
    unsafe {
        Arc::<RwLock<BVH>>::increment_strong_count(ptr as _);
        ptr
    }
}

// Section: impl Wire2Api

impl Wire2Api<RustOpaque<RwLock<AABB>>> for wire_RwLockAabb {
    fn wire2api(self) -> RustOpaque<RwLock<AABB>> {
        unsafe { support::opaque_from_dart(self.ptr as _) }
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
impl Wire2Api<Vec<RustOpaque<RwLock<AABB>>>> for *mut wire_list_RwLockAabb {
    fn wire2api(self) -> Vec<RustOpaque<RwLock<AABB>>> {
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
pub struct wire_RwLockAabb {
    ptr: *const core::ffi::c_void,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_float_64_list {
    ptr: *mut f64,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_RwLockAabb {
    ptr: *mut wire_RwLockAabb,
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

impl NewWithNullPtr for wire_RwLockAabb {
    fn new_with_null_ptr() -> Self {
        Self {
            ptr: core::ptr::null(),
        }
    }
}

// Section: sync execution mode utility

#[no_mangle]
pub extern "C" fn free_WireSyncReturn(ptr: support::WireSyncReturn) {
    unsafe {
        let _ = support::box_from_leak_ptr(ptr);
    };
}
