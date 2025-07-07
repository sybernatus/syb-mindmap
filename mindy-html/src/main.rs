mod events;
mod link_beziers;
mod link_renderer;
mod listeners;
mod mindmap;
mod node;
mod node_renderer;

use crate::events::mouse::{mouse_data_update, mouse_dragging_disable, mouse_position_update, mouse_zooming_update};
use crate::listeners::webview::{init_message, WebviewListener};
use crate::mindmap::MindmapComp;
use dioxus::prelude::*;
use std::string::ToString;
use dioxus::document::{Script};

const CSS_DATA: &str = include_str!("../assets/main.css");
const MINDMAP_BACKGROUND_DATA: &str = include_str!("../assets/background.svg");
const GITHUB_ICON: &str = include_str!("../assets/ext/github-circle.svg");
const MENU_PICTURE_ICON: &str = include_str!("../assets/ui/picture.svg");
const MINDMAP_ICON: &str = include_str!("../assets/logo/logo.svg");
static SHEET_POSITION: GlobalSignal<(f64, f64)> = GlobalSignal::new(|| (0.0, 0.0));
static SHEET_ZOOM: GlobalSignal<f64> = GlobalSignal::new(|| 1.0);

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
        Script { src: "https://html2canvas.hertzen.com/dist/html2canvas.min.js" }
        document::Style { "{CSS_DATA}" }
        div {
            class: "app",
            id: "app",
            onmousedown: mouse_data_update(is_dragging, last_mouse),
            onmouseup: mouse_dragging_disable(is_dragging),
            onmousemove: mouse_position_update(is_dragging, last_mouse),
            onmouseout: mouse_dragging_disable(is_dragging),
            onwheel: mouse_zooming_update(),
            MindmapComp { }
        }
    }
}
