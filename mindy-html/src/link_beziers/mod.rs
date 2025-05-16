use dioxus::logger::tracing;
use dioxus::prelude::*;
use mindy_engine::link::{Link};
use mindy_engine::utils::color::Color;

#[derive(Props, PartialEq, Clone)]
pub struct LinkBezierProps {
    pub link: Link
}

#[component]
pub fn LinkBezierComp(props: LinkBezierProps) -> Element {

    let path_data = props.link.path_data.unwrap_or_else(|| "".to_string());
    let stroke_width = props.link.stroke_width.unwrap_or(3.0);
    let color = props.link.color.unwrap_or(Color::from_hex("#7F6398".to_string()));

    tracing::trace!("path_data: {:?} - stroke_width: {:?} - color: {:?}", path_data, stroke_width, color.hex);

    rsx! {
        svg {
            class: "link",
            id: "link",
            path {
                d: "{path_data}",
                stroke: "{color.hex}",
                stroke_width: "{stroke_width}",
                fill: "none",
            }
        }
    }
}
