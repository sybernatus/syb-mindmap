use crate::mindmap::style::MindmapStyle;
use crate::mindmap::r#type::MindmapType;
use crate::utils::pos2::Pos2;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct MindmapMetadata {
    pub diagram_type: Option<MindmapType>,
    pub style: Option<MindmapStyle>,
    pub position_starting: Option<Pos2>,
}

impl MindmapMetadata {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_style(&self, style: Option<MindmapStyle>) -> Self {
        Self {
            style,
            ..self.clone()
        }
    }

    pub fn with_diagram_type(&self, diagram_type: Option<MindmapType>) -> Self {
        Self {
            diagram_type,
            ..self.clone()
        }
    }

    pub fn get_style(&self) -> MindmapStyle {
        self.clone()
            .style
            .unwrap_or_else(|| MindmapStyle::default())
    }
}

impl Default for MindmapMetadata {
    fn default() -> Self {
        Self {
            diagram_type: Some(MindmapType::default()),
            style: Some(MindmapStyle::default()),
            position_starting: Some(Pos2::new(300.0, 300.0)),
        }
    }
}
