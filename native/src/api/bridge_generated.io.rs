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

#[no_mangle]
pub extern "C" fn wire_entities_set_position_raw(
    indices: *mut wire_uint_8_list,
    positions: *mut wire_uint_8_list,
) -> support::WireSyncReturn {
    wire_entities_set_position_raw_impl(indices, positions)
}

#[no_mangle]
pub extern "C" fn wire_entity_set_origin(
    index: *mut wire_RawIndex,
    relative: bool,
    x: f64,
    y: f64,
) -> support::WireSyncReturn {
    wire_entity_set_origin_impl(index, relative, x, y)
}

#[no_mangle]
pub extern "C" fn wire_entity_set_rotation(
    index: *mut wire_RawIndex,
    rotation: f64,
) -> support::WireSyncReturn {
    wire_entity_set_rotation_impl(index, rotation)
}

#[no_mangle]
pub extern "C" fn wire_entities_set_rotation_raw(
    indices: *mut wire_uint_8_list,
    rotations: *mut wire_uint_8_list,
) -> support::WireSyncReturn {
    wire_entities_set_rotation_raw_impl(indices, rotations)
}

#[no_mangle]
pub extern "C" fn wire_entity_set_shape(
    index: *mut wire_RawIndex,
    shape: *mut wire_Shape,
) -> support::WireSyncReturn {
    wire_entity_set_shape_impl(index, shape)
}

#[no_mangle]
pub extern "C" fn wire_create_bvh() -> support::WireSyncReturn {
    wire_create_bvh_impl()
}

#[no_mangle]
pub extern "C" fn wire_drop_bvh(index: *mut wire_RawIndex) -> support::WireSyncReturn {
    wire_drop_bvh_impl(index)
}

#[no_mangle]
pub extern "C" fn wire_bvh_clear_and_rebuild(
    index: *mut wire_RawIndex,
    entities: *mut wire_list_raw_index,
    dilation_factor: f64,
) -> support::WireSyncReturn {
    wire_bvh_clear_and_rebuild_impl(index, entities, dilation_factor)
}

#[no_mangle]
pub extern "C" fn wire_bvh_clear_and_rebuild_raw(
    index: *mut wire_RawIndex,
    data: *mut wire_uint_8_list,
    dilation_factor: f64,
) -> support::WireSyncReturn {
    wire_bvh_clear_and_rebuild_raw_impl(index, data, dilation_factor)
}

#[no_mangle]
pub extern "C" fn wire_bvh_flatten(index: *mut wire_RawIndex) -> support::WireSyncReturn {
    wire_bvh_flatten_impl(index)
}

// Section: allocate functions

#[no_mangle]
pub extern "C" fn new_box_autoadd_raw_index_0() -> *mut wire_RawIndex {
    support::new_leak_box_ptr(wire_RawIndex::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_shape_0() -> *mut wire_Shape {
    support::new_leak_box_ptr(wire_Shape::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_shape_0() -> *mut wire_Shape {
    support::new_leak_box_ptr(wire_Shape::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_list_box_shape_0(len: i32) -> *mut wire_list_box_shape {
    let wrap = wire_list_box_shape {
        ptr: support::new_leak_vec_ptr(<wire_Shape>::new_with_null_ptr(), len),
        len,
    };
    support::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn new_list_raw_index_0(len: i32) -> *mut wire_list_raw_index {
    let wrap = wire_list_raw_index {
        ptr: support::new_leak_vec_ptr(<wire_RawIndex>::new_with_null_ptr(), len),
        len,
    };
    support::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn new_list_shape_transform_0(len: i32) -> *mut wire_list_shape_transform {
    let wrap = wire_list_shape_transform {
        ptr: support::new_leak_vec_ptr(<wire_ShapeTransform>::new_with_null_ptr(), len),
        len,
    };
    support::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn new_uint_8_list_0(len: i32) -> *mut wire_uint_8_list {
    let ans = wire_uint_8_list {
        ptr: support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    support::new_leak_box_ptr(ans)
}

// Section: related functions

// Section: impl Wire2Api

impl Wire2Api<ZeroCopyBuffer<Vec<u8>>> for *mut wire_uint_8_list {
    fn wire2api(self) -> ZeroCopyBuffer<Vec<u8>> {
        ZeroCopyBuffer(self.wire2api())
    }
}

impl Wire2Api<RawIndex> for *mut wire_RawIndex {
    fn wire2api(self) -> RawIndex {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<RawIndex>::wire2api(*wrap).into()
    }
}
impl Wire2Api<Shape> for *mut wire_Shape {
    fn wire2api(self) -> Shape {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<Shape>::wire2api(*wrap).into()
    }
}
impl Wire2Api<Box<Shape>> for *mut wire_Shape {
    fn wire2api(self) -> Box<Shape> {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<Shape>::wire2api(*wrap).into()
    }
}

impl Wire2Api<Vec<Box<Shape>>> for *mut wire_list_box_shape {
    fn wire2api(self) -> Vec<Box<Shape>> {
        let vec = unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}
impl Wire2Api<Vec<RawIndex>> for *mut wire_list_raw_index {
    fn wire2api(self) -> Vec<RawIndex> {
        let vec = unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}
impl Wire2Api<Vec<ShapeTransform>> for *mut wire_list_shape_transform {
    fn wire2api(self) -> Vec<ShapeTransform> {
        let vec = unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}
impl Wire2Api<RawIndex> for wire_RawIndex {
    fn wire2api(self) -> RawIndex {
        RawIndex(self.field0.wire2api(), self.field1.wire2api())
    }
}
impl Wire2Api<Shape> for wire_Shape {
    fn wire2api(self) -> Shape {
        match self.tag {
            0 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.Ball);
                Shape::Ball {
                    radius: ans.radius.wire2api(),
                }
            },
            1 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.Compound);
                Shape::Compound {
                    children: ans.children.wire2api(),
                    transforms: ans.transforms.wire2api(),
                }
            },
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<ShapeTransform> for wire_ShapeTransform {
    fn wire2api(self) -> ShapeTransform {
        ShapeTransform {
            position_x: self.position_x.wire2api(),
            position_y: self.position_y.wire2api(),
            rotation: self.rotation.wire2api(),
            absolute_origin_x: self.absolute_origin_x.wire2api(),
            absolute_origin_y: self.absolute_origin_y.wire2api(),
        }
    }
}

impl Wire2Api<Vec<u8>> for *mut wire_uint_8_list {
    fn wire2api(self) -> Vec<u8> {
        unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}

// Section: wire structs

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_box_shape {
    ptr: *mut wire_Shape,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_raw_index {
    ptr: *mut wire_RawIndex,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_shape_transform {
    ptr: *mut wire_ShapeTransform,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_RawIndex {
    field0: usize,
    field1: u64,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_ShapeTransform {
    position_x: f64,
    position_y: f64,
    rotation: f64,
    absolute_origin_x: f64,
    absolute_origin_y: f64,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_uint_8_list {
    ptr: *mut u8,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_Shape {
    tag: i32,
    kind: *mut ShapeKind,
}

#[repr(C)]
pub union ShapeKind {
    Ball: *mut wire_Shape_Ball,
    Compound: *mut wire_Shape_Compound,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_Shape_Ball {
    radius: f64,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_Shape_Compound {
    children: *mut wire_list_box_shape,
    transforms: *mut wire_list_shape_transform,
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

impl NewWithNullPtr for wire_Shape {
    fn new_with_null_ptr() -> Self {
        Self {
            tag: -1,
            kind: core::ptr::null_mut(),
        }
    }
}

#[no_mangle]
pub extern "C" fn inflate_Shape_Ball() -> *mut ShapeKind {
    support::new_leak_box_ptr(ShapeKind {
        Ball: support::new_leak_box_ptr(wire_Shape_Ball {
            radius: Default::default(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_Shape_Compound() -> *mut ShapeKind {
    support::new_leak_box_ptr(ShapeKind {
        Compound: support::new_leak_box_ptr(wire_Shape_Compound {
            children: core::ptr::null_mut(),
            transforms: core::ptr::null_mut(),
        }),
    })
}

impl NewWithNullPtr for wire_ShapeTransform {
    fn new_with_null_ptr() -> Self {
        Self {
            position_x: Default::default(),
            position_y: Default::default(),
            rotation: Default::default(),
            absolute_origin_x: Default::default(),
            absolute_origin_y: Default::default(),
        }
    }
}

impl Default for wire_ShapeTransform {
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
