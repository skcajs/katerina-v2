use crate::{matrix::Matrix, tuple::{Color, Point}};

#[derive(Debug, Clone, PartialEq)]
pub struct Ring {
    pub a: Color,
    pub b: Color,
    pub transform: Matrix,
}

impl Ring {
    pub fn new(a: Color, b: Color) -> Ring {
        Ring { a, b, transform: Matrix::identity() }
    }

    pub fn ring_at(&self, point: Point) -> Color {
        if (point.0 * point.0 + point.2 * point.2).sqrt().floor() as i32 % 2 == 0 {
            self.a
        } else {
            self.b
        }
    }

    pub fn ring_at_shape(&self, shape: &crate::shape::Shape, world_point: Point) -> Color {
        let object_point = shape.get_transform().inverse() * world_point;
        let pattern_point = self.get_transform().inverse() * object_point;
        self.ring_at(pattern_point)
    }

    pub fn get_transform(&self) -> &Matrix {
        &self.transform
    }

    pub fn set_transform(&mut self, transform: Matrix) {
        self.transform = transform;
    }

    pub fn with_transform(&self, transform: Matrix) -> Ring {
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
    fn a_ring_should_extend_in_both_x_and_z() {
        let pattern = Pattern::ring(Color::white(), Color::black());
        assert_eq!(pattern.pattern_at(Tuple::point(0.0, 0.0, 0.0)), Color::white());
        assert_eq!(pattern.pattern_at(Tuple::point(1.0, 0.0, 0.0)), Color::black());
        assert_eq!(pattern.pattern_at(Tuple::point(0.0, 0.0, 1.0)), Color::black());
        // 0.708 = just slightly more than √2/2
        assert_eq!(pattern.pattern_at(Tuple::point(0.708, 0.0, 0.708)), Color::black());
    }

}