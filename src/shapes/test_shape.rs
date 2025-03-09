use crate::ray::Ray;
use crate::tuple::{Point, Tuple};

#[derive(Debug, Clone, PartialEq)]
pub struct TestShape {}

impl TestShape {
    #[allow(dead_code)]
    pub fn new() -> Self {
        TestShape {
        }
    }

    pub fn local_intersect(&self, _ray: &Ray) -> Vec<f64> {
        vec![]
    }

    pub fn local_normal_at(&self, point: &Point) -> Tuple {
        Tuple::vector(point.0, point.1, point.2)
    }

}