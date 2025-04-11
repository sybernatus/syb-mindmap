use dioxus::logger::tracing;
use dioxus::prelude::*;
use mindy_engine::node::{Node, NodeStyleCustom};
use crate::NODE_LIST;

#[derive(Props, PartialEq, Clone)]
pub struct NodeProps {
    pub id: String,
    pub class: String,
    pub node: Node,
}

#[component]
pub fn Node(props: NodeProps) -> Element {
    tracing::trace!("Node position x: {:?} - y: {:?}", props.node.position.x, props.node.position.y);

    let on_click = move |_| {
        NODE_LIST.write().iter_mut().for_each(|node| {
            if node.id == props.node.id {
                tracing::trace!("Node clicked, hide children: {:?}", props.node.style_custom.children_hidden);
                node.style_custom.children_hidden = !node.style_custom.children_hidden;
            }
        });
    };
    let bg_color = props.node.style_custom.background_color;

    let NodeStyleCustom {
        background_color,
        text_wrapping,
        children_hidden,
        font_size,
        font_family,
        padding,
        ..
    } = props.node.style_custom.clone();

    let text_size = props.node.get_graphical_size();
    let text = props.node.content.text.clone().unwrap_or_else(|| "".to_string());
    let text_wrap = if text_wrapping {
        "wrap"
    } else {
        "nowrap"
    };

    rsx! {
        div {
            class: "node",
            onclick: on_click,
            style: "background-color: rgb({background_color.red}, {background_color.green}, {background_color.blue});",
            style: "width: {text_size.width}px;",
            style: "height: {text_size.height}px;",
            style: "text-wrap: {text_wrap};",
            style: "font-size: {font_size}px;",
            style: "padding: {padding}px;",
            style: "font-family: {font_family};",
            style: "font-size: {font_size}px;",
            top: "{props.node.position.y}px",
            left: "{props.node.position.x}px",
            id: "test",
            "{text}"
        }
    }
}