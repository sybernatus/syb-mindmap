use crate::events::webview::WebviewEvent;
use ::web_sys::window;
use dioxus::logger::tracing;
use mindy_engine::mindmap::Mindmap;
use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::from_value;
use web_sys::wasm_bindgen::closure::Closure;
use web_sys::wasm_bindgen::{JsCast, JsValue};
use web_sys::MessageEvent;
use mindy_engine::utils::throttler::Throttler;
use crate::mindmap::update_mindmap;

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

    /// Create a new WebviewListener to capture the message from the webview queue
    pub fn add_message_listener(&self) {
        let window = window().expect("Cannot get window");
        let throttler = Throttler::new(|webview_listener: WebviewListener|{
            match webview_listener.r#type {
                WebviewMessageType::JSON => {
                    tracing::debug!("MessageEvent JSON - {:?}", webview_listener.content);
                    match Mindmap::from_json_string(webview_listener.content) {
                        Ok(mindmap) => update_mindmap(mindmap),
                        Err(_) => {
                            return;
                        }
                    }
                }
                WebviewMessageType::YAML => {
                    tracing::debug!("MessageEvent YAML - {:?}", webview_listener.content);
                    match Mindmap::from_yaml_string(webview_listener.content) {
                        Ok(mindmap) => update_mindmap(mindmap),
                        Err(_) => {
                            return;
                        }
                    }
                }
            }
        }, 300);

        let closure = Closure::<dyn FnMut(_)>::new(move |event: MessageEvent| {

            let webview_event = WebviewEvent::new(event);

            if webview_event.is_origin_http() && !webview_event.is_origin_intellij() {
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
                            match Mindmap::from_yaml_string(DATA_JSON.to_string()) {
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
                    Ok(webview_listener) => {
                        throttler.send(webview_listener);
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
data:
  text: GCP
  children:
  - text: Databases
    children:
      - text: Big Query
      - text: Data Table
      - text: Data Table
      - text: Data Tableeeeeee     aazeazeeeee
        custom_style:
          background_color:
            red: 0
            green: 150
            blue: 0
      - text: Data Table


"#;
