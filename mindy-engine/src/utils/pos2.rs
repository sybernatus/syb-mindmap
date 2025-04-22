use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, PartialEq, Deserialize, Serialize)]
pub struct Pos2 {
    #[serde(default)]
    pub x: f32,
    #[serde(default)]
    pub y: f32,
}

impl Pos2 {
    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn subtract(&self, other: &Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }

    pub fn add(&self, other: &Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
