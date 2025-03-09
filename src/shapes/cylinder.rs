use crate::{intersection::Intersection, object::Object, ray::Ray, tuple::Tuple};


#[derive(Debug, Clone, PartialEq)]
pub struct Cylinder {
    pub minimum: f64,
    pub maximum: f64,
    pub closed: bool,
}


impl Cylinder {

    pub fn new() -> Cylinder {
        Cylinder {
            minimum: f64::NEG_INFINITY,
            maximum: f64::INFINITY,
            closed: false,
        }
    }

    pub fn local_intersect<'a>(&self, object: &'a Object, ray: &Ray) -> Vec<Intersection<'a>> {
        let mut xs = vec![];
        let a = ray.direction.0.powi(2) + ray.direction.2.powi(2);
        if a.abs() > 1e-6 {
            let b = 2.0 * ray.origin.0 * ray.direction.0 + 2.0 * ray.origin.2 * ray.direction.2;
            let c = ray.origin.0.powf(2.) + ray.origin.2.powf(2.) - 1.0;

            let discriminant = b.powi(2) - 4.0 * a * c;

            if discriminant < 0.0 {
                return vec![];
            } else {
                let t0 = (-b - discriminant.sqrt()) / (2.0 * a);
                let t1 = (-b + discriminant.sqrt()) / (2.0 * a);

                let y0 = ray.origin.1 + t0 * ray.direction.1;
                if self.minimum < y0 && y0 < self.maximum {
                    xs.push(Intersection::new(t0, object));
                }

                let y1 = ray.origin.1 + t1 * ray.direction.1;
                if self.minimum < y1 && y1 < self.maximum {
                    xs.push(Intersection::new(t1, object));
                }
            }
        }
        self.intersect_caps(object, ray, &mut xs);
        xs
    }

    pub fn local_normal_at(&self, local_point: &Tuple) -> Tuple {
        let dist = local_point.0.powi(2) + local_point.2.powi(2);
        if dist < 1.0 && local_point.1 >= self.maximum - 1e-6 {
            Tuple::vector(0.0, 1.0, 0.0)
        } else if dist < 1.0 && local_point.1 <= self.minimum + 1e-6 {
            Tuple::vector(0.0, -1.0, 0.0)
        } else {
            Tuple::vector(local_point.0, 0.0, local_point.2)
        }
    }

    fn check_cap(&self, ray: &Ray, t: f64) -> bool {
        let x = ray.origin.0 + t * ray.direction.0;
        let z = ray.origin.2 + t * ray.direction.2;
        x.powi(2) + z.powi(2) <= 1.0
    }

    pub fn intersect_caps<'a>(&self, object: &'a Object, ray: &Ray, xs: &mut Vec<Intersection<'a>>) {
        if !self.closed || ray.direction.1.abs() < 1e-6 {
            return;
        }

        let t = (self.minimum - ray.origin.1) / ray.direction.1;
        if self.check_cap(ray, t) {
            xs.push(Intersection::new(t, object));
        }

        let t = (self.maximum - ray.origin.1) / ray.direction.1;
        if self.check_cap(ray, t) {
            xs.push(Intersection::new(t, object));
        }
    }
}

#[cfg(test)]
mod tests {
    use std::sync::LazyLock;

    use super::*;

    use crate::ray::Ray;
    static OBJECT: LazyLock<Object> = LazyLock::new(|| Object::test_shape());


    #[test]
    fn a_ray_misses_a_cylinder() {
        let c = Cylinder::new();
        let r = Ray::new(Tuple::point(1.0, 0.0, 0.0), Tuple::vector(0.0, 1.0, 0.0));
        let xs = c.local_intersect(&OBJECT, &r);
        assert_eq!(xs.len(), 0);

        let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 1.0, 0.0));
        let xs = c.local_intersect(&OBJECT, &r);
        assert_eq!(xs.len(), 0);

        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(1.0, 1.0, 1.0));
        let xs = c.local_intersect(&OBJECT, &r);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn a_ray_strikes_a_cylinder() {
        let c = Cylinder::new();
        let r = Ray::new(Tuple::point(1.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let xs = c.local_intersect(&OBJECT, &r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 5.0);
        assert_eq!(xs[1].t, 5.0);

        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let xs = c.local_intersect(&OBJECT, &r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 6.0);

        let r = Ray::new(Tuple::point(0.5, 0.0, -5.0), Tuple::vector(0.1, 1.0, 1.0).normalize());
        let xs = c.local_intersect(&OBJECT, &r);
        assert_eq!(xs.len(), 2);
        assert!((xs[0].t - 6.80798).abs() < 1e-5);
        assert!((xs[1].t - 7.08872).abs() < 1e-5);
    }

    #[test]
    fn normal_vector_on_a_cylinder() {
        let c = Cylinder::new();
        let n = c.local_normal_at(&Tuple::point(1.0, 0.0, 0.0));
        assert_eq!(n, Tuple::vector(1.0, 0.0, 0.0));

        let n = c.local_normal_at(&Tuple::point(0.0, 5.0, -1.0));
        assert_eq!(n, Tuple::vector(0.0, 0.0, -1.));

        let n = c.local_normal_at(&Tuple::point(0.0, -2.0, 1.0));
        assert_eq!(n, Tuple::vector(0.0, 0.0, 1.0));

        let n = c.local_normal_at(&Tuple::point(-1.0, 1.0, 0.0));
        assert_eq!(n, Tuple::vector(-1.0, 0.0, 0.0));
    }

    #[test]
    fn the_default_minimum_and_maximum_for_a_cylinder() {
        let c = Cylinder::new();
        assert_eq!(c.minimum, f64::NEG_INFINITY);
        assert_eq!(c.maximum, f64::INFINITY);
    }

    #[test]
    fn intersecting_a_constrained_cylinder() {
        let c = Cylinder {
            minimum: 1.0,
            maximum: 2.0,
            closed: true,
        };

        let r = Ray::new(Tuple::point(0.0, 1.5, 0.0), Tuple::vector(0.1, 1.0, 0.0).normalize());
        let xs = c.local_intersect(&OBJECT, &r);
        assert_eq!(xs.len(), 0);

        let r = Ray::new(Tuple::point(0.0, 3.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let xs = c.local_intersect(&OBJECT, &r);
        assert_eq!(xs.len(), 0);

        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let xs = c.local_intersect(&OBJECT, &r);
        assert_eq!(xs.len(), 0);

        let r = Ray::new(Tuple::point(0.0, 2.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let xs = c.local_intersect(&OBJECT, &r);
        assert_eq!(xs.len(), 0);

        let r = Ray::new(Tuple::point(0.0, 1.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let xs = c.local_intersect(&OBJECT, &r);
        assert_eq!(xs.len(), 0);

        let r = Ray::new(Tuple::point(0.0, 1.5, -2.0), Tuple::vector(0.0, 0.0, 1.0));
        let xs = c.local_intersect(&OBJECT, &r);
        assert_eq!(xs.len(), 2);
    }

    #[test]
    fn the_default_closed_value_for_a_cylinder() {
        let c = Cylinder::new();
        assert!(!c.closed);
    }

    #[test]
    fn intersecting_the_caps_of_a_closed_cylinder() {
        let c = Cylinder {
            minimum: 1.0,
            maximum: 2.0,
            closed: true,
        };

        let r = Ray::new(Tuple::point(0.0, 3.0, 0.0), Tuple::vector(0.0, -1.0, 0.0));
        let xs = c.local_intersect(&OBJECT, &r);
        assert_eq!(xs.len(), 2);

        let r = Ray::new(Tuple::point(0.0, 3.0, -2.0), Tuple::vector(0.0, -1.0, 2.0));
        let xs = c.local_intersect(&OBJECT, &r);
        assert_eq!(xs.len(), 2);

        let r = Ray::new(Tuple::point(0.0, 4.0, -2.0), Tuple::vector(0.0, -1.0, 1.0));
        let xs = c.local_intersect(&OBJECT, &r);
        assert_eq!(xs.len(), 2);

        let r = Ray::new(Tuple::point(0.0, 0.0, -2.0), Tuple::vector(0.0, 1.0, 2.0));
        let xs = c.local_intersect(&OBJECT, &r);
        assert_eq!(xs.len(), 2);

        let r = Ray::new(Tuple::point(0.0, -1.0, -2.0), Tuple::vector(0.0, 1.0, 1.0));
        let xs = c.local_intersect(&OBJECT, &r);
        assert_eq!(xs.len(), 2);
    }

    #[test]
    fn the_normal_vector_on_a_cylinder_end_caps() {
        let c = Cylinder {
            minimum: 1.0,
            maximum: 2.0,
            closed: true,
        };

        let n = c.local_normal_at(&Tuple::point(0.0, 1.0, 0.0));
        assert_eq!(n, Tuple::vector(0.0, -1.0, 0.0));

        let n = c.local_normal_at(&Tuple::point(0.5, 1.0, 0.0));
        assert_eq!(n, Tuple::vector(0.0, -1.0, 0.0));

        let n = c.local_normal_at(&Tuple::point(0.0, 1.0, 0.5));
        assert_eq!(n, Tuple::vector(0.0, -1.0, 0.0));

        let n = c.local_normal_at(&Tuple::point(0.0, 2.0, 0.0));
        assert_eq!(n, Tuple::vector(0.0, 1.0, 0.0));

        let n = c.local_normal_at(&Tuple::point(0.5, 2.0, 0.0));
        assert_eq!(n, Tuple::vector(0.0, 1.0, 0.0));

        let n = c.local_normal_at(&Tuple::point(0.0, 2.0, 0.5));
        assert_eq!(n, Tuple::vector(0.0, 1.0, 0.0));
    }

}
