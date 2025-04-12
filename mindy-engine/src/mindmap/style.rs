use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct MindmapStyle {
    pub padding_horizontal: f32,
    pub padding_vertical: f32,
}

impl Default for MindmapStyle {
    fn default() -> Self {
        Self {
            padding_horizontal: 40.0,
            padding_vertical: 20.0,
        }
    }
}

impl MindmapStyle {
    pub fn new(padding_horizontal: f32, padding_vertical: f32) -> Self {
        Self {
            padding_horizontal,
            padding_vertical,
        }
    }

    pub fn get_padding_horizontal(&self) -> f32 {
        self.padding_horizontal
    }
}
