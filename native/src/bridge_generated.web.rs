use super::*;
// Section: wire functions

#[wasm_bindgen]
pub fn wire_say_hello_async(port_: MessagePort) {
    wire_say_hello_async_impl(port_)
}

#[wasm_bindgen]
pub fn wire_morton_codes_async(port_: MessagePort, xs: Box<[f64]>, ys: Box<[f64]>) {
    wire_morton_codes_async_impl(port_, xs, ys)
}

#[wasm_bindgen]
pub fn wire_morton_codes(xs: Box<[f64]>, ys: Box<[f64]>) -> support::WireSyncReturn {
    wire_morton_codes_impl(xs, ys)
}

#[wasm_bindgen]
pub fn wire_aabb_new(min_x: f64, min_y: f64, max_x: f64, max_y: f64) -> support::WireSyncReturn {
    wire_aabb_new_impl(min_x, min_y, max_x, max_y)
}

#[wasm_bindgen]
pub fn wire_aabb_new_bulk(
    min_xs: Box<[f64]>,
    min_ys: Box<[f64]>,
    max_xs: Box<[f64]>,
    max_ys: Box<[f64]>,
) -> support::WireSyncReturn {
    wire_aabb_new_bulk_impl(min_xs, min_ys, max_xs, max_ys)
}

#[wasm_bindgen]
pub fn wire_aabb_drop_bulk(aabb_ids: JsValue) -> support::WireSyncReturn {
    wire_aabb_drop_bulk_impl(aabb_ids)
}

#[wasm_bindgen]
pub fn wire_aabb_min(aabb_id: JsValue) -> support::WireSyncReturn {
    wire_aabb_min_impl(aabb_id)
}

#[wasm_bindgen]
pub fn wire_aabb_max(aabb_id: JsValue) -> support::WireSyncReturn {
    wire_aabb_max_impl(aabb_id)
}

#[wasm_bindgen]
pub fn wire_aabb_size(aabb_id: JsValue) -> support::WireSyncReturn {
    wire_aabb_size_impl(aabb_id)
}

#[wasm_bindgen]
pub fn wire_aabb_center(aabb_id: JsValue) -> support::WireSyncReturn {
    wire_aabb_center_impl(aabb_id)
}

#[wasm_bindgen]
pub fn wire_aabb_intersects_aabb(
    aabb_left_id: JsValue,
    aabb_right_id: JsValue,
) -> support::WireSyncReturn {
    wire_aabb_intersects_aabb_impl(aabb_left_id, aabb_right_id)
}

#[wasm_bindgen]
pub fn wire_aabb_contains_point(aabb_id: JsValue, point: Box<[f64]>) -> support::WireSyncReturn {
    wire_aabb_contains_point_impl(aabb_id, point)
}

#[wasm_bindgen]
pub fn wire_aabb_contains_aabb(
    aabb_left_id: JsValue,
    aabb_right_id: JsValue,
) -> support::WireSyncReturn {
    wire_aabb_contains_aabb_impl(aabb_left_id, aabb_right_id)
}

#[wasm_bindgen]
pub fn wire_aabb_merge(aabb_left_id: JsValue, aabb_right_id: JsValue) -> support::WireSyncReturn {
    wire_aabb_merge_impl(aabb_left_id, aabb_right_id)
}

#[wasm_bindgen]
pub fn wire_aabb_merge_with(port_: MessagePort, aabb: JsValue, other: JsValue) {
    wire_aabb_merge_with_impl(port_, aabb, other)
}

#[wasm_bindgen]
pub fn wire_bvh_new(aabbs: JsValue) -> support::WireSyncReturn {
    wire_bvh_new_impl(aabbs)
}

#[wasm_bindgen]
pub fn wire_bvh_new_async(port_: MessagePort, aabbs: JsValue) {
    wire_bvh_new_async_impl(port_, aabbs)
}

#[wasm_bindgen]
pub fn wire_bvh_drop(bvh_id: JsValue) -> support::WireSyncReturn {
    wire_bvh_drop_impl(bvh_id)
}

#[wasm_bindgen]
pub fn wire_bvh_flatten(bvh_id: JsValue) -> support::WireSyncReturn {
    wire_bvh_flatten_impl(bvh_id)
}

#[wasm_bindgen]
pub fn wire_bvh_flatten_async(port_: MessagePort, bvh_id: JsValue) {
    wire_bvh_flatten_async_impl(port_, bvh_id)
}

#[wasm_bindgen]
pub fn wire_bvh_depth(bvh_id: JsValue) -> support::WireSyncReturn {
    wire_bvh_depth_impl(bvh_id)
}

#[wasm_bindgen]
pub fn wire_bvh_depth_async(port_: MessagePort, bvh_id: JsValue) {
    wire_bvh_depth_async_impl(port_, bvh_id)
}

#[wasm_bindgen]
pub fn wire_bvh_query_aabb_collisions(
    bvh_id: JsValue,
    aabb_id: JsValue,
) -> support::WireSyncReturn {
    wire_bvh_query_aabb_collisions_impl(bvh_id, aabb_id)
}

#[wasm_bindgen]
pub fn wire_bvh_query_aabb_collisions_min_max(
    bvh_id: JsValue,
    min_x: f64,
    min_y: f64,
    max_x: f64,
    max_y: f64,
) -> support::WireSyncReturn {
    wire_bvh_query_aabb_collisions_min_max_impl(bvh_id, min_x, min_y, max_x, max_y)
}

#[wasm_bindgen]
pub fn wire_bvh_query_point_collisions(bvh_id: JsValue, x: f64, y: f64) -> support::WireSyncReturn {
    wire_bvh_query_point_collisions_impl(bvh_id, x, y)
}

#[wasm_bindgen]
pub fn wire_bvh_print(bvh_id: JsValue) -> support::WireSyncReturn {
    wire_bvh_print_impl(bvh_id)
}

#[wasm_bindgen]
pub fn wire_bvh_print_async(port_: MessagePort, bvh_id: JsValue) {
    wire_bvh_print_async_impl(port_, bvh_id)
}

#[wasm_bindgen]
pub fn wire_bvh_overlap_ratio(bvh_id: JsValue) -> support::WireSyncReturn {
    wire_bvh_overlap_ratio_impl(bvh_id)
}

// Section: allocate functions

// Section: related functions

// Section: impl Wire2Api

impl Wire2Api<[f64; 2]> for Box<[f64]> {
    fn wire2api(self) -> [f64; 2] {
        let vec: Vec<f64> = self.wire2api();
        support::from_vec_to_array(vec)
    }
}
impl Wire2Api<Vec<f64>> for Box<[f64]> {
    fn wire2api(self) -> Vec<f64> {
        self.into_vec()
    }
}
impl Wire2Api<Index> for JsValue {
    fn wire2api(self) -> Index {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            2,
            "Expected 2 elements, got {}",
            self_.length()
        );
        Index {
            index: self_.get(0).wire2api(),
            generation: self_.get(1).wire2api(),
        }
    }
}
impl Wire2Api<Vec<Index>> for JsValue {
    fn wire2api(self) -> Vec<Index> {
        self.dyn_into::<JsArray>()
            .unwrap()
            .iter()
            .map(Wire2Api::wire2api)
            .collect()
    }
}

// Section: impl Wire2Api for JsValue

impl Wire2Api<f64> for JsValue {
    fn wire2api(self) -> f64 {
        self.unchecked_into_f64() as _
    }
}
impl Wire2Api<[f64; 2]> for JsValue {
    fn wire2api(self) -> [f64; 2] {
        let vec: Vec<f64> = self.wire2api();
        support::from_vec_to_array(vec)
    }
}
impl Wire2Api<Vec<f64>> for JsValue {
    fn wire2api(self) -> Vec<f64> {
        self.unchecked_into::<js_sys::Float64Array>()
            .to_vec()
            .into()
    }
}
impl Wire2Api<u64> for JsValue {
    fn wire2api(self) -> u64 {
        ::std::convert::TryInto::try_into(self.dyn_into::<js_sys::BigInt>().unwrap()).unwrap()
    }
}
impl Wire2Api<usize> for JsValue {
    fn wire2api(self) -> usize {
        self.unchecked_into_f64() as _
    }
}
