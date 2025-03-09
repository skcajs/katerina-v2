use crate::{intersection::Intersection, object::Object, ray::Ray, tuple::Tuple};


#[derive(Debug, Clone, PartialEq)]
pub struct Cone {
    pub minimum: f64,
    pub maximum: f64,
    pub closed: bool,
}

impl Cone {
    pub fn new() -> Cone {
        Cone {
            minimum: f64::NEG_INFINITY,
            maximum: f64::INFINITY,
            closed: false,
        }
    }

    pub fn local_intersect<'a>(&self, object: &'a Object, ray: &Ray) -> Vec<Intersection<'a>> {
        let a = ray.direction.0.powi(2) - ray.direction.1.powi(2) + ray.direction.2.powi(2);
        let b = 2.0 * ray.origin.0 * ray.direction.0 - 2.0 * ray.origin.1 * ray.direction.1 + 2.0 * ray.origin.2 * ray.direction.2;
        let c = ray.origin.0.powi(2) - ray.origin.1.powi(2) + ray.origin.2.powi(2);

        let mut xs = vec![];

        if a.abs() < 1e-6 && b.abs() > 1e-6 {
            xs.push(Intersection::new(-c / (2.0 * b), object));
        } else if a.abs() >= 1e-6 {
            let disc = b.powi(2) - 4.0 * a * c;
            if disc < 0.0 {
                return xs;
            }
    
            let mut t0 = (-b - disc.sqrt()) / (2.0 * a);
            let mut t1 = (-b + disc.sqrt()) / (2.0 * a);
    
            if t0 > t1 {
                (t0, t1) = (t1, t0);
            } 

            let y0 = ray.origin.1 + t0 * ray.direction.1;
            if self.minimum < y0 && y0 < self.maximum {
                xs.push(Intersection::new(t0, object));
            }

            let y1 = ray.origin.1 + t1 * ray.direction.1;
            if self.minimum < y1 && y1 < self.maximum {
                xs.push(Intersection::new(t1, object));
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
            let mut y = (local_point.0.powi(2) + local_point.2.powi(2)).sqrt();
            if local_point.1 > 0.0 {
                y = -y;
            }
            Tuple::vector(local_point.0, y, local_point.2)
        }
    }

    fn check_cap(&self, ray: &Ray, t: f64, r: f64) -> bool {
        let x = ray.origin.0 + t * ray.direction.0;
        let z = ray.origin.2 + t * ray.direction.2;
        x.powi(2) + z.powi(2) <= r.abs()
    }

    pub fn intersect_caps<'a>(&self, object: &'a Object, ray: &Ray, xs: &mut Vec<Intersection<'a>>) {
        if !self.closed || ray.direction.1.abs() < 1e-6 {
            return;
        }

        let t = (self.minimum - ray.origin.1) / ray.direction.1;
        if self.check_cap(ray, t, self.minimum) {
            xs.push(Intersection::new(t, object));
        }

        let t = (self.maximum - ray.origin.1) / ray.direction.1;
        if self.check_cap(ray, t, self.maximum) {
            xs.push(Intersection::new(t, object));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ray::Ray;

    #[test]
    fn intersecting_cone_with_ray() {
        let object =  Object::test_shape();
        let cone = Cone::new();
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let xs = cone.local_intersect(&object, &r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 5.0);
        assert_eq!(xs[1].t, 5.0);

        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(1.0, 1.0, 1.0).normalize());
        let xs = cone.local_intersect(&object, &r);
        assert_eq!(xs.len(), 2);
        let delta = 1e-4;
        assert!((xs[0].t - 8.66025).abs() < delta);
        assert!((xs[1].t - 8.66025).abs() < delta);

        let r = Ray::new(Tuple::point(1.0, 1.0, -5.0), Tuple::vector(-0.5, -1.0, 1.0).normalize());
        let xs = cone.local_intersect(&object, &r);
        assert_eq!(xs.len(), 2);
        let delta = 1e-4;
        assert!((xs[0].t - 4.55006).abs() < delta);
        assert!((xs[1].t - 49.44994).abs() < delta);
    }

    #[test]
    fn intersecting_a_cone_with_a_ray_parallel_to_one_of_its_halves() {
        let object =  Object::test_shape();

        let cone = Cone::new();
        let r = Ray::new(Tuple::point(0.0, 0.0, -1.0), Tuple::vector(0.0, 1.0, 1.0).normalize());
        let xs = cone.local_intersect(&object, &r);
        assert_eq!(xs.len(), 1);
        let delta = 1e-4;
        assert!((xs[0].t - 0.35355).abs() < delta);
    }

    #[test]
    fn intersecting_a_cone_end_caps() {
        let object =  Object::test_shape();
        let cone = Cone {
            minimum: -0.5,
            maximum: 0.5,
            closed: true,
        };
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 1.0, 0.0).normalize());
        let xs = cone.local_intersect(&object, &r);
        assert_eq!(xs.len(), 0);

        let r = Ray::new(Tuple::point(0.0, 0.0, -0.25), Tuple::vector(0.0, 1.0, 1.0).normalize());
        let xs = cone.local_intersect(&object, &r);
        assert_eq!(xs.len(), 2);

        let r = Ray::new(Tuple::point(0.0, 0.0, -0.25), Tuple::vector(0.0, 1.0, 0.0).normalize());
        let xs = cone.local_intersect(&object, &r);
        assert_eq!(xs.len(), 4);
    }

    #[test]
    fn computing_the_normal_vector_on_a_cone() {
        let cone = Cone::new();
        let n = cone.local_normal_at(&Tuple::point(0.0, 0.0, 0.0));
        assert_eq!(n, Tuple::vector(0.0, 0.0, 0.0));

        let n = cone.local_normal_at(&Tuple::point(1.0, 1.0, 1.0));
        assert_eq!(n, Tuple::vector(1.0, -2f64.sqrt(), 1.0));

        let n = cone.local_normal_at(&Tuple::point(-1.0, -1.0, 0.0));
        assert_eq!(n, Tuple::vector(-1.0, 1.0, 0.0));
    }
}
