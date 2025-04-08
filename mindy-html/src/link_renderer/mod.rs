use dioxus::logger::tracing;
use dioxus::prelude::*;
use palette::num::PartialCmp;
use mindy_engine::node::{Node};
use crate::link_beziers::{LinkBezier, LinkBezierProps};
use crate::node::NodeProps;
use crate::NODE_LIST;

#[component]
pub fn LinkRenderer() -> Element {
    let mut elements: Signal<Vec<LinkBezierProps>> = use_signal(|| vec![]);

    use_effect(move || {
        let ns = NODE_LIST();
        elements.clear();
        elements.set(calculate_elements(ns));
    });

    rsx! {
        div {
            class: "link-renderer",
            id: "link-renderer",
            for element in elements.iter() {
                LinkBezier {
                    id: element.id.clone(),
                    pos_start: element.pos_start.clone(),
                    pos_end: element.pos_end.clone(),
                    color: element.color.clone(),
                    stroke_width: element.stroke_width.clone(),
                }
            }
        }
    }
}

fn calculate_elements(
    node_list: Vec<Node>,
) -> Vec<LinkBezierProps> {
    let mut elements: Vec<LinkBezierProps> = vec![];
    for node in node_list.iter() {
        if node.parent_id.is_some() {
            let parent_id = node.parent_id;
            let parent_node = node_list.iter().find(|n| Some(n.id) == parent_id);
            let parent_position = parent_node.unwrap().position.clone();
            let actual_position = node.position.clone();
            let parent_hidden_children = match parent_node {
                Some(parent) => parent.style_custom.children_hidden,
                None => false,
            };

            if parent_hidden_children {
                continue;
            }

            elements.push(
                LinkBezierProps {
                    id: Option::from("link".to_string()),
                    pos_start: parent_position,
                    pos_end: actual_position,
                    color: None,
                    stroke_width: None,
                }
            );
        }
    };
    elements
}