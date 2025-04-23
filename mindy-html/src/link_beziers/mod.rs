use dioxus::logger::tracing;
use dioxus::prelude::*;
use mindy_engine::link::bezier_svg_path;
use mindy_engine::utils::pos2::Pos2;
use palette::rgb::Rgb;
use crate::mindmap::MINDMAP;

#[derive(Props, PartialEq, Clone)]
pub struct LinkBezierProps {
    pub id: Option<String>,
    pub pos_start: Pos2,
    pub pos_end: Pos2,
    pub color: Option<Rgb>,
    pub stroke_width: Option<f32>,
    pub path_data: Option<String>,
}

#[component]
pub fn LinkBezierComp(props: LinkBezierProps) -> Element {
    let mut path_data = use_signal(|| String::new());
    let path_pos_start = use_signal(|| props.pos_start);
    let path_pos_end = use_signal(|| props.pos_end);
    // let path_data = bezier_svg_path(props.pos_start, props.pos_end, 0.2, &mindmap_bounding_box_position());
    let color = props.color.unwrap_or(Rgb::new(255.0, 255.0, 255.0));
    let stroke_width = props.stroke_width.unwrap_or(2.0);

    use_effect(move || {
        let mindmap = MINDMAP();
        let mindmap_bounding_box_position = mindmap.position;
        let path_pos_start = path_pos_start();
        let path_pos_end = path_pos_end();
        path_data.set(bezier_svg_path(path_pos_start, path_pos_end, 0.2, &mindmap_bounding_box_position.unwrap_or_default()));

        tracing::debug!("Link Bezier effect path_data {:?}", path_data());
    });

    // tracing::debug!("Link Bezier path_data {:?}", path_data());
    rsx! {
        svg {
            class: "link",
            id: "link",
            path {
                d: "{path_data()}",
                stroke: "rgb({color.red}, {color.green}, {color.blue})",
                stroke_width: "{stroke_width}",
                fill: "none",
            }

        }
    }
}
