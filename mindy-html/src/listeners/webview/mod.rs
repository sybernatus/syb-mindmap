use crate::load_json_data;
use ::web_sys::window;
use dioxus::logger::tracing;
use serde::{Deserialize, Serialize};
use web_sys::wasm_bindgen::closure::Closure;
use web_sys::wasm_bindgen::{JsCast};
use web_sys::MessageEvent;
use serde_wasm_bindgen::from_value;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct InputData {
    pub r#type: InputDataType,
    pub content: String
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum InputDataType {
    JSON,
}

pub fn activate_message_listener() {
    let window = window().expect("Cannot get window");
    let closure = Closure::<dyn FnMut(_)>::new(move |event: MessageEvent| {

        tracing::debug!("Received message from webview {:?}", event.data());
        let _data = match event.origin().as_str() {
            origin if origin.contains("http://") => load_json_data(DATA_JSON.to_string()),
            _ => {}
        };

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
    "metadata": {
        "diagram_type": "Standard"
    },
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