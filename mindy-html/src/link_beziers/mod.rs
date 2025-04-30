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
    let color = props.link.color.unwrap_or(Rgb::new(125.0, 155.0, 155.0));
    let stoke = format!("rgb({}, {}, {}) ", color.red, color.green, color.blue);

    tracing::trace!("path_data: {:?} - stroke_width: {:?} - color: {:?}", path_data, stroke_width, stoke);
    let points = Regex::new(r"M([\d.]+),([\d.]+)\s+C([\d.]+),([\d.]+)\s+([\d.]+),([\d.]+)\s+([\d.]+),([\d.]+)").ok().unwrap();
    let caps = points.captures(&*path_data).unwrap();
    let caps = Some([
        (caps[1].parse::<String>().ok().unwrap_or_default(), caps[2].parse::<String>().ok().unwrap_or_default()), // M: start
        (caps[3].parse::<String>().ok().unwrap_or_default(), caps[4].parse::<String>().ok().unwrap_or_default()), // C1
        (caps[5].parse::<String>().ok().unwrap_or_default(), caps[6].parse::<String>().ok().unwrap_or_default()), // C2
        (caps[7].parse::<String>().ok().unwrap_or_default(), caps[8].parse::<String>().ok().unwrap_or_default()), // end
    ]);
    for cap in &caps {
        tracing::debug!("cap: {:?}", cap);
    }
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
        for cap in caps {
            circle {
                style: "z-index: 10;",
                cx: "{cap[0].0}",
                cy: "{cap[0].1}",
                r: "5",
                fill: "black",
            }
            circle {
                style: "z-index: 0;",
                cx: "{cap[1].0}",
                cy: "{cap[1].1}",
                r: "5",
                fill: "red",
            }
            circle {
                style: "z-index: 0;",
                cx: "{cap[2].0}",
                cy: "{cap[2].1}",
                r: "5",
                fill: "green",
            }
            circle {
                style: "z-index: 0;",
                cx: "{cap[3].0}",
                cy: "{cap[3].1}",
                r: "5",
                fill: "yellow",
            }
        }
        }
    }
}
