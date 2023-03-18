use super::*;
// Section: wire functions

#[no_mangle]
pub extern "C" fn wire_say_hello(port_: i64) {
    wire_say_hello_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_create_entity() -> support::WireSyncReturn {
    wire_create_entity_impl()
}

#[no_mangle]
pub extern "C" fn wire_drop_entity(index: *mut wire_RawIndex) -> support::WireSyncReturn {
    wire_drop_entity_impl(index)
}

#[no_mangle]
pub extern "C" fn wire_entity_set_position(
    index: *mut wire_RawIndex,
    x: f64,
    y: f64,
) -> support::WireSyncReturn {
    wire_entity_set_position_impl(index, x, y)
}

// Section: allocate functions

#[no_mangle]
pub extern "C" fn new_box_autoadd_raw_index_0() -> *mut wire_RawIndex {
    support::new_leak_box_ptr(wire_RawIndex::new_with_null_ptr())
}

// Section: related functions

// Section: impl Wire2Api

impl Wire2Api<RawIndex> for *mut wire_RawIndex {
    fn wire2api(self) -> RawIndex {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<RawIndex>::wire2api(*wrap).into()
    }
}

impl Wire2Api<RawIndex> for wire_RawIndex {
    fn wire2api(self) -> RawIndex {
        RawIndex(self.field0.wire2api(), self.field1.wire2api())
    }
}

// Section: wire structs

#[repr(C)]
#[derive(Clone)]
pub struct wire_RawIndex {
    field0: usize,
    field1: u64,
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

impl NewWithNullPtr for wire_RawIndex {
    fn new_with_null_ptr() -> Self {
        Self {
            field0: Default::default(),
            field1: Default::default(),
        }
    }
}

impl Default for wire_RawIndex {
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
