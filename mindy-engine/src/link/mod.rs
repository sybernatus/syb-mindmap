use serde::{Deserialize, Serialize};
use crate::layout::pos2::Pos2;
use crate::layout::size::Size;
use crate::utils::color::Color;

#[derive(Debug, Clone, PartialEq, Deserialize, Default, Serialize)]
pub struct Link {
    pub pos_start: Pos2,
    pub pos_end: Pos2,
    pub size_start: Size,
    pub size_end: Size,
    pub path_data: Option<String>,
    pub color: Option<Color>,
    pub stroke_width: Option<f32>,
}

impl Link {

    /// Create a new link with default values
    pub fn new() -> Self {
        Self {
            pos_start: Default::default(),
            pos_end: Default::default(),
            size_start: Default::default(),
            size_end: Default::default(),
            path_data: None,
            color: None,
            stroke_width: None,
        }
    }

    /// Create a new link with start and end positions
    /// # Arguments
    /// * `pos_start` - The starting position of the link
    /// * `pos_end` - The ending position of the link
    /// * `size_start` - The size of the starting node
    /// * `size_end` - The size of the ending node
    /// # Returns
    /// * `Self` - A new instance of the Link struct
    pub fn from_start_end(
        pos_start: Pos2,
        pos_end: Pos2,
        size_start: Size,
        size_end: Size,
    ) -> Self {
        Self {
            pos_start,
            pos_end,
            size_start,
            size_end,
            path_data: None,
            color: None,
            stroke_width: None,
        }
    }

    /// Get the metadata of the link
    /// # Returns
    /// * `direction_x` - the link horizontal direction
    /// * `direction_y` - the link vertical direction
    /// * `distance` - the distance between the start and end position
    /// * `normal_x` - the normal vector of the link in the x direction
    /// * `normal_y` - the normal vector of the link in the y direction
    pub fn get_link_metadata(self) -> (f32, f32, f32, f32, f32) {
        let direction_x = self.pos_end.x - self.pos_start.x;
        let direction_y = self.pos_end.y - self.pos_start.y;

        let distance = (direction_x.powi(2) + direction_y.powi(2)).sqrt();

        let normal_x = -direction_y / distance;
        let normal_y = direction_x / distance;
        (direction_x, direction_y, distance, normal_x, normal_y)
    }

    /// Computes the control points for a cubic Bézier curve between two points.
    /// The offset control the bézier curve's shape.
    /// # Arguments
    /// * `offset_ratio` - The ratio by which to offset the control points from the line.
    /// # Returns
    /// * `(control_point0, control_point1, control_point2, control_point3, control_point4, control_point5)` - The computed control points.
    pub fn compute_control_points_offset(self, offset_ratio: f32) -> (Pos2, Pos2, Pos2, Pos2, Pos2, Pos2) {
        let (direction_x, _, distance, ..) =
            self.clone().get_link_metadata();

        let control_point0 = Pos2 {
            x: self.pos_start.x,
            y: self.pos_start.y,
        };

        let c1x = self.pos_start.x + (self.size_start.width / 2.0 * direction_x.signum());
        let control_point1 = Pos2 {
            x: c1x,
            y: self.pos_start.y,
        };

        let control_point2 = Pos2 {
            x: self.pos_start.x + ((self.size_start.width / 2.0 + distance / 6.0 * offset_ratio) * direction_x.signum() ),
            y: self.pos_start.y,
        };

        let control_point3 = Pos2 {
            x: self.pos_end.x + ((self.size_end.width / 2.0 + distance / 6.0 * offset_ratio) * -direction_x.signum()),
            y: self.pos_end.y,
        };

        let c4x = self.pos_end.x + (self.size_end.width / 2.0 * -direction_x.signum()) ;
        let control_point4 = Pos2 {
            x: c4x,
            y: self.pos_end.y,
        };

        let control_point5 = Pos2 {
            x: self.pos_end.x,
            y: self.pos_end.y,
        };

        (control_point0, control_point1, control_point2, control_point3, control_point4, control_point5)
    }

    /// Calculates the path data for a cubic Bézier curve between the start & end position of the link.
    /// The curve is amplified by a specified offset ratio.
    /// # Arguments
    /// * `offset_ratio` - The ratio by which to amplify the curve.
    /// # Returns
    /// * `self` - The updated link with the computed path data.
    pub fn with_path_data_bezier(mut self, offset_ratio: f32) -> Self {
        let (c0, c1, c2, c3, c4, c5) = self.clone().compute_control_points_offset(offset_ratio);
        self.path_data = Some(format!(
            "M{:.2},{:.2} L{:.2},{:.2} C{:.2},{:.2} {:.2},{:.2} {:.2},{:.2} L{:.2},{:.2}",
            c0.x,
            c0.y,
            c1.x,
            c1.y,
            c2.x,
            c2.y,
            c3.x,
            c3.y,
            c4.x,
            c4.y,
            c5.x,
            c5.y
        ));
        self
    }
}
