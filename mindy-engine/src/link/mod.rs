use serde::{Deserialize, Serialize};
use crate::utils::pos2::Pos2;
use crate::utils::rgb::Rgb;

#[derive(Debug, Clone, PartialEq, Deserialize, Default, Serialize)]
pub struct Link {
    pub pos_start: Pos2,
    pub pos_end: Pos2,
    pub path_data: Option<String>,
    pub color: Option<Rgb>,
    pub stroke_width: Option<f32>,
}

impl Link {

    /// Create a new link with default values
    pub fn new() -> Self {
        Self {
            pos_start: Default::default(),
            pos_end: Default::default(),
            path_data: None,
            color: None,
            stroke_width: None,
        }
    }

    /// Create a new link with start and end positions
    pub fn from_start_end(
        pos_start: Pos2,
        pos_end: Pos2,
    ) -> Self {
        Self {
            pos_start,
            pos_end,
            path_data: None,
            color: None,
            stroke_width: None,
        }
    }

    /// Get the metadata of the link
    /// direction_x: the link horizontal direction
    /// direction_y: the link vertical direction
    /// distance: the distance between the start and end position
    /// normal_x: the normal vector of the link in the x direction
    /// normal_y: the normal vector of the link in the y direction
    pub fn get_link_metadata(self) -> (f32, f32, f32, f32, f32) {
        let direction_x = self.pos_end.x - self.pos_start.x;
        let direction_y = self.pos_end.y - self.pos_start.y;

        let distance = (direction_x.powi(2) + direction_y.powi(2)).sqrt();

        let normal_x = -direction_y / distance;
        let normal_y = direction_x / distance;
        (direction_x, direction_y, distance, normal_x, normal_y)
    }

    /// Computes the control points for a cubic Bézier curve between two points.
    /// The control points are offset from the line connecting the two points
    /// by a specified ratio.
    pub fn compute_control_points_offset(self, offset_ratio: f32) -> (Pos2, Pos2) {
        let (direction_x, direction_y, distance, normal_x, normal_y) =
            self.clone().get_link_metadata();

        let control_point1 = Pos2 {
            x: self.pos_start.x + direction_x / 6.0 - normal_x * distance * offset_ratio,
            y: self.pos_start.y + direction_y / 6.0 - normal_y * distance * offset_ratio,
        };

        let control_point2 = Pos2 {
            x: self.pos_start.x + direction_x / 3.0 + normal_x * distance * offset_ratio,
            y: self.pos_start.y + direction_y / 3.0 + normal_y * distance * offset_ratio,
        };

        (control_point1, control_point2)
    }

    /// Calculates the path data for a cubic Bézier curve between the start & end position of the link.
    /// The curve is amplified by a specified offset ratio.
    pub fn with_path_data_bezier(mut self, offset_ratio: f32) -> Self {
        let (c1, c2) = self.clone().compute_control_points_offset(offset_ratio);
        self.path_data = Some(format!(
            "M{:.2},{:.2} C{:.2},{:.2} {:.2},{:.2} {:.2},{:.2}",
            self.pos_start.x,
            self.pos_start.y,
            c1.x,
            c1.y,
            c2.x,
            c2.y,
            self.pos_end.x,
            self.pos_end.y
        ));
        self
    }
}
