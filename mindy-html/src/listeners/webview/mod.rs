use crate::load_json_data;
use dioxus::logger::tracing;
use mindy_engine::node_input::NodeInput;
use web_sys::wasm_bindgen::closure::Closure;
use web_sys::wasm_bindgen::{JsCast};
use ::web_sys::window;
use web_sys::MessageEvent;

pub fn activate_message_listener() {
    let window = window().expect("Cannot get window");
    let closure = Closure::<dyn FnMut(_)>::new(move |event: MessageEvent| {
        // Lecture des données du message
        let data = event.data().as_string().or(Some("null".to_string()));

        let _json: NodeInput = match serde_json::from_str::<NodeInput>(&data.unwrap()) {
            Ok(json) => json,
            Err(_e) => {
                return;
            }
        };

        tracing::trace!("CLOSURE ACTIVATION !!!!!!!!!!!!!!!!!!!!!!!! {:?}", event.data());
        load_json_data();

    });
    window.add_event_listener_with_callback("message", closure.as_ref().unchecked_ref()).expect("Failed to add event listener");

    closure.forget();
}