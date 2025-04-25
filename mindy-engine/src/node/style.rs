use crate::utils::rgb::Rgb;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(default)]
pub struct NodeStyle {
    pub background_color: Option<Rgb>,
    pub children_hidden: bool,
    pub font_size: f32,
    pub font_family: String,
    pub max_width: f32,
    pub min_width: f32,
    pub min_height: f32,
    pub padding: f32,
    pub text_wrapping: bool,
}

impl Default for NodeStyle {
    fn default() -> Self {
        Self {
            background_color: None,
            children_hidden: false,
            text_wrapping: true,
            font_size: 12.0,
            padding: 10.0,
            font_family: "'Segoe UI', Tahoma, Geneva, Verdana, sans-serif".to_string(),
            max_width: 200.0,
            min_width: 0.0,
            min_height: 0.0,
        }
    }
}

impl NodeStyle {


    /// set color to the background color
    pub fn with_background_color(&mut self, color: Rgb) -> &Self {
        self.background_color = Some(color);
        self
    }
}