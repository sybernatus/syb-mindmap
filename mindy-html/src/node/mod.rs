use dioxus::html::completions::CompleteWithBraces::set;
use dioxus::prelude::*;
use mindy_engine::node::style::NodeStyle;
use mindy_engine::node::Node;
use mindy_engine::utils::pos2::Pos2;
use crate::MINDMAP_DATA;
use crate::node_renderer::NodeRendererState;

#[derive(Props, PartialEq, Clone)]
pub struct NodeProps {
    pub node: Node,
}

#[component]
pub fn NodeComp(props: NodeProps) -> Element {
    let mindmap_bounding_box_position: Signal<Pos2> = use_context::<NodeRendererState>().mindmap_bounding_box_position;
    let node = use_memo(move || props.node.clone());
    let mut node_pos: Signal<Pos2> = use_signal(|| Pos2::default());

    use_effect(move || {
        // When we read count, it becomes a dependency of the effect
        let node_position_from_initial = node().position_from_initial;
        let mindmap_bounding_box_position = mindmap_bounding_box_position();
        node_pos.set(Node::get_position_real(node_position_from_initial, mindmap_bounding_box_position).clone().unwrap_or_default());
    });

    let on_click = move |_| {
        // NODE_LIST.write().iter_mut().for_each(|node| {
        //     if node.id == props.node.id {
        //         tracing::trace!("Node clicked, hide children: {:?}", props.node.style_custom.children_hidden);
        //         node.style_custom.children_hidden = !node.style_custom.children_hidden;
        //     }
        // });
    };

    let NodeStyle {
        background_color,
        text_wrapping,
        font_size,
        font_family,
        padding,
        max_width,
        min_width,
        ..
    } = node()
        .clone()
        .style_custom
        .clone()
        .unwrap_or(NodeStyle::default());


    let text_size = node().get_graphical_size();
    let text = node().text.clone().unwrap_or_else(|| "".to_string());
    let text_wrap = if text_wrapping { "wrap" } else { "nowrap" };

    rsx! {
        div {
            class: "node",
            onclick: on_click,
            style: "background-color: rgb({background_color.red}, {background_color.green}, {background_color.blue});",
            style: "min-width: {min_width}px;",
            style: "max-width: {max_width}px;",
            style: "min-height: {text_size.height}px;",
            style: "text-wrap: {text_wrap};",
            style: "font-size: {font_size}px;",
            style: "padding: {padding}px;",
            style: "font-family: {font_family};",
            style: "font-size: {font_size}px;",
            top: "{node_pos().y}px",
            left: "{node_pos().x}px",
            id: "test",
            "{text}"
        }
    }
}
