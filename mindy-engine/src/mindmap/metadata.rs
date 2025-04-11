use serde::Deserialize;
use crate::mindmap::r#type::MindmapType;
use crate::mindmap::style::MindmapStyle;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct MindmapMetadata {
    pub diagram_type: MindmapType,
    pub style: MindmapStyle,
}

impl MindmapMetadata {
    pub fn new(diagram_type: MindmapType, style: MindmapStyle) -> Self {
        Self { diagram_type, style }
    }

    pub fn with_style(&self, style: MindmapStyle) -> Self {
        Self { style, ..self.clone() }
    }

    pub fn with_diagram_type(&self, diagram_type: MindmapType) -> Self {
        Self { diagram_type, ..self.clone() }
    }

    // pub fn layout_mindmap_standard(&mut self) -> Self {
    //
    //     self
    // }
}

impl Default for MindmapMetadata {
    fn default() -> Self {
        Self {
            diagram_type: MindmapType::default(),
            style: MindmapStyle::default(),
        }
    }
}