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
pub extern "C" fn wire_morton_codes_lut_async(
    port_: i64,
    xs: *mut wire_float_64_list,
    ys: *mut wire_float_64_list,
) {
    wire_morton_codes_lut_async_impl(port_, xs, ys)
}

#[no_mangle]
pub extern "C" fn wire_morton_codes_lut(
    xs: *mut wire_float_64_list,
    ys: *mut wire_float_64_list,
) -> support::WireSyncReturn {
    wire_morton_codes_lut_impl(xs, ys)
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

// Section: related functions

// Section: impl Wire2Api

impl Wire2Api<Vec<f64>> for *mut wire_float_64_list {
    fn wire2api(self) -> Vec<f64> {
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
