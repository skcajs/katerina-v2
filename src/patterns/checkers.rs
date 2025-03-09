use crate::tuple::{Color, Point};

#[derive(Debug, Clone, PartialEq)]
pub struct Checkers {
    pub a: Color,
    pub b: Color,
}

impl Checkers {
    pub fn new(a: Color, b: Color) -> Checkers {
        Checkers { a, b }
    }

    pub fn checkers_at(&self, point: Point) -> Color {
        if (point.0.floor() as i32 + point.1.floor() as i32 + point.2.floor() as i32) % 2 == 0 {
            self.a
        } else {
            self.b
        }
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