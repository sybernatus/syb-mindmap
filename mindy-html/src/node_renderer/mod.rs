use dioxus::logger::tracing;
use dioxus::prelude::*;
use mindy_engine::node::{Node};
use crate::node::{Node, NodeProps};
use crate::NODE_LIST;

#[component]
pub fn NodeRenderer() -> Element {
    let mut elements: Signal<Vec<NodeProps>> = use_signal(|| vec![]);

    let _ = use_effect(move || {
        let ns = NODE_LIST();
        tracing::debug!("Node list: {:?}", ns);
        elements.clear();
        elements.set(calculate_elements(ns));
    });

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

fn calculate_elements(
    node_list: Vec<Node>,
) -> Vec<NodeProps> {
    let mut elements: Vec<NodeProps> = vec![];
    for node in node_list.iter() {
        // tracing::debug!("Node ID: {:?}", node.id);
        let parent_id = node.parent_id;
        // tracing::debug!("Node parent_id: {:?}", parent_id);
        let parent_node = node_list.iter().find(|n| Some(n.id) == parent_id);

        // tracing::debug!("Node: {:?}", parent_node);
        let parent_hidden_children = match parent_node {
            Some(parent) => parent.style_custom.children_hidden,
            None => false,
        };
        // tracing::debug!("Parent hidden children: {:?}", parent_hidden_children);

        if parent_hidden_children {
            continue;
        }

        elements.push(
            NodeProps {
                id: "node".to_string(),
                class: "node".to_string(),
                node: node.clone(),
            }
        );
    }

    tracing::debug!("Elements: {:?}", elements.len());
    elements
}