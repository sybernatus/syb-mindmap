use dioxus::logger::tracing;
use dioxus::prelude::*;
use mindy_engine::node::style::NodeStyle;
use mindy_engine::node::Node;
use mindy_engine::layout::pos2::Pos2;

#[derive(Props, PartialEq, Clone)]
pub struct NodeProps {
    pub node: Node,
}

#[component]
pub fn NodeComp(props: NodeProps) -> Element {

    let NodeStyle {
        background_color,
        text_wrapping,
        font_size,
        font_family,
        padding,
        max_width,
        min_width,
        text_color,
        hidden,
        ..
    } = props.node
        .clone()
        .style_custom
        .clone();

    tracing::trace!("node style - background_color: {:?} - font_size: {:?}", background_color, font_size);

    let Pos2 { x: pos_x, y: pos_y } = props.node.clone().position_real.unwrap_or(Pos2::zero());
    let text = props.node.text.clone().unwrap_or_else(|| "".to_string());
    let text_wrap = if text_wrapping { "wrap" } else { "nowrap" };
    let display = if hidden { "none" } else { "flex" };
    let data = props.node.image.clone().unwrap_or_default().data.unwrap_or_default();
    let width = props.node.image.clone().unwrap_or_default().width.unwrap_or_default();
    tracing::trace!("data: {:?} - width: {:?}", data, width);
    rsx! {
        div {
            class: "node",
            style: "display: {display};",
            style: "justify-content: center;",
            style: "align-items: center;",
            style: "background-color: {background_color.hex};",
            style: "min-width: {min_width}px;",
            style: "max-width: {max_width}px;",
            style: "max-height: 900px;",
            style: "text-wrap: {text_wrap};",
            style: "font-size: {font_size}px;",
            style: "padding: {padding}px;",
            style: "font-family: {font_family};",
            style: "font-size: {font_size}px;",
            style: "color: {text_color.hex};",
            style: "top: {pos_y}px;",
            style: "left: {pos_x}px;",
            div {
                class: "node-image",
                style: "display: flex;",
                style: "align-items: center;",
                img {
                    src: "{data}",
                    width: "{width}px",
                }
            }
            div {
                class: "node-text",
                "{text}",
            }
        }
    }
}
