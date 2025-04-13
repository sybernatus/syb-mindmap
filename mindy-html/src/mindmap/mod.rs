use crate::link_renderer::LinkRendererComp;
use crate::node_renderer::NodeRendererComp;
use crate::SHEET_POSITION;
use dioxus::prelude::*;

#[component]
pub fn MindmapComp() -> Element {
    rsx! {
        div {
            class: "mindmap",
            id: "mindmap",
            style: "width: inherit;",
            style: "height: inherit;",
            div {
                class: "floating-menu",
                button {
                    onclick: move |_| {
                        *SHEET_POSITION.write() = (0.0, 0.0);
                    },
                    "Centered"
                }
            }
            div {
                style: "transform: translate({SHEET_POSITION().0}px, {SHEET_POSITION().1}px);",
                style: "width: 8000px;",
                style: "height: inherit;",
                LinkRendererComp { }
                NodeRendererComp { }
            }
        }

    }
}
