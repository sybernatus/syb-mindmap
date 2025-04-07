use dioxus::html::a::background_color;
use dioxus::prelude::*;
use mindy_engine::node::Node;

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

    rsx! {
        div {
            "style": "background-color: rgb({bg_color.red}, {bg_color.green}, {bg_color.blue})",
            position: "absolute",
            top: "{props.node.position.0}px",
            left: "{props.node.position.1}px",
            class: "node",
            id: "test",
            "{text}"
        }
    }
}