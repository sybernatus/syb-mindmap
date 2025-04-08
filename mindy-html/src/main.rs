mod node;
mod mindmap;
mod link_beziers;
mod link_renderer;
mod node_renderer;

use std::collections::HashSet;
use dioxus::logger::tracing;
use dioxus::prelude::*;
use mindy_engine::node::Pos2;
use mindy_engine::node::{Node as NodeCore};
use crate::mindmap::Mindmap;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const MINDMAP_BACKGROUND: Asset = asset!("/assets/background.svg");
static SHEET_POSITION: GlobalSignal<(f64, f64)> = GlobalSignal::new(||(0.0, 0.0));
static NODE_LIST: GlobalSignal<Vec<NodeCore>> = GlobalSignal::new(|| vec![]);

fn main() {
    launch(App);
}

#[component]
fn App() -> Element {
    let position = use_context_provider(|| (0.0, 0.0));
    let mut is_dragging = use_signal(|| false);
    let mut last_mouse = use_signal(|| (0.0, 0.0));

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

    let node_list = vec![
        NodeCore::new()
            .with_text("Hello, World!".to_string())
            .with_position(Pos2::new(100.0, 100.0)),
        NodeCore::new()
            .with_text("Hello, World!".to_string())
            .with_position(Pos2::new(300.0, 300.0))
            .with_id(1)
            .set_parent(0),
        NodeCore::new()
            .with_text("Hello, World!".to_string())
            .with_position(Pos2::new(360.0, 150.0))
            .with_id(2)
            .set_parent(0),
    ];

    for node in node_list.iter() {
        NODE_LIST.write().push(node.clone());
    }

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
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
            background-image: url({MINDMAP_BACKGROUND}); \
            background-repeat: repeat;",
            onmousedown: update_mouse_data,
            onmouseup: disable_dragging,
            onmousemove: position_update,
            onmouseout: disable_dragging,
            Mindmap {
                node_list
            }
        }
    }
}
