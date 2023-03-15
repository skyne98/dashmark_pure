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
pub fn wire_aabb_min(aabb_id: u64) -> support::WireSyncReturn {
    wire_aabb_min_impl(aabb_id)
}

#[wasm_bindgen]
pub fn wire_aabb_max(aabb_id: u64) -> support::WireSyncReturn {
    wire_aabb_max_impl(aabb_id)
}

#[wasm_bindgen]
pub fn wire_aabb_size(aabb_id: u64) -> support::WireSyncReturn {
    wire_aabb_size_impl(aabb_id)
}

#[wasm_bindgen]
pub fn wire_aabb_center(aabb_id: u64) -> support::WireSyncReturn {
    wire_aabb_center_impl(aabb_id)
}

#[wasm_bindgen]
pub fn wire_aabb_intersects_point(
    aabb_left_id: u64,
    aabb_right_id: u64,
) -> support::WireSyncReturn {
    wire_aabb_intersects_point_impl(aabb_left_id, aabb_right_id)
}

#[wasm_bindgen]
pub fn wire_aabb_contains(aabb_id: u64, point: Box<[f64]>) -> support::WireSyncReturn {
    wire_aabb_contains_impl(aabb_id, point)
}

#[wasm_bindgen]
pub fn wire_aabb_contains_aabb(aabb_left_id: u64, aabb_right_id: u64) -> support::WireSyncReturn {
    wire_aabb_contains_aabb_impl(aabb_left_id, aabb_right_id)
}

#[wasm_bindgen]
pub fn wire_aabb_merge(aabb_left_id: u64, aabb_right_id: u64) -> support::WireSyncReturn {
    wire_aabb_merge_impl(aabb_left_id, aabb_right_id)
}

#[wasm_bindgen]
pub fn wire_aabb_merge_with(port_: MessagePort, aabb_id: u64, other_id: u64) {
    wire_aabb_merge_with_impl(port_, aabb_id, other_id)
}

#[wasm_bindgen]
pub fn wire_bvh_new(aabbs: Box<[u64]>) -> support::WireSyncReturn {
    wire_bvh_new_impl(aabbs)
}

#[wasm_bindgen]
pub fn wire_bvh_new_async(port_: MessagePort, aabbs: Box<[u64]>) {
    wire_bvh_new_async_impl(port_, aabbs)
}

#[wasm_bindgen]
pub fn wire_bvh_flatten(bvh_id: u64) -> support::WireSyncReturn {
    wire_bvh_flatten_impl(bvh_id)
}

#[wasm_bindgen]
pub fn wire_bvh_flatten_async(port_: MessagePort, bvh_id: u64) {
    wire_bvh_flatten_async_impl(port_, bvh_id)
}

#[wasm_bindgen]
pub fn wire_bvh_depth(bvh_id: u64) -> support::WireSyncReturn {
    wire_bvh_depth_impl(bvh_id)
}

#[wasm_bindgen]
pub fn wire_bvh_depth_async(port_: MessagePort, bvh_id: u64) {
    wire_bvh_depth_async_impl(port_, bvh_id)
}

#[wasm_bindgen]
pub fn wire_bvh_print(bvh_id: u64) -> support::WireSyncReturn {
    wire_bvh_print_impl(bvh_id)
}

#[wasm_bindgen]
pub fn wire_bvh_print_async(port_: MessagePort, bvh_id: u64) {
    wire_bvh_print_async_impl(port_, bvh_id)
}

// Section: allocate functions

// Section: related functions

#[wasm_bindgen]
pub fn drop_opaque_RwLockAabb(ptr: *const c_void) {
    unsafe {
        Arc::<RwLock<AABB>>::decrement_strong_count(ptr as _);
    }
}

#[wasm_bindgen]
pub fn share_opaque_RwLockAabb(ptr: *const c_void) -> *const c_void {
    unsafe {
        Arc::<RwLock<AABB>>::increment_strong_count(ptr as _);
        ptr
    }
}

// Section: impl Wire2Api

impl Wire2Api<Vec<f64>> for Box<[f64]> {
    fn wire2api(self) -> Vec<f64> {
        self.into_vec()
    }
}

impl Wire2Api<Vec<u64>> for Box<[u64]> {
    fn wire2api(self) -> Vec<u64> {
        self.into_vec()
    }
}
// Section: impl Wire2Api for JsValue

impl Wire2Api<f64> for JsValue {
    fn wire2api(self) -> f64 {
        self.unchecked_into_f64() as _
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
impl Wire2Api<Vec<u64>> for JsValue {
    fn wire2api(self) -> Vec<u64> {
        let buf = self.dyn_into::<js_sys::BigUint64Array>().unwrap();
        let buf = js_sys::Uint8Array::new(&buf.buffer());
        support::slice_from_byte_buffer(buf.to_vec()).into()
    }
}
