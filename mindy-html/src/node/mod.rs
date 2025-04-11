use dioxus::logger::tracing;
use dioxus::prelude::*;
use mindy_engine::node_input::{NodeInput, NodeStyleCustom, Pos2};

#[derive(Props, PartialEq, Clone)]
pub struct NodeProps {
    pub node: NodeInput,
}

#[component]
pub fn Node(props: NodeProps) -> Element {

    let on_click = move |_| {
        // NODE_LIST.write().iter_mut().for_each(|node| {
        //     if node.id == props.node.id {
        //         tracing::trace!("Node clicked, hide children: {:?}", props.node.style_custom.children_hidden);
        //         node.style_custom.children_hidden = !node.style_custom.children_hidden;
        //     }
        // });
    };

    let NodeStyleCustom {
        background_color,
        text_wrapping,
        children_hidden,
        font_size,
        font_family,
        padding,
        max_width,
        min_width,
        ..
    } = props.node.clone().style_custom.clone().unwrap_or(NodeStyleCustom::default());

    let Pos2 {
        x,
        y,
        ..
    } = props.node.clone().position.unwrap();
    let text_size = props.node.get_graphical_size();
    let text = props.node.text.clone().unwrap_or_else(|| "".to_string());
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
            style: "min-width: {min_width}px;",
            style: "max-width: {max_width}px;",
            style: "min-height: {text_size.height}px;",
            style: "text-wrap: {text_wrap};",
            style: "font-size: {font_size}px;",
            style: "padding: {padding}px;",
            style: "font-family: {font_family};",
            style: "font-size: {font_size}px;",
            top: "{y}px",
            left: "{x}px",
            id: "test",
            "{text}"
        }
    }
}