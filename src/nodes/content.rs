use eframe::epaint::{Color32, FontId};


#[derive(Debug, Clone, Default)]
pub struct NodeContent {
    pub text: String,
    pub font_id: FontId,
    pub color: Color32,
}