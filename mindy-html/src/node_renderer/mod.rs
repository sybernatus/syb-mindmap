use dioxus::logger::tracing;
use dioxus::prelude::*;
use mindy_engine::node_input::NodeInput;
use crate::node::{Node, NodeProps};
use crate::{NODE_LIST_NEW};

#[component]
pub fn NodeRenderer() -> Element {
    let mut elements: Signal<Vec<NodeProps>> = use_signal(|| vec![]);

    let _ = use_effect(move || {
        let ns = NODE_LIST_NEW();
        elements.clear();
        tracing::debug!("NodeRenderer: {:?}", ns);
        elements.set(calculate_elements(ns, vec![]));
    });

    rsx! {
        div {
            class: "node-renderer",
            id: "node-renderer",
            for element in elements.iter() {
                Node {
                    node: element.node.clone(),
                }
            }
        }
    }

}

fn calculate_elements(
    node_input: Option<NodeInput>,
    mut elements: Vec<NodeProps>,
) -> Vec<NodeProps> {

    let node_input = match node_input {
        Some(node) => node,
        None => return elements,
    };

    for child in node_input.clone().children.unwrap() {
        elements = calculate_elements(Some(child), elements);
    }

    elements.push(
        NodeProps {
            node: node_input.clone(),
        }
    );

    elements
}