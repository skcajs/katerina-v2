use crate::ray::Ray;
use crate::shapes::cone::Cone;
use crate::shapes::cube::Cube;
use crate::shapes::cylinder::Cylinder;
use crate::shapes::plane::Plane;
use crate::shapes::sphere::Sphere;
use crate::shapes::test_shape::TestShape;
use crate::tuple::{Point, Vector};

#[derive(Debug, PartialEq, Clone)]
pub enum Shape {
    TestShape(TestShape),
    Sphere(Sphere),
    Plane(Plane),
    Cube(Cube),
    Cylinder(Cylinder),
    Cone(Cone),
}

impl Shape {
    pub fn local_intersect(&self, local_ray: &Ray) -> Vec<f64> {
        match self {
            Shape::TestShape(s) => s.local_intersect(&local_ray),
            Shape::Sphere(s) => s.local_intersect(&local_ray),
            Shape::Plane(s) => s.local_intersect(&local_ray),
            Shape::Cube(s) => s.local_intersect(&local_ray),
            Shape::Cylinder(s) => s.local_intersect(&local_ray),
            Shape::Cone(s) => s.local_intersect(&local_ray),
        }
    }

    pub fn local_normal_at(&self, local_point: &Point) -> Vector {
        match self {
            Shape::TestShape(s) => s.local_normal_at(&local_point),
            Shape::Sphere(s) => s.local_normal_at(&local_point),
            Shape::Plane(s) => s.local_normal_at(),
            Shape::Cube(s) => s.local_normal_at(&local_point),
            Shape::Cylinder(s) => s.local_normal_at(&local_point),
            Shape::Cone(s) => s.local_normal_at(&local_point),
        }
    }
}