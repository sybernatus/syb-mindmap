use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum MindmapType {
    // Standard,
    LeftRightHorizontal,
}

impl Default for MindmapType {
    fn default() -> Self {
        Self::LeftRightHorizontal
    }
}
