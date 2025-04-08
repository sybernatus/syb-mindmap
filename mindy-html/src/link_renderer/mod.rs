use dioxus::prelude::*;
use mindy_engine::node::{Node};
use crate::link_beziers::{LinkBezier, LinkBezierProps};

#[derive(Props, PartialEq, Clone)]
pub struct LinkRendererProps {
    pub node_list: Vec<Node>,
}

#[component]
pub fn LinkRenderer(props: LinkRendererProps) -> Element {
    let mut elements: Vec<LinkBezierProps> = vec![];
    for node in props.node_list.iter() {
        if node.parent_id.is_some() {
            let parent_id = 0;
            let parent_node = props.node_list.iter().find(|n| n.id == parent_id);
            let parent_position = parent_node.unwrap().position.clone();
            let actual_position = node.position.clone();
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