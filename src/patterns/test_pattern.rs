use crate::tuple::{Color, Point, Tuple};

#[derive(Debug, Clone, PartialEq)]
pub struct TestPattern {}

impl TestPattern {
    pub fn new() -> TestPattern {
        TestPattern {  }
    }

    pub fn test_pattern_at(&self, point: Point) -> Color {
        Tuple::color(point.0, point.1, point.2)
    }
}