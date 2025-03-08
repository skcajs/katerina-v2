use crate::{matrix::Matrix, tuple::{Color, Point}};

#[derive(Debug, Clone, PartialEq)]
pub struct Stripe {
    pub a: Color,
    pub b: Color,
    pub transform: Matrix,
}

impl Stripe {
    pub fn new(a: Color, b: Color) -> Stripe {
        Stripe { a, b, transform: Matrix::identity() }
    }

    pub fn stripe_at(&self, point: Point) -> Color {
        if point.0.floor() as i32 % 2 == 0 {
            self.a
        } else {
            self.b
        }
    }

    pub fn stripe_at_shape(&self, shape: &crate::shape::Shape, world_point: Point) -> Color {
        let object_point = shape.get_transform().inverse() * world_point;
        let pattern_point = self.get_transform().inverse() * object_point;
        self.stripe_at(pattern_point)
    }

    pub fn get_transform(&self) -> &Matrix {
        &self.transform
    }

    pub fn set_transform(&mut self, transform: Matrix) {
        self.transform = transform;
    }

    pub fn with_transform(&self, transform: Matrix) -> Stripe {
        let mut new_stripe = self.clone();
        new_stripe.set_transform(transform);
        new_stripe
    }
}