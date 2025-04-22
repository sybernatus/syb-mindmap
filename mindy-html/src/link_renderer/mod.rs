use crate::link_beziers::{LinkBezierComp, LinkBezierProps};
use crate::MINDMAP_DATA;
use dioxus::logger::tracing;
use dioxus::prelude::*;
use mindy_engine::node::Node;
use mindy_engine::utils::pos2::Pos2;

#[component]
pub fn LinkRendererComp() -> Element {
    let mut elements: Signal<Vec<LinkBezierProps>> = use_signal(|| vec![]);
    let mut mindmap_pos: Signal<Pos2> = use_signal(|| Pos2::default());

    use_effect(move || {
        let mindmap_data = MINDMAP_DATA();

        match mindmap_data {
            Some(mindmap) => {
                let bounding_box = mindmap.get_node_bounding_box();
                mindmap_pos.set(bounding_box.clone().unwrap_or_default().0);
            }
            None => mindmap_pos.set(Pos2::default())
        }
    });

    use_effect(move || {
        let ns = MINDMAP_DATA();
        let offset = mindmap_pos();
        let ns = match ns {
            Some(node) => node,
            None => return,
        };
        elements.clear();
        elements.set(calculate_elements(&ns, None, offset.clone(), vec![]));
    });

    rsx! {
        div {
            class: "link-renderer",
            id: "link-renderer",
            style: "min-width: inherit;",
            style: "min-height: inherit;",
            for element in elements.iter() {
                LinkBezierComp {
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
    node_input: &Node,
    parent_position: Option<Pos2>,
    offset: Pos2,
    mut elements: Vec<LinkBezierProps>,
) -> Vec<LinkBezierProps> {
    if node_input.text.is_none() || node_input.clone().text.unwrap().is_empty() {
        return elements;
    }

    let children = match node_input.children.clone() {
        Some(children) => children,
        None => vec![],
    };

    for child in &children {
        let parent_position = match node_input.position_from_initial.clone() {
            None => return elements,
            Some(pos) => pos,
        };
        elements = calculate_elements(child, Some(parent_position), offset.clone(), elements);
    }

    tracing::trace!("parent_position: {:?}", parent_position);
    let actual_position = match node_input.position_from_initial.clone() {
        None => return elements,
        Some(pos) => pos,
    };

    tracing::trace!("actual_position: {:?}", actual_position);

    let parent_position = match parent_position {
        None => return elements,
        Some(pos) => pos,
    };

    elements.push(LinkBezierProps {
        id: Option::from("link".to_string()),
        pos_start: parent_position.subtract(&offset.clone()),
        pos_end: actual_position.subtract(&offset.clone()),
        color: None,
        stroke_width: None,
    });

    elements
}
