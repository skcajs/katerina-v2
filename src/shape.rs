use crate::ray::Ray;
use crate::shapes::cube::Cube;
use crate::shapes::plane::Plane;
use crate::shapes::sphere::Sphere;
use crate::shapes::test_shape::TestShape;
use crate::tuple::{Point, Vector};

#[derive(Debug, PartialEq, Clone)]
pub enum Shape {
    Sphere(Sphere),
    TestShape(TestShape),
    Plane(Plane),
    Cube(Cube),
}

impl Shape {
    pub fn local_intersect(&self, local_ray: &Ray) -> Vec<f64> {
        match self {
            Shape::Sphere(s) => s.local_intersect(&local_ray),
            Shape::TestShape(s) => s.local_intersect(&local_ray),
            Shape::Plane(s) => s.local_intersect(&local_ray),
            Shape::Cube(s) => s.local_intersect(&local_ray),
        }
    }

    pub fn local_normal_at(&self, local_point: &Point) -> Vector {
        match self {
            Shape::Sphere(s) => s.local_normal_at(&local_point),
            Shape::TestShape(s) => s.local_normal_at(&local_point),
            Shape::Plane(s) => s.local_normal_at(),
            Shape::Cube(s) => s.local_normal_at(&local_point),
        }
    }
}