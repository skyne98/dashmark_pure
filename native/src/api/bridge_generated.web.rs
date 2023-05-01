use super::*;
// Section: wire functions

#[wasm_bindgen]
pub fn wire_say_hello(port_: MessagePort) {
    wire_say_hello_impl(port_)
}

#[wasm_bindgen]
pub fn wire_update(dt: f64) -> support::WireSyncReturn {
    wire_update_impl(dt)
}

#[wasm_bindgen]
pub fn wire_create_entity() -> support::WireSyncReturn {
    wire_create_entity_impl()
}

#[wasm_bindgen]
pub fn wire_drop_entity(index: JsValue) -> support::WireSyncReturn {
    wire_drop_entity_impl(index)
}

#[wasm_bindgen]
pub fn wire_entities_set_transform_raw(
    indices: Box<[u32]>,
    positions: Box<[f32]>,
    origins: Box<[f32]>,
    rotations: Box<[f32]>,
    scales: Box<[f32]>,
) -> support::WireSyncReturn {
    wire_entities_set_transform_raw_impl(indices, positions, origins, rotations, scales)
}

#[wasm_bindgen]
pub fn wire_entities_set_position_raw(
    indices: Box<[u32]>,
    positions: Box<[f32]>,
) -> support::WireSyncReturn {
    wire_entities_set_position_raw_impl(indices, positions)
}

#[wasm_bindgen]
pub fn wire_entities_set_origin_raw(
    indices: Box<[u32]>,
    origins: Box<[f32]>,
) -> support::WireSyncReturn {
    wire_entities_set_origin_raw_impl(indices, origins)
}

#[wasm_bindgen]
pub fn wire_entities_set_rotation_raw(
    indices: Box<[u32]>,
    rotations: Box<[f32]>,
) -> support::WireSyncReturn {
    wire_entities_set_rotation_raw_impl(indices, rotations)
}

#[wasm_bindgen]
pub fn wire_entities_set_scale_raw(
    indices: Box<[u32]>,
    scales: Box<[f32]>,
) -> support::WireSyncReturn {
    wire_entities_set_scale_raw_impl(indices, scales)
}

#[wasm_bindgen]
pub fn wire_query_aabb(x: f32, y: f32, width: f32, height: f32) -> support::WireSyncReturn {
    wire_query_aabb_impl(x, y, width, height)
}

#[wasm_bindgen]
pub fn wire_query_aabb_raw(x: f32, y: f32, width: f32, height: f32) -> support::WireSyncReturn {
    wire_query_aabb_raw_impl(x, y, width, height)
}

#[wasm_bindgen]
pub fn wire_entity_set_vertices_raw(
    index: JsValue,
    vertices: Box<[f32]>,
) -> support::WireSyncReturn {
    wire_entity_set_vertices_raw_impl(index, vertices)
}

#[wasm_bindgen]
pub fn wire_entity_set_tex_coords_raw(
    index: JsValue,
    tex_coords: Box<[f32]>,
) -> support::WireSyncReturn {
    wire_entity_set_tex_coords_raw_impl(index, tex_coords)
}

#[wasm_bindgen]
pub fn wire_entity_set_indices_raw(index: JsValue, indices: Box<[u16]>) -> support::WireSyncReturn {
    wire_entity_set_indices_raw_impl(index, indices)
}

#[wasm_bindgen]
pub fn wire_entities_set_priority_raw(
    indices: Box<[u32]>,
    priorities: Box<[i32]>,
) -> support::WireSyncReturn {
    wire_entities_set_priority_raw_impl(indices, priorities)
}

#[wasm_bindgen]
pub fn wire_entity_set_shape(index: JsValue, shape: JsValue) -> support::WireSyncReturn {
    wire_entity_set_shape_impl(index, shape)
}

#[wasm_bindgen]
pub fn wire_entity_set_color(index: JsValue, color: i32) -> support::WireSyncReturn {
    wire_entity_set_color_impl(index, color)
}

#[wasm_bindgen]
pub fn wire_batches_count() -> support::WireSyncReturn {
    wire_batches_count_impl()
}

#[wasm_bindgen]
pub fn wire_vertices(batch_index: u16) -> support::WireSyncReturn {
    wire_vertices_impl(batch_index)
}

#[wasm_bindgen]
pub fn wire_tex_coords(batch_index: u16) -> support::WireSyncReturn {
    wire_tex_coords_impl(batch_index)
}

#[wasm_bindgen]
pub fn wire_indices(batch_index: u16) -> support::WireSyncReturn {
    wire_indices_impl(batch_index)
}

#[wasm_bindgen]
pub fn wire_colors(batch_index: u16) -> support::WireSyncReturn {
    wire_colors_impl(batch_index)
}

// Section: allocate functions

// Section: related functions

// Section: impl Wire2Api

impl Wire2Api<Vec<f32>> for Box<[f32]> {
    fn wire2api(self) -> Vec<f32> {
        self.into_vec()
    }
}
impl Wire2Api<GenerationalIndex> for JsValue {
    fn wire2api(self) -> GenerationalIndex {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            2,
            "Expected 2 elements, got {}",
            self_.length()
        );
        GenerationalIndex(self_.get(0).wire2api(), self_.get(1).wire2api())
    }
}

impl Wire2Api<Vec<i32>> for Box<[i32]> {
    fn wire2api(self) -> Vec<i32> {
        self.into_vec()
    }
}
impl Wire2Api<Vec<Shape>> for JsValue {
    fn wire2api(self) -> Vec<Shape> {
        self.dyn_into::<JsArray>()
            .unwrap()
            .iter()
            .map(Wire2Api::wire2api)
            .collect()
    }
}
impl Wire2Api<Vec<ShapeTransform>> for JsValue {
    fn wire2api(self) -> Vec<ShapeTransform> {
        self.dyn_into::<JsArray>()
            .unwrap()
            .iter()
            .map(Wire2Api::wire2api)
            .collect()
    }
}
impl Wire2Api<Shape> for JsValue {
    fn wire2api(self) -> Shape {
        let self_ = self.unchecked_into::<JsArray>();
        match self_.get(0).unchecked_into_f64() as _ {
            0 => Shape::Ball {
                radius: self_.get(1).wire2api(),
            },
            1 => Shape::Compound {
                children: self_.get(1).wire2api(),
                transforms: self_.get(2).wire2api(),
            },
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<ShapeTransform> for JsValue {
    fn wire2api(self) -> ShapeTransform {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            5,
            "Expected 5 elements, got {}",
            self_.length()
        );
        ShapeTransform {
            position_x: self_.get(0).wire2api(),
            position_y: self_.get(1).wire2api(),
            rotation: self_.get(2).wire2api(),
            absolute_origin_x: self_.get(3).wire2api(),
            absolute_origin_y: self_.get(4).wire2api(),
        }
    }
}

impl Wire2Api<Vec<u16>> for Box<[u16]> {
    fn wire2api(self) -> Vec<u16> {
        self.into_vec()
    }
}
impl Wire2Api<Vec<u32>> for Box<[u32]> {
    fn wire2api(self) -> Vec<u32> {
        self.into_vec()
    }
}

// Section: impl Wire2Api for JsValue

impl Wire2Api<f32> for JsValue {
    fn wire2api(self) -> f32 {
        self.unchecked_into_f64() as _
    }
}
impl Wire2Api<f64> for JsValue {
    fn wire2api(self) -> f64 {
        self.unchecked_into_f64() as _
    }
}
impl Wire2Api<Vec<f32>> for JsValue {
    fn wire2api(self) -> Vec<f32> {
        self.unchecked_into::<js_sys::Float32Array>()
            .to_vec()
            .into()
    }
}
impl Wire2Api<i32> for JsValue {
    fn wire2api(self) -> i32 {
        self.unchecked_into_f64() as _
    }
}
impl Wire2Api<Vec<i32>> for JsValue {
    fn wire2api(self) -> Vec<i32> {
        self.unchecked_into::<js_sys::Int32Array>().to_vec().into()
    }
}
impl Wire2Api<u16> for JsValue {
    fn wire2api(self) -> u16 {
        self.unchecked_into_f64() as _
    }
}
impl Wire2Api<u32> for JsValue {
    fn wire2api(self) -> u32 {
        self.unchecked_into_f64() as _
    }
}
impl Wire2Api<u64> for JsValue {
    fn wire2api(self) -> u64 {
        ::std::convert::TryInto::try_into(self.dyn_into::<js_sys::BigInt>().unwrap()).unwrap()
    }
}
impl Wire2Api<Vec<u16>> for JsValue {
    fn wire2api(self) -> Vec<u16> {
        self.unchecked_into::<js_sys::Uint16Array>().to_vec().into()
    }
}
impl Wire2Api<Vec<u32>> for JsValue {
    fn wire2api(self) -> Vec<u32> {
        self.unchecked_into::<js_sys::Uint32Array>().to_vec().into()
    }
}
impl Wire2Api<usize> for JsValue {
    fn wire2api(self) -> usize {
        self.unchecked_into_f64() as _
    }
}
