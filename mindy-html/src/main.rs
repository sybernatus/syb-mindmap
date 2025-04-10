mod node;
mod mindmap;
mod link_beziers;
mod link_renderer;
mod node_renderer;

use std::cell::RefCell;
use crate::mindmap::Mindmap;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use dioxus::html::a::widows;
use dioxus::logger::tracing;
use dioxus::prelude::*;
use mindy_engine::node::Pos2;
use mindy_engine::node::Node as NodeCore;
use std::string::ToString;
use ::web_sys::window;
use dioxus::html::completions::CompleteWithBraces::data;
use serde_json::Value;
use web_sys::MessageEvent;
use web_sys::wasm_bindgen::closure::Closure;
use web_sys::wasm_bindgen::{JsCast, JsValue};
use serde::Deserialize;
use mindy_engine::node_input::NodeInput;

const CSS_DATA: &str = include_str!("../assets/main.css");
const MINDMAP_BACKGROUND_DATA: &str = include_str!("../assets/background.svg");
static SHEET_POSITION: GlobalSignal<(f64, f64)> = GlobalSignal::new(||(0.0, 0.0));
static NODE_LIST: GlobalSignal<Vec<NodeCore>> = GlobalSignal::new(|| vec![]);

#[derive(Deserialize, Debug)]
struct TestData {
    data: String,
}

fn main() {
    launch(App);
}

#[component]
fn App() -> Element {
    let is_dragging = use_signal(|| false);
    let last_mouse = use_signal(|| (0.0, 0.0));
    load_json_data();
    activate_message_listener();

    // let node_list = vec![
    //     NodeCore::new()
    //         .with_text("Hello, World!".to_string())
    //         .with_position(Pos2::new(100.0, 100.0)),
    //     NodeCore::new()
    //         .with_text("Hello, World!".to_string())
    //         .with_position(Pos2::new(300.0, 300.0))
    //         .with_id(1)
    //         .set_parent(0),
    //     NodeCore::new()
    //         .with_text("Hello, World!".to_string())
    //         .with_position(Pos2::new(360.0, 150.0))
    //         .with_id(2)
    //         .set_parent(0),
    // ];

    // for node in node_list.iter() {
    //     NODE_LIST.write().push(node.clone());
    // }

    rsx! {
        // document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Style { "{CSS_DATA}" }
        button {
            onclick: move |_| {
                *SHEET_POSITION.write() = (0.0, 0.0);
            },
            "Centered"
        }
        div {
            class: "app",
            id: "app",
            style: "\
            background-image: url(data:image/svg+xml;base64,{STANDARD.encode(MINDMAP_BACKGROUND_DATA.to_string())}); \
            background-repeat: repeat;",
            onmousedown: mouse_data_update(is_dragging, last_mouse),
            onmouseup: mouse_dragging_disable(is_dragging),
            onmousemove: mouse_position_update(is_dragging, last_mouse),
            onmouseout: mouse_dragging_disable(is_dragging),
            Mindmap { }
        }
    }
}

fn mouse_data_update(mut is_dragging: Signal<bool>, mut last_mouse: Signal<(f64, f64)>) -> impl Fn(Event<MouseData>) {
    move |event: Event<MouseData>| {
        use_future(move || {
            let value = event.clone();
            async move {
                tracing::trace!("Mouse down event: {:?}", value);
                is_dragging.set(true);
                last_mouse.set((value.data().coordinates().client().x, value.data().coordinates().client().y));
                tracing::trace!("Mouse down position: {:?}", last_mouse);
            }
        });
    }
}

fn mouse_position_update(mut is_dragging: Signal<bool>, mut last_mouse: Signal<(f64, f64)>) -> impl Fn(Event<MouseData>) {
    move |event: Event<MouseData>| {
        use_future(move || {
            let value = event.clone();
            async move {
                if is_dragging() {
                    let current_mouse = (value.data.coordinates().client().x, value.data.coordinates().client().y);
                    *SHEET_POSITION.write() = (
                        SHEET_POSITION().0 + current_mouse.0 - last_mouse().0,
                        SHEET_POSITION().1 + current_mouse.1 - last_mouse().1,
                    );
                    last_mouse.set(current_mouse);
                }
            }
        });
    }
}

fn mouse_dragging_disable(mut is_dragging: Signal<bool>) -> impl Fn(Event<MouseData>) {
    move |event: Event<MouseData>| {
        use_future(move || {
            async move {
                is_dragging.set(false);
            }
        });
    }
}

fn load_json_data() {
    let data_json = r#"
        {
            "text": "Node 1",
            "children": [
                {
                    "text": "Node 2",
                    "children": []
                },
                {
                    "text": "Node 3",
                    "children": []
                },
                {
                    "text": "Node 3",
                    "children": []
                },
                {
                    "text": "Node 3",
                    "children": []
                },
                {
                    "text": "Node 3",
                    "children": []
                },
                {
                    "text": "Node 3",
                    "children": []
                }
            ]
        }
        "#;
    let json: NodeInput = match serde_json::from_str::<NodeInput>(data_json) {
        Ok(json) => json,
        Err(e) => {
            tracing::error!("Error decoding json: {:?}", e);
            return;
        }
    };



    if ! json.text.is_none() {
        let node_list = json.to_node_vec(None, &RefCell::new(0), Pos2::new(300.0, 300.0), 0, 0, &mut vec![]);
        NODE_LIST.write().clear();
        for node in node_list.iter() {
            NODE_LIST.write().push(node.clone());
        }
    }
}

fn activate_message_listener() {
    let window = window().expect("Cannot get window");
    let closure = Closure::<dyn FnMut(_)>::new(move |event: MessageEvent| {
        // Lecture des données du message
        let data = event.data().as_string().or(Some("null".to_string()));

        let json: NodeInput = match serde_json::from_str::<NodeInput>(&data.unwrap()) {
            Ok(json) => json,
            Err(e) => {
                return;
            }
        };

        tracing::debug!("CLOSURE ACTIVATION !!!!!!!!!!!!!!!!!!!!!!!! {:?}", event.data());
        load_json_data();

    });
    window.add_event_listener_with_callback("message", closure.as_ref().unchecked_ref()).expect("Failed to add event listener");

    closure.forget();
}