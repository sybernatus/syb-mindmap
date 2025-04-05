use crate::text_utils::text_size::calculate_text_size;
use eframe::egui;
use eframe::epaint::StrokeKind;
use egui::{Color32, Context, FontId, Pos2, Rect, Vec2};

pub struct Node {
    pub text: String,
}

pub fn draw_node(ctx: &Context, ui: &egui::Ui, node: Node) {

    let text_font_id = FontId::proportional(20.0);
    let text_color = Color32::from_rgb(255, 255, 255);
    let text_size = calculate_text_size(ctx, node.text.as_str(), text_font_id.to_owned(), text_color);

    let node_pos = Pos2::new(10.0, 20.0);
    let node_size = Vec2::new(text_size.x + 20.0, text_size.y + 20.0);
    let node_draw = Rect::from_min_size(node_pos, node_size);

    ui.painter()
        .rect_filled(
            node_draw,
            40.0,
            egui::Color32::from_rgb(122, 10, 0),
        );
    ui.painter()
        .rect_stroke(node_draw, 40.0, egui::Stroke::new(1.0, egui::Color32::from_rgb(255, 255, 255)), StrokeKind::Inside);

    let node_center = node_draw.center();
    let text_pos = Pos2::new(
        node_center.x - text_size.x / 2.0,
        node_center.y - text_size.y / 2.0,
    );
    ui.painter()
        .text(
            text_pos,
            egui::Align2::LEFT_TOP,
            node.text,
            text_font_id,
            text_color,
        );
}