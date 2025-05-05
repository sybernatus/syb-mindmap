use serde::{Deserialize, Serialize};


#[derive(Debug, Clone, PartialEq, Deserialize, Default, Serialize)]
pub enum ImagePosition {
    #[default]
    Left,
    Right,
    Top,
    Bottom,
}

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(default)]
pub struct NodeImage {
    pub data: Option<String>,
    pub path: Option<String>,
    pub width: Option<f32>,
    pub position: ImagePosition
}

impl Default for NodeImage {
    fn default() -> Self {
        Self {
            data: None,
            path: None,
            width: None,
            position: ImagePosition::Left
        }
    }
}