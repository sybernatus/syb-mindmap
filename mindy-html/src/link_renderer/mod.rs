use dioxus::logger::tracing;
use dioxus::prelude::*;
use mindy_engine::node_input::{NodeInput, Pos2};
use crate::link_beziers::{LinkBezier, LinkBezierProps};
use crate::{NODE_LIST_NEW};

#[component]
pub fn LinkRenderer() -> Element {
    let mut elements: Signal<Vec<LinkBezierProps>> = use_signal(|| vec![]);

    use_effect(move || {
        let ns = NODE_LIST_NEW();
        tracing::debug!("LinkRenderer: {:?}", ns);
        let ns = match ns {
            Some(node) => node,
            None => return
        };
        elements.clear();
        elements.set(calculate_elements(&ns, None, vec![]));
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
    node_input: &NodeInput,
    parent_position: Option<Pos2>,
    mut elements: Vec<LinkBezierProps>
) -> Vec<LinkBezierProps> {

    for child in &node_input.clone().children.unwrap() {
        let parent_position = match node_input.position.clone() {
            None => return elements,
            Some(pos) => pos
        };
        elements = calculate_elements(child, Some(parent_position), elements);
    }

    tracing::debug!("parent_position: {:?}", parent_position);
    let actual_position = match node_input.position.clone() {
        None => return elements,
        Some(pos) => pos
    };

    tracing::debug!("actual_position: {:?}", actual_position);

    let parent_position = match parent_position {
        None => return elements,
        Some(pos) => pos
    };

    elements.push(LinkBezierProps {
        id: Option::from("link".to_string()),
        pos_start: parent_position,
        pos_end: actual_position,
        color: None,
        stroke_width: None,
    });
    tracing::debug!("elementsssssssssssssssssss: {:?}", elements.len());

    elements

}