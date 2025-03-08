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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{color::Colors, light::Light, material::Material, matrix::Matrix, pattern::Pattern, shape::Shape, transformation::Transformation, tuple::Tuple};

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