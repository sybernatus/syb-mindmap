use dioxus::logger::tracing;
use dioxus::prelude::*;
use mindy_engine::node::Node;
use crate::NODE_LIST;

#[derive(Props, PartialEq, Clone)]
pub struct NodeProps {
    pub id: String,
    pub class: String,
    pub node: Node,
}

#[component]
pub fn Node(props: NodeProps) -> Element {
    let text = props.node.content.text.unwrap_or_else(|| "".to_string());
    let bg_color = props.node.style_custom.color;
    tracing::trace!("Node position x: {:?} - y: {:?}", props.node.position.x, props.node.position.y);

    let on_click = move |_| {
        NODE_LIST.write().iter_mut().for_each(|node| {
            if node.id == props.node.id {
                tracing::trace!("Node clicked, hide children: {:?}", props.node.style_custom.children_hidden);
                node.style_custom.children_hidden = !node.style_custom.children_hidden;
            }
        });
    };

    let text_wrap = if props.node.style_custom.text_wrapping {
        "wrap"
    } else {
        "nowrap"
    };

    let text_size = props.node.size.clone().unwrap();

    rsx! {
        div {
            class: "node",
            onclick: on_click,
            style: "background-color: rgb({bg_color.red}, {bg_color.green}, {bg_color.blue});",
            style: "width: {text_size.width}px;",
            style: "height: {text_size.height}px;",
            style: "text-wrap: {text_wrap};",
            style: "font-size: {props.node.style_custom.font_size}px;",
            style: "padding: {props.node.style_custom.padding}px;",
            top: "{props.node.position.y}px",
            left: "{props.node.position.x}px",
            id: "test",
            "{text}"
        }
    }
}