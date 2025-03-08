use crate::{patterns::stripe::Stripe, tuple::{Color, Point}};

#[derive(Debug, Clone, PartialEq)]
pub enum Pattern {
    Stripe(Stripe),
    // Gradient(Box<GradientPattern>),
    // Ring(Box<RingPattern>),
    // Checkers(Box<CheckersPattern>),
    // TestPattern(Box<TestPattern>),
}

impl Pattern {
    pub fn stripe(a: Color, b: Color) -> Pattern {
        Pattern::Stripe(Stripe::new(a, b))
    }

    // pub fn gradient(a: Color, b: Color) -> Pattern {
    //     Pattern::Gradient(Box::new(GradientPattern::new(a, b)))
    // }

    // pub fn ring(a: Color, b: Color) -> Pattern {
    //     Pattern::Ring(Box::new(RingPattern::new(a, b)))
    // }

    // pub fn checkers(a: Color, b: Color) -> Pattern {
    //     Pattern::Checkers(Box::new(CheckersPattern::new(a, b)))
    // }

    // pub fn test_pattern() -> Pattern {
    //     Pattern::TestPattern(Box::new(TestPattern::new()))
    // }

    pub fn pattern_at(&self, point: Point) -> Color {
        match self {
            Pattern::Stripe(p) => p.stripe_at(point),
            // Pattern::Gradient(p) => p.pattern_at(point),
            // Pattern::Ring(p) => p.pattern_at(point),
            // Pattern::Checkers(p) => p.pattern_at(point),
            // Pattern::TestPattern(p) => p.pattern_at(point),
        }
    }

    pub fn pattern_at_shape(&self, shape: &crate::shape::Shape, world_point: Point) -> Color {
        match self {
            Pattern::Stripe(p) => p.stripe_at_shape(shape, world_point),
            // Pattern::Gradient(p) => p.pattern_at_shape(shape, world_point),
            // Pattern::Ring(p) => p.pattern_at_shape(shape, world_point),
            // Pattern::Checkers(p) => p.pattern_at_shape(shape, world_point),
            // Pattern::TestPattern(p) => p.pattern_at_shape(shape, world_point),
        }
    }

    pub fn get_transform(&self) -> &crate::matrix::Matrix {
        match self {
            Pattern::Stripe(p) => p.get_transform(),
            // Pattern::Gradient(p) => p.get_transform(),
            // Pattern::Ring(p) => p.get_transform(),
            // Pattern::Checkers(p) => p.get_transform(),
            // Pattern::TestPattern(p) => p.get_transform(),
        }
    }

    pub fn set_transform(&mut self, transform: crate::matrix::Matrix) {
        match self {
            Pattern::Stripe(p) => p.set_transform(transform),
            // Pattern::Gradient(p) => p.set_transform(transform),
            // Pattern::Ring(p) => p.set_transform(transform),
            // Pattern::Checkers(p) => p.set_transform(transform),
            // Pattern::TestPattern(p) => p.set_transform(transform),
        }
    }

    pub fn with_transform(&self, transform: crate::matrix::Matrix) -> Pattern {
        match self {
            Pattern::Stripe(p) => Pattern::Stripe(p.with_transform(transform)),
            // Pattern::Gradient(p) => Pattern::Gradient(p.with_transform(transform)),
            // Pattern::Ring(p) => Pattern::Ring(p.with_transform(transform)),
            // Pattern::Checkers(p) => Pattern::Checkers(p.with_transform(transform)),
            // Pattern::TestPattern(p) => Pattern::TestPattern(p.with_transform(transform)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{color::Colors, light::Light, material::Material, matrix::Matrix, transformation::Transformation, shape::Shape, tuple::Tuple};

    #[test]
    fn a_stripe_pattern_is_constant_in_y() {
        let pattern = Pattern::stripe(Color::white(), Color::black());
        assert_eq!(pattern.pattern_at(Tuple::point(0.0, 0.0, 0.0)), Color::white());
        assert_eq!(pattern.pattern_at(Tuple::point(0.0, 1.0, 0.0)), Color::white());
        assert_eq!(pattern.pattern_at(Tuple::point(0.0, 2.0, 0.0)), Color::white());
    }

    #[test]
    fn a_stripe_pattern_is_constant_in_z() {
        let pattern = Pattern::stripe(Color::white(), Color::black());
        assert_eq!(pattern.pattern_at(Tuple::point(0.0, 0.0, 0.0)), Color::white());
        assert_eq!(pattern.pattern_at(Tuple::point(0.0, 0.0, 1.0)), Color::white());
        assert_eq!(pattern.pattern_at(Tuple::point(0.0, 0.0, 2.0)), Color::white());
    }

    #[test]
    fn a_stripe_pattern_alternates_in_x() {
        let pattern = Pattern::stripe(Color::white(), Color::black());
        assert_eq!(pattern.pattern_at(Tuple::point(0.0, 0.0, 0.0)), Color::white());
        assert_eq!(pattern.pattern_at(Tuple::point(0.9, 0.0, 0.0)), Color::white());
        assert_eq!(pattern.pattern_at(Tuple::point(1.0, 0.0, 0.0)), Color::black());
        assert_eq!(pattern.pattern_at(Tuple::point(-0.1, 0.0, 0.0)), Color::black());
        assert_eq!(pattern.pattern_at(Tuple::point(-1.0, 0.0, 0.0)), Color::black());   
        assert_eq!(pattern.pattern_at(Tuple::point(-1.1, 0.0, 0.0)), Color::white());
    }

    #[test]
    fn lighting_with_a_pattern_applied() {
        let m = Material::new()
            .with_pattern(Pattern::stripe(Color::white(), Color::black()))
            .with_ambient(1.0)
            .with_diffuse(0.0)
            .with_specular(0.0);
        let eyev = Tuple::vector(0.0, 0.0, -1.0);
        let normalv = Tuple::vector(0.0, 0.0, -1.0);
        let light = Light::new(Tuple::point(0.0, 0.0, -10.0), Color::white());
        let c1 = m.lighting(&Shape::test_shape(), &light, Tuple::point(0.9, 0.0, 0.0), eyev, normalv, false);
        let c2 = m.lighting(&Shape::test_shape(), &light, Tuple::point(1.1, 0.0, 0.0), eyev, normalv, false);
        assert_eq!(c1, Color::white());
        assert_eq!(c2, Color::black());
    }

    #[test]
    fn stripes_with_an_object_transformation() {
        let object = Shape::sphere()
            .with_transform(Matrix::scaling(2.0, 2.0, 2.0));
        let pattern = Pattern::stripe(Color::white(), Color::black());
        let c = pattern.pattern_at_shape(&object, Tuple::point(1.5, 0.0, 0.0));
        assert_eq!(c, Color::white());
    }

    #[test]
    fn stripes_with_a_pattern_transformation() {
        let object = Shape::sphere();
        let pattern = Pattern::stripe(Color::white(), Color::black())
            .with_transform(Matrix::scaling(2.0, 2.0, 2.0));
        let c = pattern.pattern_at_shape(&object, Tuple::point(1.5, 0.0, 0.0));
        assert_eq!(c, Color::white());
    }
}