use crate::mindmap::style::MindmapStyle;
use crate::mindmap::r#type::MindmapType;
use crate::utils::pos2::Pos2;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct MindmapMetadata {
    #[serde(default)]
    pub diagram_type: MindmapType,
    #[serde(default)]
    pub style: MindmapStyle,
    #[serde(default)]
    pub position_starting: Option<Pos2>,
}

impl MindmapMetadata {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_style(&self, style: MindmapStyle) -> Self {
        Self {
            style,
            ..self.clone()
        }
    }

    pub fn with_diagram_type(&self, diagram_type: MindmapType) -> Self {
        Self {
            diagram_type,
            ..self.clone()
        }
    }

    pub fn get_style(&self) -> MindmapStyle {
        self.clone().style
    }
}

impl Default for MindmapMetadata {
    fn default() -> Self {
        Self {
            diagram_type: MindmapType::default(),
            style: MindmapStyle::default(),
            position_starting: None,
        }
    }
}
