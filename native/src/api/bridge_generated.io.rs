use super::*;
// Section: wire functions

#[no_mangle]
pub extern "C" fn wire_say_hello(port_: i64) {
    wire_say_hello_impl(port_)
}

#[no_mangle]
pub extern "C" fn wire_screen_size_changed(width: f32, height: f32) -> support::WireSyncReturn {
    wire_screen_size_changed_impl(width, height)
}

#[no_mangle]
pub extern "C" fn wire_update(dt: f64) -> support::WireSyncReturn {
    wire_update_impl(dt)
}

#[no_mangle]
pub extern "C" fn wire_create_entity() -> support::WireSyncReturn {
    wire_create_entity_impl()
}

#[no_mangle]
pub extern "C" fn wire_drop_entity(index: *mut wire_GenerationalIndex) -> support::WireSyncReturn {
    wire_drop_entity_impl(index)
}

#[no_mangle]
pub extern "C" fn wire_entities_set_transform_raw(
    indices: *mut wire_uint_32_list,
    positions: *mut wire_float_32_list,
    origins: *mut wire_float_32_list,
    rotations: *mut wire_float_32_list,
    scales: *mut wire_float_32_list,
) -> support::WireSyncReturn {
    wire_entities_set_transform_raw_impl(indices, positions, origins, rotations, scales)
}

#[no_mangle]
pub extern "C" fn wire_entities_set_position_raw(
    indices: *mut wire_uint_32_list,
    positions: *mut wire_float_32_list,
) -> support::WireSyncReturn {
    wire_entities_set_position_raw_impl(indices, positions)
}

#[no_mangle]
pub extern "C" fn wire_entities_set_origin_raw(
    indices: *mut wire_uint_32_list,
    origins: *mut wire_float_32_list,
) -> support::WireSyncReturn {
    wire_entities_set_origin_raw_impl(indices, origins)
}

#[no_mangle]
pub extern "C" fn wire_entities_set_rotation_raw(
    indices: *mut wire_uint_32_list,
    rotations: *mut wire_float_32_list,
) -> support::WireSyncReturn {
    wire_entities_set_rotation_raw_impl(indices, rotations)
}

#[no_mangle]
pub extern "C" fn wire_entities_set_scale_raw(
    indices: *mut wire_uint_32_list,
    scales: *mut wire_float_32_list,
) -> support::WireSyncReturn {
    wire_entities_set_scale_raw_impl(indices, scales)
}

#[no_mangle]
pub extern "C" fn wire_query_aabb(
    x: f32,
    y: f32,
    width: f32,
    height: f32,
) -> support::WireSyncReturn {
    wire_query_aabb_impl(x, y, width, height)
}

#[no_mangle]
pub extern "C" fn wire_query_aabb_raw(
    x: f32,
    y: f32,
    width: f32,
    height: f32,
) -> support::WireSyncReturn {
    wire_query_aabb_raw_impl(x, y, width, height)
}

#[no_mangle]
pub extern "C" fn wire_entity_set_vertices_raw(
    index: *mut wire_GenerationalIndex,
    vertices: *mut wire_float_32_list,
) -> support::WireSyncReturn {
    wire_entity_set_vertices_raw_impl(index, vertices)
}

#[no_mangle]
pub extern "C" fn wire_entity_set_tex_coords_raw(
    index: *mut wire_GenerationalIndex,
    tex_coords: *mut wire_float_32_list,
) -> support::WireSyncReturn {
    wire_entity_set_tex_coords_raw_impl(index, tex_coords)
}

#[no_mangle]
pub extern "C" fn wire_entity_set_indices_raw(
    index: *mut wire_GenerationalIndex,
    indices: *mut wire_uint_16_list,
) -> support::WireSyncReturn {
    wire_entity_set_indices_raw_impl(index, indices)
}

#[no_mangle]
pub extern "C" fn wire_entities_set_priority_raw(
    indices: *mut wire_uint_32_list,
    priorities: *mut wire_int_32_list,
) -> support::WireSyncReturn {
    wire_entities_set_priority_raw_impl(indices, priorities)
}

#[no_mangle]
pub extern "C" fn wire_entity_set_shape(
    index: *mut wire_GenerationalIndex,
    shape: *mut wire_Shape,
) -> support::WireSyncReturn {
    wire_entity_set_shape_impl(index, shape)
}

#[no_mangle]
pub extern "C" fn wire_entity_set_color(
    index: *mut wire_GenerationalIndex,
    color: i32,
) -> support::WireSyncReturn {
    wire_entity_set_color_impl(index, color)
}

#[no_mangle]
pub extern "C" fn wire_batches_count() -> support::WireSyncReturn {
    wire_batches_count_impl()
}

#[no_mangle]
pub extern "C" fn wire_vertices(batch_index: u16) -> support::WireSyncReturn {
    wire_vertices_impl(batch_index)
}

#[no_mangle]
pub extern "C" fn wire_tex_coords(batch_index: u16) -> support::WireSyncReturn {
    wire_tex_coords_impl(batch_index)
}

#[no_mangle]
pub extern "C" fn wire_indices(batch_index: u16) -> support::WireSyncReturn {
    wire_indices_impl(batch_index)
}

#[no_mangle]
pub extern "C" fn wire_colors(batch_index: u16) -> support::WireSyncReturn {
    wire_colors_impl(batch_index)
}

// Section: allocate functions

#[no_mangle]
pub extern "C" fn new_box_autoadd_generational_index_0() -> *mut wire_GenerationalIndex {
    support::new_leak_box_ptr(wire_GenerationalIndex::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_shape_0() -> *mut wire_Shape {
    support::new_leak_box_ptr(wire_Shape::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_float_32_list_0(len: i32) -> *mut wire_float_32_list {
    let ans = wire_float_32_list {
        ptr: support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    support::new_leak_box_ptr(ans)
}

#[no_mangle]
pub extern "C" fn new_int_32_list_0(len: i32) -> *mut wire_int_32_list {
    let ans = wire_int_32_list {
        ptr: support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    support::new_leak_box_ptr(ans)
}

#[no_mangle]
pub extern "C" fn new_list_shape_0(len: i32) -> *mut wire_list_shape {
    let wrap = wire_list_shape {
        ptr: support::new_leak_vec_ptr(<wire_Shape>::new_with_null_ptr(), len),
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
pub extern "C" fn new_uint_16_list_0(len: i32) -> *mut wire_uint_16_list {
    let ans = wire_uint_16_list {
        ptr: support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    support::new_leak_box_ptr(ans)
}

#[no_mangle]
pub extern "C" fn new_uint_32_list_0(len: i32) -> *mut wire_uint_32_list {
    let ans = wire_uint_32_list {
        ptr: support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    support::new_leak_box_ptr(ans)
}

// Section: related functions

// Section: impl Wire2Api

impl Wire2Api<GenerationalIndex> for *mut wire_GenerationalIndex {
    fn wire2api(self) -> GenerationalIndex {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<GenerationalIndex>::wire2api(*wrap).into()
    }
}
impl Wire2Api<Shape> for *mut wire_Shape {
    fn wire2api(self) -> Shape {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<Shape>::wire2api(*wrap).into()
    }
}

impl Wire2Api<Vec<f32>> for *mut wire_float_32_list {
    fn wire2api(self) -> Vec<f32> {
        unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}
impl Wire2Api<GenerationalIndex> for wire_GenerationalIndex {
    fn wire2api(self) -> GenerationalIndex {
        GenerationalIndex(self.field0.wire2api(), self.field1.wire2api())
    }
}

impl Wire2Api<Vec<i32>> for *mut wire_int_32_list {
    fn wire2api(self) -> Vec<i32> {
        unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}
impl Wire2Api<Vec<Shape>> for *mut wire_list_shape {
    fn wire2api(self) -> Vec<Shape> {
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

impl Wire2Api<Vec<u16>> for *mut wire_uint_16_list {
    fn wire2api(self) -> Vec<u16> {
        unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}
impl Wire2Api<Vec<u32>> for *mut wire_uint_32_list {
    fn wire2api(self) -> Vec<u32> {
        unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}

// Section: wire structs

#[repr(C)]
#[derive(Clone)]
pub struct wire_float_32_list {
    ptr: *mut f32,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_GenerationalIndex {
    field0: usize,
    field1: u64,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_int_32_list {
    ptr: *mut i32,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_shape {
    ptr: *mut wire_Shape,
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
pub struct wire_ShapeTransform {
    position_x: f32,
    position_y: f32,
    rotation: f32,
    absolute_origin_x: f32,
    absolute_origin_y: f32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_uint_16_list {
    ptr: *mut u16,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_uint_32_list {
    ptr: *mut u32,
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
    radius: f32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_Shape_Compound {
    children: *mut wire_list_shape,
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

impl NewWithNullPtr for wire_GenerationalIndex {
    fn new_with_null_ptr() -> Self {
        Self {
            field0: Default::default(),
            field1: Default::default(),
        }
    }
}

impl Default for wire_GenerationalIndex {
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
