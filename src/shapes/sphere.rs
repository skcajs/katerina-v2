use crate::intersection::Intersection;
use crate::matrix::Matrix;
use crate::shape::Shape;
use crate::tuple::Tuple;
use crate::ray::Ray;
use crate::material::Material;


#[derive(Clone, PartialEq, Debug)]
pub struct Sphere {
    origin: Tuple,
    radius: f64,
    transform: Matrix,
    material: Material,
}

impl Sphere {
    pub fn new() -> Sphere {
        Sphere {
            origin: Tuple::point(0.0, 0.0, 0.0),
            radius: 1.,
            transform: Matrix::identity(),
            material: Material::new()
        }
    }

    pub fn local_intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let sphere_to_ray = ray.origin - Tuple::point(0.0, 0.0, 0.0);
        let a = ray.direction.dot(ray.direction);
        let b = 2.0 * ray.direction.dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;
        let discriminant = b.powi(2) - 4.0 * a * c;

        if discriminant < 0.0 {
            return vec![];
        }

        let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b + discriminant.sqrt()) / (2.0 * a);
        vec![
            Intersection::new(t1, Shape::Sphere(self.clone())), 
            Intersection::new(t2, Shape::Sphere(self.clone()))
            ]
    }

    pub fn local_normal_at(&self, point: &Tuple) -> Tuple {
        *point - Tuple::point(0.0, 0.0, 0.0)
    }

    pub fn get_transform(&self) -> &Matrix {
        &self.transform
    }

    pub fn set_transform(&mut self, transform: Matrix) {
        self.transform = transform;
    }

    pub fn with_transform(&self, transform: Matrix) -> Sphere {
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

    pub fn with_material(&self, material: Material) -> Sphere {
        let mut new_sphere = self.clone();
        new_sphere.set_material(material);
        new_sphere
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::transformation::Transformation;

    #[test]
    fn a_ray_intersects_a_sphere_at_two_points() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.local_intersect(&r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 6.0);
    }

    #[test]
    fn a_ray_misses_a_sphere() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 1.0, 0.0));
        let s = Sphere::new();
        let xs = s.local_intersect(&r);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn a_ray_originates_inside_a_sphere() {
        let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.local_intersect(&r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -1.0);
        assert_eq!(xs[1].t, 1.0);
    }

    #[test]
    fn a_sphere_is_behind_a_ray() {
        let r = Ray::new(Tuple::point(0.0, 0.0, 5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.local_intersect(&r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -6.0);
        assert_eq!(xs[1].t, -4.0);
    }

    #[test]
    fn intersect_sets_the_object_on_the_intersection() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.local_intersect(&r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].object, Shape::Sphere(s.clone()));
        assert_eq!(xs[1].object, Shape::Sphere(s.clone()));
    }

    #[test]
    fn a_sphere_default_transformation() {
        let s = Sphere::new();
        assert_eq!(s.transform, Matrix::identity());
    }

    #[test]
    fn changing_a_sphere_transformation() {
        let mut s = Sphere::new();
        let t = Matrix::translation(2.0, 3.0, 4.0);
        s.set_transform(t.clone());
        assert_eq!(s.transform, t);
    }

    #[test]
    fn intersecting_a_scaled_sphere_with_a_ray() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let mut s = Shape::sphere();
        s.set_transform(Matrix::scaling(2.0, 2.0, 2.0));
        let xs = s.intersect(&r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 3.0);
        assert_eq!(xs[1].t, 7.0);
    }

    #[test]
    fn intersecting_a_translated_sphere_with_a_ray() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let mut s = Shape::sphere();
        s.set_transform(Matrix::translation(5.0, 0.0, 0.0));
        let xs = s.intersect(&r);
        assert_eq!(xs.len(), 0);
    }

    // #[test]
    // fn the_normal_on_a_sphere_at_a_point_on_the_x_axis() {
    //     let s = Shape::sphere();
    //     let n = s.intersect(Tuple::point(1.0, 0.0, 0.0));
    //     assert_eq!(n, Tuple::vector(1.0, 0.0, 0.0));
    // }

    // #[test]
    // fn the_normal_on_a_sphere_at_a_point_on_the_y_axis() {
    //     let s = Shape::sphere();
    //     let n = s.intersect(Tuple::point(0.0, 1.0, 0.0));
    //     assert_eq!(n, Tuple::vector(0.0, 1.0, 0.0));
    // }

    // #[test]
    // fn the_normal_on_a_sphere_at_a_point_on_the_z_axis() {
    //     let s = Sphere::new();
    //     let n = s.local_intersect(Tuple::point(0.0, 0.0, 1.0));
    //     assert_eq!(n, Tuple::vector(0.0, 0.0, 1.0));
    // }

    // #[test]
    // fn the_normal_on_a_sphere_at_a_nonaxial_point() {
    //     let s = Sphere::new();
    //     let n = s.local_intersect(Tuple::point(3.0_f64.sqrt() / 3.0, 3.0_f64.sqrt() / 3.0, 3.0_f64.sqrt() / 3.0));
    //     assert_eq!(n, Tuple::vector(3.0_f64.sqrt() / 3.0, 3.0_f64.sqrt() / 3.0, 3.0_f64.sqrt() / 3.0));
    // }

    // #[test]
    // fn the_normal_is_a_normalized_vector() {
    //     let s = Sphere::new();
    //     let n = s.local_intersect(Tuple::point(3.0_f64.sqrt() / 3.0, 3.0_f64.sqrt() / 3.0, 3.0_f64.sqrt() / 3.0));
    //     assert_eq!(n, n.normalize());
    // }

    #[test]
    fn computing_the_normal_on_a_translated_sphere() {
        let mut s = Shape::sphere();
        s.set_transform(Matrix::translation(0.0, 1.0, 0.0));
        let n = s.normal_at(Tuple::point(0.0, 1.70711, -0.70711));
        let delta = 1e-5;
        assert!((n.0 - 0.0).abs() < delta);
        assert!((n.1 - 0.70711).abs() < delta);
        assert!((n.2 + 0.70711).abs() < delta);
    }

    #[test]
    fn computing_the_normal_on_a_transformed_sphere() {
        let mut s = Shape::sphere();
        let m = Matrix::scaling(1.0, 0.5, 1.0) * Matrix::rotation_z(std::f64::consts::PI / 5.0);
        s.set_transform(m);
        let n = s.normal_at(Tuple::point(0.0, 2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0));
        let delta = 1e-5;
        assert!((n.0 - 0.0).abs() < delta);
        assert!((n.1 - 0.97014).abs() < delta);
        assert!((n.2 + 0.24254).abs() < delta);
    }

    #[test]
    fn a_sphere_has_a_default_material() {
        let s = Sphere::new();
        assert_eq!(s.material, Material::new());
    }

    #[test]
    fn a_sphere_may_be_assigned_a_material() {
        let mut s = Sphere::new();
        let m = Material::new().with_ambient(1.0);
        s.set_material(m.clone());
        assert_eq!(s.material, m);
    }
}