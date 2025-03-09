use crate::tuple::{Color, Point};

#[derive(Debug, Clone, PartialEq)]
pub struct Gradient {
    pub a: Color,
    pub b: Color,
}

impl Gradient {
    pub fn new(a: Color, b: Color) -> Gradient {
        Gradient { a, b }
    }

    pub fn gradient_at(&self, point: Point) -> Color {
        let distance = self.b - self.a;
        let fraction = point.0 - point.0.floor();
        self.a + distance * fraction
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