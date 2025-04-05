use eframe::epaint::{Color32, FontId};


#[derive(Debug, Clone)]
pub struct NodeContent {
    pub text: String,
    pub font_id: FontId,
    pub color: Color32,
}

impl Default for NodeContent {
    fn default() -> Self {
        Self {
            text: "".to_string(),
            font_id: Default::default(),
            color: Color32::from_rgb(255, 255, 255),
        }
    }
}