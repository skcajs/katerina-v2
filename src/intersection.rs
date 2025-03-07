use crate::{ray::Ray, sphere::Sphere, tuple::Tuple};

pub struct Record<'a> {
    pub t: f64,
    pub object: &'a Sphere,
    pub point: Tuple,
    pub eyev: Tuple,
    pub normalv: Tuple,
    pub inside: bool,
    pub over_point: Tuple,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Intersection<'a> {
    pub t: f64,
    pub object: &'a Sphere,
}

impl<'a> Intersection<'a> {
    pub fn new(t: f64, object: &'a Sphere) -> Intersection<'a> {
        Intersection { t, object }
    }

    pub fn prepare_computations(&self, ray: &Ray) -> Record {
        let mut normalv = self.object.normal_at(ray.position(self.t));
        let eyev = -ray.direction;
        let inside = if normalv.dot(eyev) < 0.0 {
            normalv = -normalv;
            true
        } else {
            false
        };
        let point = ray.position(self.t);
        let over_point = point + normalv * 0.0001;
        Record {
            t: self.t,
            object: self.object,
            point,
            eyev,
            normalv,
            inside,
            over_point
        }
    }
}