use crate::intersection::Intersection;
use crate::tuple::{Point, Tuple};

#[derive(Debug, Clone, PartialEq)]
pub struct TestShape {}

impl TestShape {
    #[allow(dead_code)]
    pub fn new() -> Self {
        TestShape {
        }
    }

    pub fn local_intersect<'a>(&'a self) -> Vec<Intersection<'a>> {
        vec![]
    }

    pub fn local_normal_at(&self, point: &Point) -> Tuple {
        Tuple::vector(point.0, point.1, point.2)
    }

}