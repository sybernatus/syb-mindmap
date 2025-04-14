use web_sys::js_sys::{Object, Reflect};
use web_sys::wasm_bindgen::JsValue;
use web_sys::MessageEvent;

#[derive(Debug, Clone, PartialEq)]
pub struct WebviewEvent {
    pub event: MessageEvent,
}

impl WebviewEvent {
    pub fn new(event: MessageEvent) -> Self {
        Self { event }
    }

    pub fn get_data(&self) -> JsValue {
        self.event.data()
    }

    pub fn get_origin(&self) -> String {
        self.event.origin()
    }

    #[allow(dead_code)]
    pub fn get_source(&self) -> Option<Object> {
        self.event.source()
    }

    pub fn data_has_field(&self, obj: JsValue , key: &str) -> bool {
        Reflect::get(&obj, &JsValue::from_str(key))
            .map(|v| !v.is_undefined())
            .unwrap_or(false)
    }

    pub fn data_has_one_of_fields(&self, obj: JsValue , keys: Vec<&str>) -> bool {
        for key in keys {
            if self.data_has_field(obj.clone(), key) {
                return true;
            }
        }
        false
    }

    pub fn is_origin_http(&self) -> bool {
        self.get_origin().as_str().contains("http://")
    }
}