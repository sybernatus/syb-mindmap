use crate::mindmap::metadata::MindmapMetadata;
use crate::node::Node;
use serde::Deserialize;

pub mod metadata;
pub mod style;
pub mod r#type;

#[derive(Debug, Clone, PartialEq, Deserialize)]
pub struct MindMap {
    pub metadata: Option<MindmapMetadata>,
    pub data: Option<Node>,
}

impl MindMap {
    pub fn new(metadata: Option<MindmapMetadata>, data: Option<Node>) -> Self {
        Self { metadata, data }
    }

    pub fn with_metadata(&self, metadata: MindmapMetadata) -> Self {
        Self { metadata: Some(metadata), ..self.clone() }
    }

    pub fn with_data(&self, data: Node) -> Self {
        Self { data: Some(data), ..self.clone() }
    }
}

impl Default for MindMap {
    fn default() -> Self {
        Self {
            metadata: Some(MindmapMetadata::default()),
            data: Some(Node::default()),
        }
    }
}