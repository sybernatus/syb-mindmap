use dioxus::prelude::*;
use mindy_engine::node::{Node};
use crate::node::{Node, NodeProps};

#[derive(Props, PartialEq, Clone)]
pub struct NodeRendererProps {
    pub node_list: Vec<Node>,
}

#[component]
pub fn NodeRenderer(props: NodeRendererProps) -> Element {
    let mut elements: Vec<NodeProps> = vec![];
    for node in props.node_list.iter() {
        elements.push(
            NodeProps {
                id: "node".to_string(),
                class: "node".to_string(),
                node: node.clone(),
            }
        );
    }
    rsx! {
        div {
            class: "node-renderer",
            id: "node-renderer",
            for element in elements.iter() {
                Node {
                    id: element.id.clone(),
                    class: element.class.clone(),
                    node: element.node.clone(),
                }
            }
        }
    }

}