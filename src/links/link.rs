use eframe::egui;
use eframe::emath::Pos2;
use eframe::epaint::{Color32, Stroke};
use egui::epaint::CubicBezierShape;
use egui::Rect;

#[derive(Debug, Clone)]
pub struct LinkStyleBezier {
    pub offset: f32,
    pub color: Color32,
    pub width: f32,
}

impl Default for LinkStyleBezier {
    fn default() -> Self {
        Self { offset: 0.15, color: Color32::LIGHT_GRAY, width: 2.0 }
    }
}
#[derive(Debug, Clone)]
pub struct Link {
    pub node_source: Rect,
    pub node_target: Rect,
    pub style: LinkStyleBezier,
}



impl Link {

    pub fn new_from_nodes(node_source: Rect, node_target: Rect) -> Self {
        Self { node_source, node_target, style: LinkStyleBezier::default() }
    }

    pub fn draw_bezier(&self, ui: &egui::Ui) {

        let (p1, p2) = Self::compute_control_points_offset(
            self.node_source.center(),
            self.node_target.center(),
            self.style.offset
        );


        let (
            direction_x,
            direction_y,
            distance,
            normal_x,
            normal_y
        ) = Self::get_link_metadata(self.node_source.center(), self.node_target.center());

        println!("dist: {} - dx: {} - dy: {}",distance, direction_x / distance, direction_y / distance);

        let p0x = match direction_x {
            d if d > 0.0 => self.node_source.center().x + self.node_source.size().x / 2.0 - 10.0,
            d if d < 0.0 => self.node_source.center().x - self.node_source.size().x / 2.0 + 10.0,
            _ => 0.0
        };
        let p0y = match direction_y {
            d if d > 0.0 => self.node_source.center().y + self.node_source.size().y / 2.0 - 5.0,
            d if d < 0.0 => self.node_source.center().y - self.node_source.size().y / 2.0 + 5.0,
            _ => 0.0
        };
        let p3x = match direction_x {
            d if d > 0.0 => self.node_target.center().x - self.node_target.size().x / 2.0 + 10.0,
            d if d < 0.0 => self.node_target.center().x + self.node_target.size().x / 2.0 - 10.0,
            _ => 0.0
        };

        let p3y = match direction_y {
            d if d > 0.0 => self.node_target.center().y - self.node_target.size().y / 2.0 + 5.0,
            d if d < 0.0 => self.node_target.center().y + self.node_target.size().y / 2.0 - 5.0,
            _ => 0.0
        };

        let p0 = Pos2::new(p0x, p0y);
        let p3 = Pos2::new(p3x, p3y);

        let points = [
            p0,
            p1,
            p2,
            p3
        ];

        let bezier = CubicBezierShape::from_points_stroke(
            points,
            false,
            Default::default(),
            Stroke::new(
                self.style.width,
                self.style.color
            )
        );
        ui.painter().add(bezier);
    }

    fn get_link_metadata(
        start: Pos2,
        end: Pos2
    ) -> (f32, f32, f32, f32, f32) {
        let direction_x = end.x - start.x;
        let direction_y = end.y - start.y;

        let distance = (direction_x.powi(2) + direction_y.powi(2)).sqrt();

        let normal_x = -direction_y / distance;
        let normal_y = direction_x / distance;
        ( direction_x, direction_y, distance, normal_x, normal_y)
    }

    fn compute_control_points_offset(
        start: Pos2,
        end: Pos2,
        offset_ratio: f32,
    ) -> (Pos2, Pos2) {

        let (direction_x, direction_y, distance, normal_x, normal_y) = Self::get_link_metadata(start, end);

        let control_point1 = Pos2 {
            x: start.x + direction_x / 3.0 + normal_x * distance * offset_ratio,
            y: start.y + direction_y / 3.0 + normal_y * distance * offset_ratio,
        };

        let control_point2 = Pos2 {
            x: start.x + 2.0 * direction_x / 3.0 + normal_x * distance * -offset_ratio,
            y: start.y + 2.0 * direction_y / 3.0 + normal_y * distance * -offset_ratio,
        };

        (control_point1, control_point2)
    }
}