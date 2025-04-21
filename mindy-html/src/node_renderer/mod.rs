use crate::node::{NodeComp, NodeProps};
use crate::MINDMAP_DATA;
use dioxus::logger::tracing;
use dioxus::prelude::*;
use mindy_engine::node::Node;

#[component]
pub fn NodeRendererComp() -> Element {
    let mut elements: Signal<Vec<NodeProps>> = use_signal(|| vec![]);

    use_effect(move || {
        let mindmap_data = MINDMAP_DATA();
        tracing::trace!("NodeRenderer: {:?}", mindmap_data);
        elements.set(calculate_elements(mindmap_data, vec![]));
    });

    rsx! {
        div {
            class: "node-renderer",
            id: "node-renderer",
            style: "min-width: inherit;",
            style: "min-height: inherit;",
            for element in elements.iter() {
                NodeComp {
                    node: element.node.clone(),
                }
            }
        }
    }
}

fn calculate_elements(mindmap_data: Option<Node>, mut elements: Vec<NodeProps>) -> Vec<NodeProps> {
    let node_input = match mindmap_data {
        Some(node) => node,
        None => return elements,
    };

    if node_input.text.is_none() || node_input.clone().text.unwrap().is_empty() {
        return elements;
    }

    let children = match node_input.to_owned().children {
        Some(children) => children,
        None => vec![],
    };

    for child in children {
        elements = calculate_elements(Some(child), elements);
    }

    elements.push(NodeProps {
        node: node_input.clone(),
    });

    elements
}
