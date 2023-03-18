use super::*;
// Section: wire functions

#[wasm_bindgen]
pub fn wire_say_hello(port_: MessagePort) {
    wire_say_hello_impl(port_)
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
pub fn wire_entity_set_position(index: JsValue, x: f64, y: f64) -> support::WireSyncReturn {
    wire_entity_set_position_impl(index, x, y)
}

// Section: allocate functions

// Section: related functions

// Section: impl Wire2Api

impl Wire2Api<RawIndex> for JsValue {
    fn wire2api(self) -> RawIndex {
        let self_ = self.dyn_into::<JsArray>().unwrap();
        assert_eq!(
            self_.length(),
            2,
            "Expected 2 elements, got {}",
            self_.length()
        );
        RawIndex(self_.get(0).wire2api(), self_.get(1).wire2api())
    }
}

// Section: impl Wire2Api for JsValue

impl Wire2Api<f64> for JsValue {
    fn wire2api(self) -> f64 {
        self.unchecked_into_f64() as _
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
