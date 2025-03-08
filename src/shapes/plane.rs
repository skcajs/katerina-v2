use crate::{intersection::Intersection, material::Material, matrix::Matrix, ray::Ray, shape::Shape, tuple::Tuple};

#[derive(Debug, Clone, PartialEq)]
pub struct Plane {
    transform: Matrix,
    material: Material,
    normal: Tuple,
    distance: f32,
}

impl Plane {
    pub fn new() -> Plane {
        Plane {
            normal: Tuple::vector(0.0, 1.0, 0.0),
            distance: 0.0,
            transform: Matrix::identity(),
            material: Material::new(),
        }
    }

    pub fn local_intersect(&self, ray: &Ray) -> Vec<Intersection> {
        if ray.direction.1.abs() < 1e-5 {
            return vec![];
        }

        let t = -ray.origin.1 / ray.direction.1;
        vec![Intersection::new(t, Shape::Plane(self.clone()))]
    }

    pub fn local_normal_at(&self) -> Tuple {
        self.normal
    }

    pub fn get_transform(&self) -> &Matrix {
        &self.transform
    }

    pub fn set_transform(&mut self, transform: Matrix) {
        self.transform = transform;
    }

    pub fn with_transform(&self, transform: Matrix) -> Plane {
        let mut new_sphere = self.clone();
        new_sphere.set_transform(transform);
        new_sphere
    }

    pub fn get_material(&self) -> &Material {
        &self.material
    }

    pub fn set_material(&mut self, material: Material) {
        self.material = material;
    }

    pub fn with_material(&self, material: Material) -> Plane {
        let mut new_sphere = self.clone();
        new_sphere.set_material(material);
        new_sphere
    }
}

#[cfg(test)]
mod tests {
    use crate::{ray::Ray, tuple::Tuple};

    use super::*;

    #[test]
    fn the_normal_of_a_plane_is_constant_everywhere() {
        let p = Shape::plane();
        let n1 = p.normal_at(Tuple::point(0.0, 0.0, 0.0));
        let n2 = p.normal_at(Tuple::point(10.0, 0.0, -10.0));
        let n3 = p.normal_at(Tuple::point(-5.0, 0.0, 150.0));
        assert_eq!(n1, Tuple::vector(0.0, 1.0, 0.0));
        assert_eq!(n2, Tuple::vector(0.0, 1.0, 0.0));
        assert_eq!(n3, Tuple::vector(0.0, 1.0, 0.0));
    }

    #[test]
    fn intersect_with_a_ray_parallel_to_the_plane() {
        let p = Shape::plane();
        let r = Ray::new(Tuple::point(0.0, 10.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
        let xs = p.intersect(&r);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn intersect_with_a_coplanar_ray() {
        let p = Shape::plane();
        let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
        let xs = p.intersect(&r);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn a_ray_intersecting_a_plane_from_above() {
        let p = Shape::plane();
        let r = Ray::new(Tuple::point(0.0, 1.0, 0.0), Tuple::vector(0.0, -1.0, 0.0));
        let xs = p.intersect(&r);
        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0].t, 1.0);
        assert_eq!(xs[0].object, p.clone());
    }

    #[test]
    fn a_ray_intersecting_a_plane_from_below() {
        let p = Shape::plane();
        let r = Ray::new(Tuple::point(0.0, -1.0, 0.0), Tuple::vector(0.0, 1.0, 0.0));
        let xs = p.intersect(&r);
        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0].t, 1.0);
        assert_eq!(xs[0].object, p.clone());
    }
}