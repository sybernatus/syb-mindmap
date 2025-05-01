use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum MindmapType {
    LeftRightBottom,
    LeftRightHorizontal,
}

impl Default for MindmapType {
    fn default() -> Self {
        Self::LeftRightHorizontal
    }
}
