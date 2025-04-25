use serde::{Deserialize, Serialize};
use crate::utils::rgb::Rgb;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct MindmapStyle {
    #[serde(default)]
    pub padding_horizontal: f32,
    #[serde(default)]
    pub padding_vertical: f32,
    #[serde(default)]
    pub root_node_color: Rgb,
}

impl Default for MindmapStyle {
    fn default() -> Self {
        Self {
            padding_horizontal: 40.0,
            padding_vertical: 20.0,
            root_node_color: Rgb::new(100.0, 100.0, 100.0),
        }
    }
}

impl MindmapStyle {
    pub fn new(padding_horizontal: f32, padding_vertical: f32) -> Self {
        Self {
            padding_horizontal,
            padding_vertical,
            root_node_color: Rgb::new(0.0, 0.0, 0.0),
        }
    }

    pub fn get_padding_horizontal(&self) -> f32 {
        self.padding_horizontal
    }

    pub fn with_root_node_color(&mut self, color: Rgb) -> &Self {
        self.root_node_color = color;
        self
    }
}
