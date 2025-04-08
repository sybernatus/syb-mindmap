use crate::link_renderer::LinkRenderer;
use crate::node_renderer::NodeRenderer;
use crate::POSITION;
use dioxus::logger::tracing;
use dioxus::prelude::*;
use mindy_engine::node::Node;


#[derive(Props, PartialEq, Clone)]
pub struct MindmapProps {
    pub node_list: Vec<Node>,
}

#[component]
pub fn Mindmap(props: MindmapProps) -> Element {
    let mut position = use_signal(|| (0.0, 0.0));

    rsx! {
        div {
            class: "mindmap",
            id: "mindmap",
            style: "transform: translate({POSITION().0}px, {POSITION().1}px);",
            LinkRenderer {
                node_list: props.node_list.clone(),
            }
            NodeRenderer {
                node_list: props.node_list.clone(),
            }
        }
    }
}