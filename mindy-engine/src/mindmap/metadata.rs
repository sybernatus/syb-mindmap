use crate::mindmap::style::MindmapStyle;
use crate::mindmap::r#type::MindmapType;
use crate::layout::pos2::Pos2;
use serde::{Deserialize, Serialize};
use crate::node::style::NodeStyle;

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(default)]
pub struct MindmapMetadata {
    pub diagram_type: MindmapType,
    pub style: MindmapStyle,
    pub position_starting: Option<Pos2>,
    pub global_node_style: NodeStyle
}

impl Default for MindmapMetadata {
    fn default() -> Self {
        Self {
            diagram_type: MindmapType::default(),
            style: MindmapStyle::default(),
            position_starting: Some(Pos2::zero()),
            global_node_style: NodeStyle::default(),
        }
    }
}
