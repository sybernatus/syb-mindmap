use crate::link_beziers::{LinkBezierComp, LinkBezierProps};
use crate::mindmap::MINDMAP;
use dioxus::logger::tracing;
use dioxus::prelude::*;
use mindy_engine::link::Link;
use mindy_engine::node::Node;
use mindy_engine::layout::pos2::Pos2;
use mindy_engine::layout::Size2D;
use mindy_engine::layout::size::Size;

#[component]
pub fn LinkRendererComp() -> Element {
    let mut elements: Signal<Vec<LinkBezierProps>> = use_signal(|| vec![]);

    use_effect(move || {
        let mindmap_position = MINDMAP().position;
        let mindmap_data = MINDMAP().data;
        let elements_new = to_links_vec(mindmap_data.to_owned(), None, None, mindmap_position,vec![]);
        elements.clear();
        elements.set(elements_new);
    });

    rsx! {
        div {
            class: "link-renderer",
            id: "link-renderer",
            style: "min-width: inherit;",
            style: "min-height: inherit;",
            for element in elements() {
                LinkBezierComp {
                    link: element.link.clone(),
                }
            }
        }
    }
}

/// Compute a vector of links from a parent node moving through all children
/// The link is calculated as follows:
/// - The end point is the real position of the child node
/// - The start point is the real position of the parent node
fn to_links_vec(
    current_node: Option<Node>,
    parent_position: Option<Pos2>,
    parent_size: Option<Size>,
    offset: Option<Pos2>,
    mut elements: Vec<LinkBezierProps>,
) -> Vec<LinkBezierProps> {

    let offset_input = offset.clone().unwrap_or_else(|| Pos2::default());
    tracing::trace!("offset_input: {:?}", offset_input);

    let node_input = match current_node {
        Some(node) => node,
        None => return elements,
    };

    if node_input.text.is_none() || node_input.clone().text.unwrap().is_empty() {
        return elements;
    }

    let children = node_input.to_owned().children.unwrap_or_else(|| vec![]);
    tracing::trace!("children: {:?}", children.len());

    for child in children {
        let parent_position = match node_input.position_real.clone() {
            None => return elements,
            Some(pos) => pos,
        };

        let parent_size = node_input
            .get_graphical_size();

        elements = to_links_vec(Some(child), Some(parent_position), Some(parent_size), offset.clone(), elements);
    }

    tracing::trace!("parent_position: {:?}", parent_position);
    let actual_position = match node_input.position_real.clone() {
        None => return elements,
        Some(pos) => pos,
    };

    tracing::trace!("actual_position: {:?}", actual_position);

    let parent_position = match parent_position {
        None => return elements,
        Some(pos) => pos,
    };

    let current_size = node_input
        .get_graphical_size();

    let parent_size = parent_size.unwrap_or_else(|| Size::new(200.0, 200.0));

    let link = Link::from_start_end(parent_position.clone(), actual_position.clone(), parent_size.clone(), current_size.clone())
        .with_path_data_bezier(0.5);

    elements.push(LinkBezierProps {
        link,
    });

    elements
}
