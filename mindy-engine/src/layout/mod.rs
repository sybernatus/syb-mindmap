pub mod pos2;
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