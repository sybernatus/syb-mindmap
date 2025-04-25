mod events;
mod link_beziers;
mod link_renderer;
mod listeners;
mod mindmap;
mod node;
mod node_renderer;

use crate::events::mouse::{mouse_data_update, mouse_dragging_disable, mouse_position_update};
use crate::listeners::webview::{init_message, WebviewListener};
use crate::mindmap::MindmapComp;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use dioxus::prelude::*;
use std::string::ToString;

const CSS_DATA: &str = include_str!("../assets/main.css");
const MINDMAP_BACKGROUND_DATA: &str = include_str!("../assets/background.svg");
const MINDMAP_ICON: &str = include_str!("../assets/logo/logo.svg");
static SHEET_POSITION: GlobalSignal<(f64, f64)> = GlobalSignal::new(|| (0.0, 0.0));

fn main() {
    launch(App);
}

#[component]
fn App() -> Element {
    let is_dragging = use_signal(|| false);
    let last_mouse = use_signal(|| (0.0, 0.0));
    WebviewListener::new().add_message_listener();
    init_message();

    rsx! {
        // document::Link { rel: "stylesheet", href: MAIN_CSS }
        document::Style { "{CSS_DATA}" }
        header {
            class: "banner",
            img {
                class: "logo",
                src: "data:image/svg+xml;base64,{STANDARD.encode(MINDMAP_ICON.to_string())}",
            }
            h1 {
                class: "title",
                "Mindmap"
            }
        }
        div {
            class: "app",
            id: "app",
            onmousedown: mouse_data_update(is_dragging, last_mouse),
            onmouseup: mouse_dragging_disable(is_dragging),
            onmousemove: mouse_position_update(is_dragging, last_mouse),
            onmouseout: mouse_dragging_disable(is_dragging),
            MindmapComp { }
        }
    }
}
