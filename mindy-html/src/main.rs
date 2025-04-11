mod node;
mod mindmap;
mod link_beziers;
mod link_renderer;
mod node_renderer;
mod events;
mod listeners;

use std::cell::RefCell;
use crate::mindmap::Mindmap;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use dioxus::logger::tracing;
use dioxus::prelude::*;
use std::string::ToString;
use mindy_engine::node::Node;
use crate::events::mouse::{mouse_data_update, mouse_dragging_disable, mouse_position_update};
use crate::listeners::webview::activate_message_listener;

const CSS_DATA: &str = include_str!("../assets/main.css");
const MINDMAP_BACKGROUND_DATA: &str = include_str!("../assets/background.svg");
static SHEET_POSITION: GlobalSignal<(f64, f64)> = GlobalSignal::new(||(0.0, 0.0));
static NODE_LIST_NEW: GlobalSignal<Option<Node>> = GlobalSignal::new(|| None);

fn main() {
    launch(App);
}

#[component]
fn App() -> Element {
    let is_dragging = use_signal(|| false);
    let last_mouse = use_signal(|| (0.0, 0.0));
    load_json_data();
    activate_message_listener();

    rsx! {
        // document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Style { "{CSS_DATA}" }
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

fn load_json_data() {
    let data_json = r#"
        {
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
        "#;
    let mut node_input: Node = match serde_json::from_str::<Node>(data_json) {
        Ok(mut json) => {
            json.layout_mindmap_center();
            // tracing::debug!("node: {:?}", json);
            json
        },
        Err(e) => {
            tracing::error!("Error decoding json: {:?}", e);
            return;
        }
    };

    *NODE_LIST_NEW.write() = Some(node_input.clone());


    // if ! json.text.is_none() {
    //     let node_list = json.to_node_vec(
    //         None, &RefCell::new(0),
    //         Pos2::new(100.0, 100.0),
    //         0,
    //         0,
    //         &mut vec![]
    //     );
    //     NODE_LIST.write().clear();
    //     for node in node_list.iter() {
    //         NODE_LIST.write().push(node.clone());
    //     }
    // }
}