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
pub fn wire_aabb_new_bulk_benchmark(
    min_xs: Box<[f64]>,
    min_ys: Box<[f64]>,
    max_xs: Box<[f64]>,
    max_ys: Box<[f64]>,
) -> support::WireSyncReturn {
    wire_aabb_new_bulk_benchmark_impl(min_xs, min_ys, max_xs, max_ys)
}

#[wasm_bindgen]
pub fn wire_aabb_min(aabb: JsValue) -> support::WireSyncReturn {
    wire_aabb_min_impl(aabb)
}

#[wasm_bindgen]
pub fn wire_aabb_max(aabb: JsValue) -> support::WireSyncReturn {
    wire_aabb_max_impl(aabb)
}

#[wasm_bindgen]
pub fn wire_aabb_size(aabb: JsValue) -> support::WireSyncReturn {
    wire_aabb_size_impl(aabb)
}

#[wasm_bindgen]
pub fn wire_aabb_center(aabb: JsValue) -> support::WireSyncReturn {
    wire_aabb_center_impl(aabb)
}

#[wasm_bindgen]
pub fn wire_aabb_intersects(aabb_left: JsValue, aabb_right: JsValue) -> support::WireSyncReturn {
    wire_aabb_intersects_impl(aabb_left, aabb_right)
}

#[wasm_bindgen]
pub fn wire_aabb_contains(aabb: JsValue, point: Box<[f64]>) -> support::WireSyncReturn {
    wire_aabb_contains_impl(aabb, point)
}

#[wasm_bindgen]
pub fn wire_aabb_contains_aabb(aabb_left: JsValue, aabb_right: JsValue) -> support::WireSyncReturn {
    wire_aabb_contains_aabb_impl(aabb_left, aabb_right)
}

#[wasm_bindgen]
pub fn wire_aabb_merge(aabb_left: JsValue, aabb_right: JsValue) -> support::WireSyncReturn {
    wire_aabb_merge_impl(aabb_left, aabb_right)
}

#[wasm_bindgen]
pub fn wire_aabb_merge_with(aabb: JsValue, other: JsValue) -> support::WireSyncReturn {
    wire_aabb_merge_with_impl(aabb, other)
}

#[wasm_bindgen]
pub fn wire_bvh_new(aabbs: JsValue) -> support::WireSyncReturn {
    wire_bvh_new_impl(aabbs)
}

#[wasm_bindgen]
pub fn wire_bvh_new_async(port_: MessagePort, aabbs: JsValue) {
    wire_bvh_new_async_impl(port_, aabbs)
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

#[wasm_bindgen]
pub fn drop_opaque_RwLockBvh(ptr: *const c_void) {
    unsafe {
        Arc::<RwLock<BVH>>::decrement_strong_count(ptr as _);
    }
}

#[wasm_bindgen]
pub fn share_opaque_RwLockBvh(ptr: *const c_void) -> *const c_void {
    unsafe {
        Arc::<RwLock<BVH>>::increment_strong_count(ptr as _);
        ptr
    }
}

// Section: impl Wire2Api

impl Wire2Api<Vec<f64>> for Box<[f64]> {
    fn wire2api(self) -> Vec<f64> {
        self.into_vec()
    }
}
impl Wire2Api<Vec<RustOpaque<RwLock<AABB>>>> for JsValue {
    fn wire2api(self) -> Vec<RustOpaque<RwLock<AABB>>> {
        self.dyn_into::<JsArray>()
            .unwrap()
            .iter()
            .map(Wire2Api::wire2api)
            .collect()
    }
}
// Section: impl Wire2Api for JsValue

impl Wire2Api<RustOpaque<RwLock<AABB>>> for JsValue {
    fn wire2api(self) -> RustOpaque<RwLock<AABB>> {
        #[cfg(target_pointer_width = "64")]
        {
            compile_error!("64-bit pointers are not supported.");
        }

        unsafe { support::opaque_from_dart((self.as_f64().unwrap() as usize) as _) }
    }
}
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
