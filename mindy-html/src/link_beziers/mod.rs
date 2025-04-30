use dioxus::logger::tracing;
use dioxus::prelude::*;
use mindy_engine::link::{Link};
use mindy_engine::utils::rgb::Rgb;
use regex::Regex;

#[derive(Props, PartialEq, Clone)]
pub struct LinkBezierProps {
    pub link: Link
}

#[component]
pub fn LinkBezierComp(props: LinkBezierProps) -> Element {

    let path_data = props.link.path_data.unwrap_or_else(|| "".to_string());
    let stroke_width = props.link.stroke_width.unwrap_or(3.0);
    let color = props.link.color.unwrap_or(Rgb::new(200.0, 200.0, 200.0));
    let stoke = format!("rgb({}, {}, {}) ", color.red, color.green, color.blue);

    tracing::trace!("path_data: {:?} - stroke_width: {:?} - color: {:?}", path_data, stroke_width, stoke);

    rsx! {
        svg {
            class: "link",
            id: "link",
            path {
                d: "{path_data}",
                stroke: "{stoke}",
                stroke_width: "{stroke_width}",
                fill: "none",
            }
        }
    }
}
