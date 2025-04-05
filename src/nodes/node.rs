use crate::nodes::content::NodeContent;
use crate::text_utils::text_size::calculate_text_size;
use eframe::egui;
use eframe::epaint::StrokeKind;
use egui::{Context, Pos2, Rect, Vec2};

#[derive(Debug, Clone, Default)]
pub struct Node {
    pub content: NodeContent,
    pub position: Pos2,
}

impl Node {

    pub fn new( Node { content, position }: Node) -> Self {
        Self { content, position }
    }

    pub fn get_rect(ctx: &Context, node: Node) -> Rect {

        let Node { content, position } = node.clone();
        let NodeContent { text, font_id, color } = content.clone();
        let text_size = calculate_text_size(ctx, text.as_str(), font_id.to_owned(), color);

        let node_size = Vec2::new(text_size.x + 20.0, text_size.y + 20.0);
        Rect::from_min_size(position, node_size)
    }

    pub fn draw(&self, ui: &egui::Ui) {

        let node_shape = Self::get_rect(ui.ctx(), self.clone());
        ui.painter()
            .rect_filled(
                node_shape,
                40.0,
                egui::Color32::from_rgb(122, 10, 0),
            );
        ui.painter()
            .rect_stroke(node_shape, 40.0, egui::Stroke::new(1.0, egui::Color32::from_rgb(255, 255, 255)), StrokeKind::Inside);

        let NodeContent { text, font_id, color } = self.content.clone();
        let text_size = calculate_text_size(ui.ctx(), text.as_str(), font_id.to_owned(), color);

        let node_size = Vec2::new(text_size.x + 20.0, text_size.y + 20.0);
        let node_draw = Rect::from_min_size(self.position, node_size);
        let node_center = node_draw.center();
        let text_pos = Pos2::new(
            node_center.x - text_size.x / 2.0,
            node_center.y - text_size.y / 2.0,
        );
        ui.painter()
            .text(
                text_pos,
                egui::Align2::LEFT_TOP,
                text,
                font_id,
                color,
            );
    }
}