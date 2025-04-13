use dioxus::logger::tracing;
use dioxus::prelude::*;
use mindy_engine::link::bezier_svg_path;
use mindy_engine::utils::pos2::Pos2;
use palette::rgb::Rgb;

#[derive(Props, PartialEq, Clone)]
pub struct LinkBezierProps {
    pub id: Option<String>,
    pub pos_start: Pos2,
    pub pos_end: Pos2,
    pub color: Option<Rgb>,
    pub stroke_width: Option<f32>,
}

#[component]
pub fn LinkBezierComp(props: LinkBezierProps) -> Element {
    let path_data = bezier_svg_path(props.pos_start, props.pos_end, 0.15);
    let color = props.color.unwrap_or(Rgb::new(255.0, 255.0, 255.0));
    let stroke_width = props.stroke_width.unwrap_or(2.0);
    tracing::trace!("Link Bezier path_data {:?}", path_data);
    rsx! {
        svg {
            class: "link",
            id: "link",
            path {
                d: "{path_data}",
                stroke: "rgb({color.red}, {color.green}, {color.blue})",
                stroke_width: "{stroke_width}",
                fill: "none",
            }

        }
    }
}
