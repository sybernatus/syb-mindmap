use crate::link_renderer::LinkRendererComp;
use crate::node_renderer::{NodeRendererComp};
use crate::{MINDMAP_BACKGROUND_DATA, MINDMAP_DATA, SHEET_POSITION};
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use dioxus::prelude::*;
use mindy_engine::utils::pos2::Pos2;
use mindy_engine::utils::size::Size;


#[derive(Clone, Copy)]
pub struct MindmapState {
    pub mindmap_bounding_box_position: Signal<Pos2>
}

#[component]
pub fn MindmapComp() -> Element {
    let mut mindmap_size: Signal<Size> = use_signal(|| Size::default());
    let mut mindmap_position: Signal<Pos2> = use_signal(|| Pos2::default());
    let mut state = use_context_provider(|| MindmapState {
        mindmap_bounding_box_position: Signal::new(Pos2::default())
    });

    use_effect(move || {
        let mindmap_data = MINDMAP_DATA();

        match mindmap_data {
            Some(mindmap) => {
                let bounding_box = mindmap.get_node_bounding_box();
                state.mindmap_bounding_box_position.set(bounding_box.clone().unwrap_or_default().0);
            }
            None => state.mindmap_bounding_box_position.set(Pos2::default())
        }
    });

    use_effect(move || {
        let mindmap_data = MINDMAP_DATA();

        match mindmap_data {
            Some(mindmap) => {
                let size = mindmap.get_node_bounding_box();
                mindmap_position.set(size.clone().unwrap_or_default().0);
                mindmap_size.set(size.clone().unwrap_or_default().1);
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
                style: "width: {mindmap_size().width - mindmap_position().x}px;",
                style: "height: {mindmap_size().height - mindmap_position().y}px;",
                LinkRendererComp { }
                NodeRendererComp { }
            }
        }

    }
}
