use crate::{intersection::Intersection, object::Object, ray::Ray, tuple::Tuple};

#[derive(Debug, Clone, PartialEq)]
pub struct Plane;

impl Plane {
    pub fn new() -> Plane {
        Plane {}
    }

    pub fn local_intersect<'a>(&self, object: &'a Object, ray: &Ray) -> Vec<Intersection<'a>> {
        if ray.direction.1.abs() < 1e-5 {
            return vec![];
        }

        let t = -ray.origin.1 / ray.direction.1;
        vec![Intersection::new(t, object)]
    }

    pub fn local_normal_at(&self) -> Tuple {
        Tuple::vector(0.0, 1.0, 0.0)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    use crate::{object::Object, ray::Ray};

    #[test]
    fn the_normal_of_a_plane_is_constant_everywhere() {
        let p = Object::plane();
        let n1 = p.normal_at(&Tuple::point(0.0, 0.0, 0.0));
        let n2 = p.normal_at(&Tuple::point(10.0, 0.0, -10.0));
        let n3 = p.normal_at(&Tuple::point(-5.0, 0.0, 150.0));
        assert_eq!(n1, Tuple::vector(0.0, 1.0, 0.0));
        assert_eq!(n2, Tuple::vector(0.0, 1.0, 0.0));
        assert_eq!(n3, Tuple::vector(0.0, 1.0, 0.0));
    }

    #[test]
    fn intersect_with_a_ray_parallel_to_the_plane() {
        let p = Object::plane();
        let r = Ray::new(Tuple::point(0.0, 10.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
        let xs = p.intersect(&r);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn intersect_with_a_coplanar_ray() {
        let p = Object::plane();
        let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
        let xs = p.intersect(&r);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn a_ray_intersecting_a_plane_from_above() {
        let p = Object::plane();
        let r = Ray::new(Tuple::point(0.0, 1.0, 0.0), Tuple::vector(0.0, -1.0, 0.0));
        let xs = p.intersect(&r);
        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0].t, 1.0);
        assert_eq!(xs[0].object, &p);
    }

    #[test]
    fn a_ray_intersecting_a_plane_from_below() {
        let p = Object::plane();
        let r = Ray::new(Tuple::point(0.0, -1.0, 0.0), Tuple::vector(0.0, 1.0, 0.0));
        let xs = p.intersect(&r);
        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0].t, 1.0);
        assert_eq!(xs[0].object, &p);
    }
}