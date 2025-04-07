mod node;
mod mindmap;
mod link_beziers;
mod link_renderer;
mod node_renderer;

use dioxus::prelude::*;
use mindy_engine::node::Pos2;
use mindy_engine::node::{Node as NodeCore};
use crate::mindmap::Mindmap;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

fn main() {
    launch(App);
}

#[component]
fn App() -> Element {

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

    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Mindmap {
            node_list
        }
    }
}
