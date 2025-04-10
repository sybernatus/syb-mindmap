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
    let mut is_dragging = use_signal(|| false);
    let mut last_mouse = use_signal(|| (0.0, 0.0));
    load_json_data();

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


    let update_mouse_data = move |event: Event<MouseData>| {
        tracing::trace!("Mouse down event: {:?}", event);
        is_dragging.set(true);
        last_mouse.set((event.data().coordinates().client().x, event.data().coordinates().client().y));
        tracing::trace!("Mouse down position: {:?}", last_mouse);
    };

    let disable_dragging = move |_event: Event<MouseData>| {
        is_dragging.set(false);
    };

    let position_update = move |event: Event<MouseData>| {
        if is_dragging() {
            let current_mouse = (event.data.coordinates().client().x, event.data.coordinates().client().y);
            *SHEET_POSITION.write() = (
                SHEET_POSITION().0 + current_mouse.0 - last_mouse().0,
                SHEET_POSITION().1 + current_mouse.1 - last_mouse().1,
            );
            last_mouse.set(current_mouse);
        }
    };

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
            onmousedown: update_mouse_data,
            onmouseup: disable_dragging,
            onmousemove: position_update,
            onmouseout: disable_dragging,
            Mindmap { }
        }
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