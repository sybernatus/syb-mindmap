use crate::layout::size::Size;

pub mod pos2;
pub mod size;
pub mod left_right_horizontal;
pub mod left_right_bottom;

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
    fn max(&self, other: Size) -> Self;
}

pub trait Layout {

    /// Divide the element vector in right & left trees
    /// # Arguments
    /// * `elements` - A mutable reference to a vector of nodes
    /// # Returns
    /// * `(right_tree, left_tree)` - A tuple of two vectors of mutable references to nodes
    fn divide_elements_tree<T>(elements: &mut Vec<T>) -> (Vec<&mut T>, Vec<&mut T>) {
        // divide the children into two trees
        let mut right_tree: Vec<&mut T> = Vec::new();
        let mut left_tree: Vec<&mut T> = Vec::new();
        for (index, child) in elements.iter_mut().enumerate() {
            match index {
                index if index % 2 == 0 => right_tree.push(child),
                _ => left_tree.push(child),
            }
        }
        (right_tree, left_tree)
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    pub struct Test {}
    impl Layout for Test {}
    #[test]
    fn test_position2d() {
        let mut vec1 = vec![0, 1, 2, 4, 5];
        let (right, left) = Test::divide_elements_tree(&mut vec1);
        assert_eq!(right.len(), 3);
        assert_eq!(left.len(), 2);
    }
}