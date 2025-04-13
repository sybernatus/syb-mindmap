use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
pub enum MindmapType {
    Standard,
}

impl Default for MindmapType {
    fn default() -> Self {
        Self::Standard
    }
}
