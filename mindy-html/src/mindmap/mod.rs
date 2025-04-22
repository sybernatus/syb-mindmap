use crate::link_renderer::LinkRendererComp;
use crate::node_renderer::NodeRendererComp;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use crate::{MINDMAP_DATA, SHEET_POSITION, MINDMAP_BACKGROUND_DATA};
use dioxus::prelude::*;
use mindy_engine::utils::size::Size;

#[component]
pub fn MindmapComp() -> Element {
    let mut mindmap_size: Signal<Size> = use_signal(|| Size::default());

    use_effect(move || {
        let mindmap_data = MINDMAP_DATA();

        match mindmap_data {
            Some(mindmap) => {
                let size = mindmap.get_node_bounding_box();
                mindmap_size.set(size.unwrap_or_default().1);
            }
            None => mindmap_size.set(Size::default()),
        }
    });
    rsx! {
        div {
            class: "mindmap",
            id: "mindmap",
            style: "width: inherit;",
            style: "height: inherit;",
            style: "\
                background-image: url(data:image/svg+xml;base64,{STANDARD.encode(MINDMAP_BACKGROUND_DATA.to_string())}); \
                background-repeat: repeat;",
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
                class: "mindmap-background",
                style: "transform: translate({SHEET_POSITION().0}px, {SHEET_POSITION().1}px);",
                style: "min-width: {mindmap_size().width}px;",
                style: "min-height: {mindmap_size().height}px;",
                LinkRendererComp { }
                NodeRendererComp { }
            }
        }

    }
}
