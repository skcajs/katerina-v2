use crate::{matrix::Matrix, tuple::{Color, Point}};

#[derive(Debug, Clone, PartialEq)]
pub struct Checkers {
    pub a: Color,
    pub b: Color,
    pub transform: Matrix,
}

impl Checkers {
    pub fn new(a: Color, b: Color) -> Checkers {
        Checkers { a, b, transform: Matrix::identity() }
    }

    pub fn checkers_at(&self, point: Point) -> Color {
        if (point.0.floor() as i32 + point.1.floor() as i32 + point.2.floor() as i32) % 2 == 0 {
            self.a
        } else {
            self.b
        }
    }

    pub fn checkers_at_shape(&self, shape: &crate::shape::Shape, world_point: Point) -> Color {
        let object_point = shape.get_transform().inverse() * world_point;
        let pattern_point = self.get_transform().inverse() * object_point;
        self.checkers_at(pattern_point)
    }

    pub fn get_transform(&self) -> &Matrix {
        &self.transform
    }

    pub fn set_transform(&mut self, transform: Matrix) {
        self.transform = transform;
    }

    pub fn with_transform(&self, transform: Matrix) -> Checkers {
        let mut new_checkers = self.clone();
        new_checkers.set_transform(transform);
        new_checkers
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{color::Colors, pattern::Pattern, tuple::Tuple};

    #[test]
    fn checkers_should_repeat_in_x() {
        let pattern = Pattern::checkers(Color::white(), Color::black());
        assert_eq!(pattern.pattern_at(Tuple::point(0.0, 0.0, 0.0)), Color::white());
        assert_eq!(pattern.pattern_at(Tuple::point(0.99, 0.0, 0.0)), Color::white());
        assert_eq!(pattern.pattern_at(Tuple::point(1.01, 0.0, 0.0)), Color::black());
    }

    #[test]
    fn checkers_should_repeat_in_y() {
        let pattern = Pattern::checkers(Color::white(), Color::black());
        assert_eq!(pattern.pattern_at(Tuple::point(0.0, 0.0, 0.0)), Color::white());
        assert_eq!(pattern.pattern_at(Tuple::point(0.0, 0.99, 0.0)), Color::white());
        assert_eq!(pattern.pattern_at(Tuple::point(0.0, 1.01, 0.0)), Color::black());
    }

    #[test]
    fn checkers_should_repeat_in_z() {
        let pattern = Pattern::checkers(Color::white(), Color::black());
        assert_eq!(pattern.pattern_at(Tuple::point(0.0, 0.0, 0.0)), Color::white());
        assert_eq!(pattern.pattern_at(Tuple::point(0.0, 0.0, 0.99)), Color::white());
        assert_eq!(pattern.pattern_at(Tuple::point(0.0, 0.0, 1.01)), Color::black());
    }
}