use crate::events::webview::WebviewEvent;
use crate::update_mindmap;
use ::web_sys::window;
use dioxus::logger::tracing;
use mindy_engine::mindmap::Mindmap;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::from_value;
use web_sys::wasm_bindgen::closure::Closure;
use web_sys::wasm_bindgen::{JsCast, JsValue};
use web_sys::MessageEvent;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum WebviewMessageType {
    JSON,
    YAML,
}

impl Default for WebviewMessageType {
    fn default() -> Self {
        Self::JSON
    }
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct WebviewListener {
    pub r#type: WebviewMessageType,
    pub content: String,
}

impl WebviewListener {
    pub fn new() -> Self {
        Self {
            r#type: Default::default(),
            content: "".to_string(),
        }
    }

    pub fn add_message_listener(&self) {
        let window = window().expect("Cannot get window");
        let closure = Closure::<dyn FnMut(_)>::new(move |event: MessageEvent| {
            let webview_event = WebviewEvent::new(event);

            if webview_event.is_origin_http() {
                match webview_event.get_data().dyn_into() {
                    Ok(dynamic_object) => {
                        let js_value_object: JsValue = dynamic_object;
                        if webview_event.data_has_one_of_fields(
                            js_value_object,
                            vec!["isAngularDevTools", "source"],
                        ) {
                            tracing::trace!("Skipping message from source");
                        } else {
                            tracing::trace!(
                                "Received message from source - {:?}",
                                webview_event.get_data()
                            );
                            match Mindmap::from_json_string(DATA_JSON.to_string()) {
                                Ok(mindmap) => update_mindmap(mindmap),
                                Err(_) => {
                                    return;
                                }
                            };
                        }
                    }
                    Err(e) => {
                        tracing::error!("Error parsing message from source - {:?}", e);
                    }
                };
            } else {
                match from_value::<WebviewListener>(webview_event.get_data()) {
                    Ok(webview_listener) => match webview_listener.r#type {
                        WebviewMessageType::JSON => {
                            match Mindmap::from_json_string(webview_listener.content) {
                                Ok(mindmap) => update_mindmap(mindmap),
                                Err(_) => {
                                    return;
                                }
                            }
                        }
                        WebviewMessageType::YAML => {
                            match Mindmap::from_yaml_string(webview_listener.content) {
                                Ok(mindmap) => update_mindmap(mindmap),
                                Err(_) => {
                                    return;
                                }
                            }
                        }
                    },
                    Err(err) => {
                        tracing::error!("Error parsing message from source - {:?}", err);
                    }
                };
            };
        });

        window
            .add_event_listener_with_callback("message", closure.as_ref().unchecked_ref())
            .expect("Failed to add event listener");

        closure.forget();
    }
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
        .post_message(JsValue::from_str(init_message).as_ref(), "*")
        .unwrap();
    tracing::debug!("init_message - {:?}", init_message);
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
                        "text": "Node 2.2\natata",
                        "children": []
                    },
                    {
                        "text": "Node 2.3"
                    }
                ]
            },
            {
                "text": "<p>Test</p>",
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
