use crate::{matrix::Matrix, object::Object, patterns::{checkers::Checkers, gradient::Gradient, ring::Ring, stripe::Stripe, test_pattern::TestPattern}, tuple::{Color, Point}};

#[derive(Debug, Clone, PartialEq)]
pub enum PatternEnum {
    TestPattern(TestPattern),
    Stripe(Stripe),
    Gradient(Gradient),
    Ring(Ring),
    Checkers(Checkers),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Pattern {
    pub pattern: PatternEnum,
    pub transform: Matrix,
}

impl Pattern {

    pub fn test_pattern() -> Pattern {
        Pattern {
            pattern: PatternEnum::TestPattern(TestPattern::new()),
            transform: Matrix::identity(),
        }
    }

    pub fn stripe(a: Color, b: Color) -> Pattern {

        Pattern {
            pattern: PatternEnum::Stripe(Stripe::new(a, b)),
            transform: Matrix::identity(),
        }
    }

    pub fn gradient(a: Color, b: Color) -> Pattern {
        Pattern {
            pattern: PatternEnum::Gradient(Gradient::new(a, b)),
            transform: Matrix::identity(),
        }
    }

    pub fn ring(a: Color, b: Color) -> Pattern {
        Pattern {
            pattern: PatternEnum::Ring(Ring::new(a, b)),
            transform: Matrix::identity(),
        }
    }

    pub fn checkers(a: Color, b: Color) -> Pattern {
        Pattern {
            pattern: PatternEnum::Checkers(Checkers::new(a, b)),
            transform: Matrix::identity(),
        }
    }

    pub fn pattern_at(&self, point: Point) -> Color {
        match &self.pattern {
            PatternEnum::TestPattern(p) => p.test_pattern_at(point),
            PatternEnum::Stripe(p) => p.stripe_at(point),
            PatternEnum::Gradient(p) => p.gradient_at(point),
            PatternEnum::Ring(p) => p.ring_at(point),
            PatternEnum::Checkers(p) => p.checkers_at(point),
        }
    }

    pub fn pattern_at_shape(&self, shape: &Object, world_point: Point) -> Color {
        let object_point = shape.get_transform().inverse() * world_point;
        let pattern_point = self.get_transform().inverse() * object_point;
        self.pattern_at(pattern_point)
    }

    pub fn get_transform(&self) -> &Matrix {
        &self.transform
    }

    pub fn set_transform(&mut self, transform: Matrix) {
        self.transform = transform;
    }

    pub fn with_transform(&self, transform: Matrix) -> Pattern {
        let mut new_pattern = self.clone();
        new_pattern.set_transform(transform);
        new_pattern
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{matrix::Matrix, object::Object, transformation::Transformation, tuple::Tuple};

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
        let obj = Object::sphere()
            .with_transform(Matrix::scaling(2.0, 2.0, 2.0));
        let pattern = Pattern::test_pattern();
        let c = pattern.pattern_at_shape(&obj, Tuple::point(2.0, 3.0, 4.0));
        assert_eq!(c, Tuple::color(1., 1.5, 2.));
    }

    #[test]
    fn a_pattern_with_both_an_object_and_a_pattern_transformation() {
        let obj = Object::sphere()
            .with_transform(Matrix::scaling(2.0, 2.0, 2.0));
        let pattern = Pattern::test_pattern()
            .with_transform(Matrix::translation(0.5,1., 1.5));
        let c = pattern.pattern_at_shape(&obj, Tuple::point(2.5, 3.0, 3.5));
        assert_eq!(c, Tuple::color(0.75, 0.5, 0.25));
    }
}
