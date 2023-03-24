use super::*;
// Section: wire functions

#[wasm_bindgen]
pub fn wire_say_hello(port_: MessagePort) {
    wire_say_hello_impl(port_)
}

#[wasm_bindgen]
pub fn wire_move_state_to_ui_thread() -> support::WireSyncReturn {
    wire_move_state_to_ui_thread_impl()
}

#[wasm_bindgen]
pub fn wire_request_draw() -> support::WireSyncReturn {
    wire_request_draw_impl()
}

#[wasm_bindgen]
pub fn wire_request_resize(width: u32, height: u32) -> support::WireSyncReturn {
    wire_request_resize_impl(width, height)
}

#[wasm_bindgen]
pub fn wire_set_current_time(time: f64) -> support::WireSyncReturn {
    wire_set_current_time_impl(time)
}

// Section: allocate functions

// Section: related functions

// Section: impl Wire2Api

// Section: impl Wire2Api for JsValue

impl Wire2Api<f64> for JsValue {
    fn wire2api(self) -> f64 {
        self.unchecked_into_f64() as _
    }
}
impl Wire2Api<u32> for JsValue {
    fn wire2api(self) -> u32 {
        self.unchecked_into_f64() as _
    }
}
