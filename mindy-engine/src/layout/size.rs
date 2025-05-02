use serde::{Deserialize, Serialize};
use crate::layout::Size2D;

#[derive(Debug, Clone, Default, PartialEq, Deserialize, Serialize)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

impl Size2D for Size {

    fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }
    fn width(&self) -> f32 {
        self.width
    }

    fn height(&self) -> f32 {
        self.height
    }

    fn width_mut(&mut self) -> &mut f32 {
        &mut self.width
    }

    fn height_mut(&mut self) -> &mut f32 {
        &mut self.height
    }
}