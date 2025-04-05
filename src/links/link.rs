use eframe::egui;
use eframe::emath::Pos2;
use eframe::epaint::{Color32, Stroke};
use egui::epaint::CubicBezierShape;
use egui::Rect;

#[derive(Debug, Clone)]
pub struct Link {
    pub node_source: Rect,
    pub node_target: Rect,
}

impl Link {

    pub fn new_from_nodes(node_source: Rect, node_target: Rect) -> Self {
        Self { node_source, node_target }
    }

    pub fn draw_bezier(&self, ui: &egui::Ui) {
        let points = vec![
            self.node_source.center(),
            Pos2::new(self.node_source.center().x + 50.0, self.node_source.center().y),
            Pos2::new(self.node_source.center().x + 5.0, self.node_target.center().y),
            self.node_target.center()
        ];
        let bezier = CubicBezierShape::from_points_stroke(
            <[Pos2; 4]>::try_from(points).unwrap(),
            false,
            Default::default(),
            Stroke::new(1.5, Color32::LIGHT_GRAY)
        );
        ui.painter().add(bezier);
    }
}