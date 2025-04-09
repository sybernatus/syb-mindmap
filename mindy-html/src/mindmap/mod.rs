use crate::link_renderer::LinkRenderer;
use crate::node_renderer::NodeRenderer;
use crate::SHEET_POSITION;
use dioxus::prelude::*;

#[component]
pub fn Mindmap() -> Element {

    rsx! {
        div {
            class: "mindmap",
            id: "mindmap",
            style: "transform: translate({SHEET_POSITION().0}px, {SHEET_POSITION().1}px);",
            LinkRenderer { }
            NodeRenderer { }
        }
    }
}