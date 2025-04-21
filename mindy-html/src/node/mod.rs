use dioxus::prelude::*;
use mindy_engine::node::style::NodeStyle;
use mindy_engine::node::Node;
use mindy_engine::utils::pos2::Pos2;
use mindy_engine::utils::size::Size;
use crate::MINDMAP_DATA;

#[derive(Props, PartialEq, Clone)]
pub struct NodeProps {
    pub node: Node,
}

#[component]
pub fn NodeComp(props: NodeProps) -> Element {
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
    } = props
        .node
        .clone()
        .style_custom
        .clone()
        .unwrap_or(NodeStyle::default());

    let Pos2 { x, y, .. } = props
        .node
        .clone()
        .position
        .unwrap_or_else(|| Pos2::new(0.0, 0.0));
    let text_size = props.node.get_graphical_size();
    let text = props.node.text.clone().unwrap_or_else(|| "".to_string());
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
            top: "{y - mindmap_pos().y}px",
            left: "{x - mindmap_pos().x}px",
            id: "test",
            "{text}"
        }
    }
}
