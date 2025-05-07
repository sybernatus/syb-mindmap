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

    fn max(&self, other: Size) -> Self {
        Self {
            width: self.width.max(other.width),
            height: self.height.max(other.height),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_size() {
        let size = Size::new(100.0, 200.0);
        assert_eq!(size.width, 100.0);
        assert_eq!(size.height, 200.0);
    }

    #[test]
    fn test_size_default() {
        let size: Size = Default::default();
        assert_eq!(size.width, 0.0);
        assert_eq!(size.height, 0.0);
    }
}