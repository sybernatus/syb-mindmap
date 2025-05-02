use serde::{Deserialize, Serialize};
use crate::utils::color::Color;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct MindmapStyle {
    #[serde(default)]
    pub padding_horizontal: f32,
    #[serde(default)]
    pub padding_vertical: f32,
    #[serde(default)]
    pub root_node_color: Color,
}

impl Default for MindmapStyle {
    fn default() -> Self {
        Self {
            padding_horizontal: 40.0,
            padding_vertical: 20.0,
            root_node_color: Color::from_hex("#AA4545".to_string()),
        }
    }
}

impl MindmapStyle {
    pub fn new(padding_horizontal: f32, padding_vertical: f32) -> Self {
        Self {
            padding_horizontal,
            padding_vertical,
            root_node_color: Color::from_rgb(0, 0, 0),
        }
    }

    pub fn get_padding_horizontal(&self) -> f32 {
        self.padding_horizontal
    }

    pub fn with_root_node_color(&mut self, color: Color) -> &Self {
        self.root_node_color = color;
        self
    }
}
