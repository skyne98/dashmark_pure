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
pub fn wire_morton_codes_lut_async(port_: MessagePort, xs: Box<[f64]>, ys: Box<[f64]>) {
    wire_morton_codes_lut_async_impl(port_, xs, ys)
}

#[wasm_bindgen]
pub fn wire_morton_codes_lut(xs: Box<[f64]>, ys: Box<[f64]>) -> support::WireSyncReturn {
    wire_morton_codes_lut_impl(xs, ys)
}

// Section: allocate functions

// Section: related functions

// Section: impl Wire2Api

impl Wire2Api<Vec<f64>> for Box<[f64]> {
    fn wire2api(self) -> Vec<f64> {
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
