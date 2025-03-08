use crate::{matrix::Matrix, tuple::{Color, Point}};

#[derive(Debug, Clone, PartialEq)]
pub struct Gradient {
    pub a: Color,
    pub b: Color,
    pub transform: Matrix,
}

impl Gradient {
    pub fn new(a: Color, b: Color) -> Gradient {
        Gradient { a, b, transform: Matrix::identity() }
    }

    pub fn gradient_at(&self, point: Point) -> Color {
        let distance = self.b - self.a;
        let fraction = point.0 - point.0.floor();
        self.a + distance * fraction
    }

    pub fn gradient_at_shape(&self, shape: &crate::shape::Shape, world_point: Point) -> Color {
        let object_point = shape.get_transform().inverse() * world_point;
        let pattern_point = self.get_transform().inverse() * object_point;
        self.gradient_at(pattern_point)
    }

    pub fn get_transform(&self) -> &Matrix {
        &self.transform
    }

    pub fn set_transform(&mut self, transform: Matrix) {
        self.transform = transform;
    }

    pub fn with_transform(&self, transform: Matrix) -> Gradient {
        let mut new_stripe = self.clone();
        new_stripe.set_transform(transform);
        new_stripe
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{color::Colors, pattern::Pattern, tuple::Tuple};

    #[test]
    fn a_gradient_pattern_linearly_interpolates_between_colors() {
        let pattern = Pattern::gradient(Color::white(), Color::black());
        assert_eq!(pattern.pattern_at(Tuple::point(0.0, 0.0, 0.0)), Color::white());
        assert_eq!(pattern.pattern_at(Tuple::point(0.25, 0.0, 0.0)), Color::color(0.75, 0.75, 0.75));
        assert_eq!(pattern.pattern_at(Tuple::point(0.5, 0.0, 0.0)), Color::color(0.5, 0.5, 0.5));
        assert_eq!(pattern.pattern_at(Tuple::point(0.75, 0.0, 0.0)), Color::color(0.25, 0.25, 0.25));
    }

}