use crate::node::{NodeComp, NodeProps};
use dioxus::logger::tracing;
use dioxus::prelude::*;
use mindy_engine::node::Node;
use mindy_engine::utils::pos2::Pos2;
use crate::mindmap::MINDMAP;

#[component]
pub fn NodeRendererComp() -> Element {

    let mut elements: Signal<Vec<NodeProps>> = use_signal(|| vec![]);

    use_effect(move || {
        let mindmap_position = MINDMAP().position;
        let mindmap_data = MINDMAP().data;
        let elements_new = to_node_props_vec(mindmap_data, &mindmap_position.unwrap_or_default(), vec![]);
        elements.clear();
        elements.set(elements_new);
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


/// Compute a vector of nodes from a parent node moving through all children
fn to_node_props_vec(mindmap_data: Option<Node>, offset: &Pos2, mut elements: Vec<NodeProps>) -> Vec<NodeProps> {
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
        elements = to_node_props_vec(Some(child), offset, elements);
    }

    tracing::debug!("offset: {:?}", offset);
    elements.push(NodeProps {
        node: node_input,
    });

    elements
}
