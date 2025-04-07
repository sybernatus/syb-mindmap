mod node;

use dioxus::prelude::*;
use mindy_engine::node::Pos2;
use mindy_engine::node::{Node as NodeCore};
use crate::node::Node;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Node {
            id: "test",
            class: "node",
            node: NodeCore::new()
                .with_text("Hello, World!".to_string())
                .with_position(Pos2::new(300.0, 100.0))
                .with_color(122.0, 10.0, 0.0)

        }
    }
}
