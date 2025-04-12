use serde::Deserialize;

#[derive(Debug, Clone, Default, PartialEq, Deserialize)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}
