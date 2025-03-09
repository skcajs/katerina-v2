use crate::intersection::Intersection;
use crate::object::Object;
use crate::tuple::Tuple;
use crate::ray::Ray;


#[derive(Clone, PartialEq, Debug)]
pub struct Sphere;

impl Sphere {
    pub fn new() -> Sphere {
        Sphere {}
    }

    pub fn local_intersect<'a>(&self, object: &'a Object, ray: &Ray) -> Vec<Intersection<'a>> {
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
            Intersection::new(t1, object),
            Intersection::new(t2, object)
        ]
    }

    pub fn local_normal_at(&self, point: &Tuple) -> Tuple {
        *point - Tuple::point(0.0, 0.0, 0.0)
    }
}

#[cfg(test)]
mod tests {
    use std::sync::LazyLock;

    use super::*;

    use crate::{helper::glass_sphere, matrix::Matrix, object::Object, transformation::Transformation};
    static OBJECT: LazyLock<Object> = LazyLock::new(|| Object::test_shape());

    #[test]
    fn a_ray_intersects_a_sphere_at_two_points() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.local_intersect(&OBJECT, &r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 6.0);
    }

    #[test]
    fn a_ray_misses_a_sphere() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 1.0, 0.0));
        let s = Sphere::new();
        let xs = s.local_intersect(&OBJECT, &r);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn a_ray_originates_inside_a_sphere() {
        let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.local_intersect(&OBJECT, &r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -1.0);
        assert_eq!(xs[1].t, 1.0);
    }

    #[test]
    fn a_sphere_is_behind_a_ray() {
        let r = Ray::new(Tuple::point(0.0, 0.0, 5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Sphere::new();
        let xs = s.local_intersect(&OBJECT, &r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -6.0);
        assert_eq!(xs[1].t, -4.0);
    }

    #[test]
    fn intersecting_a_scaled_sphere_with_a_ray() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let mut s = Object::sphere();
        s.set_transform(Matrix::scaling(2.0, 2.0, 2.0));
        let xs = s.intersect(&r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 3.0);
        assert_eq!(xs[1].t, 7.0);
    }

    #[test]
    fn intersecting_a_translated_sphere_with_a_ray() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let mut s = Object::sphere();
        s.set_transform(Matrix::translation(5.0, 0.0, 0.0));
        let xs = s.intersect(&r);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn computing_the_normal_on_a_translated_sphere() {
        let mut s = Object::sphere();
        s.set_transform(Matrix::translation(0.0, 1.0, 0.0));
        let n = s.normal_at(&Tuple::point(0.0, 1.70711, -0.70711));
        let delta = 1e-5;
        assert!((n.0 - 0.0).abs() < delta);
        assert!((n.1 - 0.70711).abs() < delta);
        assert!((n.2 + 0.70711).abs() < delta);
    }

    #[test]
    fn computing_the_normal_on_a_transformed_sphere() {
        let mut s = Object::sphere();
        let m = Matrix::scaling(1.0, 0.5, 1.0) * Matrix::rotation_z(std::f64::consts::PI / 5.0);
        s.set_transform(m);
        let n = s.normal_at(&Tuple::point(0.0, 2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0));
        let delta = 1e-5;
        assert!((n.0 - 0.0).abs() < delta);
        assert!((n.1 - 0.97014).abs() < delta);
        assert!((n.2 + 0.24254).abs() < delta);
    }

    #[test]
    fn a_helper_for_producing_a_sphere_with_a_glassy_material() {
        let s = glass_sphere();
        assert_eq!(s.get_material().transparency, 1.0);
        assert_eq!(s.get_material().refractive_index, 1.5);
    }
}
