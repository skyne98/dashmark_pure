use super::*;
// Section: wire functions

#[wasm_bindgen]
pub fn wire_say_hello(port_: MessagePort) {
    wire_say_hello_impl(port_)
}

#[wasm_bindgen]
pub fn wire_get_message(port_: MessagePort) {
    wire_get_message_impl(port_)
}

// Section: allocate functions

// Section: related functions

// Section: impl Wire2Api

// Section: impl Wire2Api for JsValue
