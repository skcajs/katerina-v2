use crate::intersection::Intersection;
use crate::material::Material;
use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::tuple::{Point, Tuple};

#[derive(Debug, Clone, PartialEq)]
pub struct TestShape {
    transform: Matrix,
    material: Material,
}

impl TestShape {
    #[allow(dead_code)]
    pub fn new() -> Self {
        TestShape {
            transform: Matrix::identity(),
            material: Material::new(),
        }
    }

    pub fn local_intersect(&self, _ray: &Ray) -> Vec<Intersection> {
        
        vec![]
    }

    pub fn local_normal_at(&self, point: &Point) -> Tuple {
        Tuple::vector(point.0, point.1, point.2)
    }

    pub fn get_transform(&self) -> &Matrix {
        &self.transform
    }

    pub fn set_transform(&mut self, transform: Matrix) {
        self.transform = transform;
    }

    pub fn with_transform(&self, transform: Matrix) -> TestShape {
        let mut new_shape = self.clone();
        new_shape.set_transform(transform);
        new_shape
    }

    pub fn get_material(&self) -> &Material {
        &self.material
    }

    pub fn set_material(&mut self, material: Material) {
        self.material = material;
    }

    pub fn with_material(&self, material: Material) -> TestShape {
        let mut new_shape = self.clone();
        new_shape.set_material(material);
        new_shape
    }

}