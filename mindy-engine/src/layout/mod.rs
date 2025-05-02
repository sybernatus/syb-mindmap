pub mod pos2;
pub mod size;
pub mod left_right_horizontal;

pub trait Position2D {
    fn x(&self) -> f32;
    fn y(&self) -> f32;
    fn x_mut(&mut self) -> &mut f32;
    fn y_mut(&mut self) -> &mut f32;

    fn add(&mut self, other: &Self) -> &mut Self {
        *self.x_mut() += other.x();
        *self.y_mut() += other.y();
        self
    }

    fn subtract(&mut self, other: &Self) -> &mut Self {
        *self.x_mut() -= other.x();
        *self.y_mut() -= other.y();
        self
    }

}

pub trait Size2D {

    fn new(width: f32, height: f32) -> Self;
    fn width(&self) -> f32;
    fn height(&self) -> f32;
    fn width_mut(&mut self) -> &mut f32;
    fn height_mut(&mut self) -> &mut f32;

}