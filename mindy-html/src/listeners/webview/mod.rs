use crate::load_json_data;
use ::web_sys::window;
use dioxus::logger::tracing;
use serde::{Deserialize, Serialize};
use web_sys::wasm_bindgen::closure::Closure;
use web_sys::wasm_bindgen::{JsCast, JsValue};
use web_sys::MessageEvent;
use serde_wasm_bindgen::from_value;
use web_sys::js_sys::{Reflect};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct InputData {
    pub r#type: InputDataType,
    pub content: String
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum InputDataType {
    JSON,
}

pub fn init_message() {
    let init_message = r#"
    {
        "type": "init",
        "content": "init_message"
    }"#;
    tracing::debug!("init_message - {:?}", init_message);
    window()
        .unwrap()
        .post_message(JsValue::from_str(init_message).as_ref(), "*").unwrap();
    tracing::debug!("init_message - {:?}", init_message);
}

pub fn activate_message_listener() {
    fn has_field(obj: JsValue, key: &str) -> bool {
        Reflect::get(&obj, &JsValue::from_str(key))
            .map(|v| !v.is_undefined())
            .unwrap_or(false)
    }

    let window = window().expect("Cannot get window");
    let closure = Closure::<dyn FnMut(_)>::new(move |event: MessageEvent| {

        match event.data().dyn_into() {
            Ok(obj) => {
                let o: JsValue = obj;
                if has_field(o.clone(), "source") || has_field(o.clone(), "isAngularDevTools") {
                    tracing::trace!("Skipping message from source");
                    return;
                } else {
                    tracing::trace!("Received message from source - {:?}", event.data());
                    load_json_data(DATA_JSON.to_string());
                }
            },
            Err(_) => {
                tracing::error!("Error parsing message from source");
            }
        }
        if event.origin().as_str().contains("http://") {


        }

        match from_value::<InputData>(event.data()) {
            Ok(data) => match data.r#type {
                InputDataType::JSON => load_json_data(data.content)
            },
            Err(_) => {}
        }
    });

    window
        .add_event_listener_with_callback("message", closure.as_ref().unchecked_ref())
        .expect("Failed to add event listener");

    closure.forget();
}

const DATA_JSON: &str = r#"
{
    "data": {
        "text": "Node 0",
        "children": [
            {
                "text": "Node 1",
                "position_direction": "Left",
                "children": [
                    {
                        "text": "Node 1.1",
                        "children": []
                    }
                ]
            },
            {
                "text": "Node 2",
                "children": [
                    {
                        "text": "Node 123NodeNode 123NodeNode 123NodeNode 123NodeNode 123NodeNode 123NodeNode 123NodeNode 123NodeNode 123Node 123Node 123Node 123Node 123Node 123Node 123Node 123Node 123",
                        "children": []
                    },
                    {
                        "text": "Node 2.2",
                        "children": []
                    },
                    {
                        "text": "Node 2.3",
                        "children": []
                    }
                ]
            },
            {
                "text": "Node 3",
                "children": []
            },
            {
                "text": "Node 4",
                "children": []
            },
            {
                "text": "Node 5",
                "children": []
            },
            {
                "text": "Node 6",
                "children": []
            }
        ]
    }
}
"#;