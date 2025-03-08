use crate::{ray::Ray, shape::Shape, tuple::Tuple};

pub struct Record {
    pub t: f64,
    pub object: Shape,
    pub point: Tuple,
    pub eyev: Tuple,
    pub normalv: Tuple,
    pub inside: bool,
    pub over_point: Tuple,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Intersection {
    pub t: f64,
    pub object: Shape,
}

impl Intersection {
    pub fn new(t: f64, object: Shape) -> Intersection {
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
            object: self.object.clone(),
            point,
            eyev,
            normalv,
            inside,
            over_point
        }
    }
}