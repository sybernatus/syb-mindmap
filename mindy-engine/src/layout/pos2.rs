use serde::{Deserialize, Serialize};
use crate::layout::Position2D;

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
}

impl Position2D for Pos2 {
    fn x(&self) -> f32 {
        self.x
    }

    fn y(&self) -> f32 {
        self.y
    }

    fn x_mut(&mut self) -> &mut f32 {
        &mut self.x
    }

    fn y_mut(&mut self) -> &mut f32 {
        &mut self.y
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pos2() {
        let pos = Pos2::new(10.0, 20.0);
        assert_eq!(pos.x, 10.0);
        assert_eq!(pos.y, 20.0);
    }

    #[test]
    fn test_pos2_default() {
        let pos: Pos2 = Default::default();
        assert_eq!(pos.x, 0.0);
        assert_eq!(pos.y, 0.0);
    }
}