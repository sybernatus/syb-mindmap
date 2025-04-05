use crate::nodes::content::NodeContent;
use crate::text_utils::text_size::calculate_text_size;
use eframe::egui;
use eframe::epaint::StrokeKind;
use egui::{Context, Pos2, Rect, Vec2};

#[derive(Debug, Clone, Default)]
pub struct Link {
    pub content: NodeContent,
    pub position: Pos2,
}
