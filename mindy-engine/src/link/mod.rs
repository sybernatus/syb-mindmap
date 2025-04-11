use crate::utils::pos2::Pos2;

pub fn get_link_metadata(
    start: Pos2,
    end: Pos2
) -> (f32, f32, f32, f32, f32) {
    let direction_x = end.x - start.x;
    let direction_y = end.y - start.y;

    let distance = (direction_x.powi(2) + direction_y.powi(2)).sqrt();

    let normal_x = -direction_y / distance;
    let normal_y = direction_x / distance;
    ( direction_x, direction_y, distance, normal_x, normal_y)
}

pub fn compute_control_points_offset(
    start: Pos2,
    end: Pos2,
    offset_ratio: f32,
) -> (Pos2, Pos2) {

    let (direction_x, direction_y, distance, normal_x, normal_y) = get_link_metadata(start.clone(), end.clone());

    let control_point1 = Pos2 {
        x: start.x + direction_x / 3.0 + normal_x * distance * offset_ratio,
        y: start.y + direction_y / 3.0 + normal_y * distance * offset_ratio,
    };

    let control_point2 = Pos2 {
        x: start.x + direction_x / 6.0 + normal_x * distance * -offset_ratio,
        y: start.y + direction_y / 6.0 + normal_y * distance * -offset_ratio,
    };

    (control_point1, control_point2)
}

pub fn bezier_svg_path(start: Pos2, end: Pos2, offset_ratio: f32) -> String {
    let (c1, c2) = compute_control_points_offset(start.clone(), end.clone(), offset_ratio);
    format!(
        "M{:.2},{:.2} C{:.2},{:.2} {:.2},{:.2} {:.2},{:.2}",
        start.x, start.y,
        c1.x, c1.y,
        c2.x, c2.y,
        end.x, end.y
    )
}