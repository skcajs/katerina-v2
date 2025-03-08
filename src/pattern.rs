use crate::{patterns::{checkers::Checkers, gradient::Gradient, ring::Ring, stripe::Stripe, test_pattern::TestPattern}, tuple::{Color, Point}};

#[derive(Debug, Clone, PartialEq)]
pub enum Pattern {
    TestPattern(TestPattern),
    Stripe(Stripe),
    Gradient(Gradient),
    Ring(Ring),
    Checkers(Checkers),
}

impl Pattern {

    pub fn test_pattern() -> Pattern {
        Pattern::TestPattern(TestPattern::new())
    }

    pub fn stripe(a: Color, b: Color) -> Pattern {
        Pattern::Stripe(Stripe::new(a, b))
    }

    pub fn gradient(a: Color, b: Color) -> Pattern {
        Pattern::Gradient(Gradient::new(a, b))
    }

    pub fn ring(a: Color, b: Color) -> Pattern {
        Pattern::Ring(Ring::new(a, b))
    }

    pub fn checkers(a: Color, b: Color) -> Pattern {
        Pattern::Checkers(Checkers::new(a, b))
    }

    pub fn pattern_at(&self, point: Point) -> Color {
        match self {
            Pattern::TestPattern(p) => p.test_pattern_at(point),
            Pattern::Stripe(p) => p.stripe_at(point),
            Pattern::Gradient(p) => p.gradient_at(point),
            Pattern::Ring(p) => p.ring_at(point),
            Pattern::Checkers(p) => p.checkers_at(point),
        }
    }

    pub fn pattern_at_shape(&self, shape: &crate::shape::Shape, world_point: Point) -> Color {
        match self {
            Pattern::TestPattern(p) => p.test_pattern_at_shape(shape, world_point),
            Pattern::Stripe(p) => p.stripe_at_shape(shape, world_point),
            Pattern::Gradient(p) => p.gradient_at_shape(shape, world_point),
            Pattern::Ring(p) => p.ring_at_shape(shape, world_point),
            Pattern::Checkers(p) => p.checkers_at_shape(shape, world_point),
        }
    }

    pub fn get_transform(&self) -> &crate::matrix::Matrix {
        match self {
            Pattern::TestPattern(p) => p.get_transform(),
            Pattern::Stripe(p) => p.get_transform(),
            Pattern::Gradient(p) => p.get_transform(),
            Pattern::Ring(p) => p.get_transform(),
            Pattern::Checkers(p) => p.get_transform(),
        }
    }

    pub fn set_transform(&mut self, transform: crate::matrix::Matrix) {
        match self {
            Pattern::TestPattern(p) => p.set_transform(transform),
            Pattern::Stripe(p) => p.set_transform(transform),
            Pattern::Gradient(p) => p.set_transform(transform),
            Pattern::Ring(p) => p.set_transform(transform),
            Pattern::Checkers(p) => p.set_transform(transform),
        }
    }

    pub fn with_transform(&self, transform: crate::matrix::Matrix) -> Pattern {
        match self {
            Pattern::TestPattern(p) => Pattern::TestPattern(p.with_transform(transform)),
            Pattern::Stripe(p) => Pattern::Stripe(p.with_transform(transform)),
            Pattern::Gradient(p) => Pattern::Gradient(p.with_transform(transform)),
            Pattern::Ring(p) => Pattern::Ring(p.with_transform(transform)),
            Pattern::Checkers(p) => Pattern::Checkers(p.with_transform(transform)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{matrix::Matrix, shape::Shape, transformation::Transformation, tuple::Tuple};

    #[test]
    fn the_default_pattern_transformation() {
        let pattern = Pattern::test_pattern();
        assert_eq!(pattern.get_transform(), &Matrix::identity());
    }

    #[test]
    fn assigning_a_transformation() {
        let mut pattern = Pattern::test_pattern();
        pattern.set_transform(Matrix::translation(1.0, 2.0, 3.0));
        assert_eq!(pattern.get_transform(), &Matrix::translation(1.0, 2.0, 3.0));
    }

    #[test]
    fn a_pattern_with_an_object_transformation() {
        let shape = Shape::sphere()
            .with_transform(Matrix::scaling(2.0, 2.0, 2.0));
        let pattern = Pattern::test_pattern();
        let c = pattern.pattern_at_shape(&shape, Tuple::point(2.0, 3.0, 4.0));
        assert_eq!(c, Tuple::color(1., 1.5, 2.));
    }

    #[test]
    fn a_pattern_with_both_an_object_and_a_pattern_transformation() {
        let shape = Shape::sphere()
            .with_transform(Matrix::scaling(2.0, 2.0, 2.0));
        let pattern = Pattern::test_pattern()
            .with_transform(Matrix::translation(0.5,1., 1.5));
        let c = pattern.pattern_at_shape(&shape, Tuple::point(2.5, 3.0, 3.5));
        assert_eq!(c, Tuple::color(0.75, 0.5, 0.25));
    }
}
