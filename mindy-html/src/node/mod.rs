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
        ..
    } = props.node
        .clone()
        .style_custom
        .clone();

    let background_color = background_color.unwrap_or_default();
    tracing::debug!("node style - background_color: {:?} - font_size: {:?}", background_color, font_size);

    let Pos2 { x: pos_x, y: pos_y } = props.node.clone().position_real.unwrap_or_default();
    let text = props.node.text.clone().unwrap_or_else(|| "".to_string());
    let text_wrap = if text_wrapping { "wrap" } else { "nowrap" };

    rsx! {
        div {
            class: "node",
            style: "background-color: {background_color.hex};",
            style: "min-width: {min_width}px;",
            style: "max-width: {max_width}px;",
            style: "max-height: 900px;",
            style: "text-wrap: {text_wrap};",
            style: "font-size: {font_size}px;",
            style: "padding: {padding}px;",
            style: "font-family: {font_family};",
            style: "font-size: {font_size}px;",
            top: "{pos_y}px",
            left: "{pos_x}px",
            id: "test",
            "{text}",
        }
    }
}
