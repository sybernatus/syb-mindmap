use serde::Deserialize;

#[derive(Debug, Clone, PartialEq, Deserialize, Default)]
pub enum MindmapType {
    #[default]
    Standard,
}
