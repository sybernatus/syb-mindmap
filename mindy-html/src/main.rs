mod events;
mod link_beziers;
mod link_renderer;
mod listeners;
mod mindmap;
mod node;
mod node_renderer;

use crate::events::mouse::{mouse_data_update, mouse_dragging_disable, mouse_position_update};
use crate::listeners::webview::{activate_message_listener};
use crate::mindmap::MindmapComp;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use dioxus::logger::tracing;
use dioxus::prelude::*;
use mindy_engine::mindmap::metadata::MindmapMetadata;
use mindy_engine::mindmap::{Mindmap as MindmapCore};
use mindy_engine::node::Node;
use std::string::ToString;

const CSS_DATA: &str = include_str!("../assets/main.css");
const MINDMAP_BACKGROUND_DATA: &str = include_str!("../assets/background.svg");
static SHEET_POSITION: GlobalSignal<(f64, f64)> = GlobalSignal::new(|| (0.0, 0.0));
static MINDMAP_METADATA: GlobalSignal<Option<MindmapMetadata>> = GlobalSignal::new(|| None);
static MINDMAP_DATA: GlobalSignal<Option<Node>> = GlobalSignal::new(|| None);

fn main() {
    launch(App);
}

#[component]
fn App() -> Element {
    let is_dragging = use_signal(|| false);
    let last_mouse = use_signal(|| (0.0, 0.0));
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
            MindmapComp { }
        }
    }
}

fn load_json_data(data_json: String) {
    tracing::debug!("load_json_data - {:?}", data_json);
    let input_data = match serde_json::from_str::<MindmapCore>(data_json.as_str()) {
        Ok(mut input_data) => {
            tracing::debug!("load_json_data - {:?}", input_data);
            let metadata = input_data
                .clone()
                .metadata
                .unwrap_or_else(|| MindmapMetadata::default());
            input_data.layout_mindmap();
            let json = input_data.data.unwrap_or_else(|| Node::default());
            MindmapCore {
                metadata: Some(metadata),
                data: Some(json),
            }
        }
        Err(e) => {
            tracing::error!("Error decoding json: {:?}", e);
            return;
        }
    };

    tracing::trace!("load_json_data - {:?}", input_data);
    *MINDMAP_DATA.write() = input_data.data;
    *MINDMAP_METADATA.write() = input_data.metadata;
}
