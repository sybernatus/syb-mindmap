use crate::link_renderer::LinkRendererComp;
use crate::node_renderer::{NodeRendererComp};
use crate::{MINDMAP_BACKGROUND_DATA, SHEET_POSITION};
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use dioxus::logger::tracing;
use dioxus::prelude::*;
use mindy_engine::mindmap::Mindmap;
use mindy_engine::layout::pos2::Pos2;
use mindy_engine::layout::size::Size;

pub(crate) static MINDMAP: GlobalSignal<Mindmap> = GlobalSignal::new(|| Mindmap::default());

#[component]
pub fn MindmapComp() -> Element {
    let mut mindmap_size: Signal<Size> = use_signal(|| Size::default());
    let mut mindmap_position: Signal<Pos2> = use_signal(|| Pos2::default());

    use_effect(move || {
        let mindmap = MINDMAP();
        tracing::debug!("mindmap: position: {:?}", mindmap.position);
        tracing::debug!("mindmap: size: {:?}", mindmap.size);

        match mindmap.position {
            Some(position) => mindmap_position.set(position),
            None => mindmap_position.set(Pos2::default())
        }

        match mindmap.size {
            Some(size) => mindmap_size.set(size),
            None => mindmap_size.set(Size::default())
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

/// Update the global state of the mindmap
pub fn update_mindmap(mut mindmap: Mindmap) {
    *MINDMAP.write() = mindmap
        .compute_all()
        .clone();
}
