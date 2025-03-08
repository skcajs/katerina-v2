use crate::{matrix::Matrix, tuple::{Color, Point, Tuple}};

#[derive(Debug, Clone, PartialEq)]
pub struct TestPattern {
    pub transform: Matrix,
}

impl TestPattern {
    pub fn new() -> TestPattern {
        TestPattern { transform: Matrix::identity() }
    }

    pub fn test_pattern_at(&self, point: Point) -> Color {
        Tuple::color(point.0, point.1, point.2)
    }

    pub fn test_pattern_at_shape(&self, shape: &crate::shape::Shape, world_point: Point) -> Color {
        let object_point = shape.get_transform().inverse() * world_point;
        let pattern_point = self.get_transform().inverse() * object_point;
        self.test_pattern_at(pattern_point)
    }

    pub fn get_transform(&self) -> &Matrix {
        &self.transform
    }

    pub fn set_transform(&mut self, transform: Matrix) {
        self.transform = transform;
    }

    pub fn with_transform(&self, transform: Matrix) -> TestPattern {
        let mut new_stripe = self.clone();
        new_stripe.set_transform(transform);
        new_stripe
    }
}