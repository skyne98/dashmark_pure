use super::*;
// Section: wire functions

#[no_mangle]
pub extern "C" fn wire_say_hello(port_: i64) {
    wire_say_hello_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_move_state_to_ui_thread() -> support::WireSyncReturn {
    wire_move_state_to_ui_thread_impl()
}

#[no_mangle]
pub extern "C" fn wire_request_draw() -> support::WireSyncReturn {
    wire_request_draw_impl()
}

#[no_mangle]
pub extern "C" fn wire_request_resize(width: u32, height: u32) -> support::WireSyncReturn {
    wire_request_resize_impl(width, height)
}

#[no_mangle]
pub extern "C" fn wire_set_current_time(time: f64) -> support::WireSyncReturn {
    wire_set_current_time_impl(time)
}

// Section: allocate functions

// Section: related functions

// Section: impl Wire2Api

// Section: wire structs

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
