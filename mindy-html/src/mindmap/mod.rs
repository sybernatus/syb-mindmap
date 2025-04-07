use crate::link_renderer::LinkRenderer;
use crate::node_renderer::NodeRenderer;
use dioxus::prelude::*;
use mindy_engine::node::Node;

#[derive(Props, PartialEq, Clone)]
pub struct MindmapProps {
    pub node_list: Vec<Node>,
}

#[component]
pub fn Mindmap(props: MindmapProps) -> Element {
    rsx! {
        div {
            class: "mindmap",
            id: "mindmap",
            LinkRenderer {
                node_list: props.node_list.clone(),
            }
            NodeRenderer {
                node_list: props.node_list.clone(),
            }
        }
    }
}