use crate::{object::Object, ray::Ray, tuple::Tuple};

pub struct Record<'a> {
    pub t: f64,
    pub object: &'a Object,
    pub point: Tuple,
    pub eyev: Tuple,
    pub normalv: Tuple,
    pub reflectv: Tuple,
    pub inside: bool,
    pub over_point: Tuple,
    pub under_point: Tuple,
    pub n1: f64,
    pub n2: f64,
    pub schlick: f64,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Intersection<'a> {
    pub t: f64,
    pub object: &'a Object,
}

impl<'a> Intersection<'a> {
    pub fn new(t: f64, object: &'a Object) -> Intersection<'a> {
        Intersection { t, object }
    }

    pub fn prepare_computations(&self, ray: &Ray, xs: &Vec<Intersection>) -> Record {
        let mut n1 = 1.0;
        let mut n2 = 1.0;
        let mut containers: Vec<Object> = vec![];

        for i in xs {
            if i == self {
                if containers.is_empty() {
                    n1 = 1.0;
                } else {
                    n1 = containers.last().unwrap().get_material().refractive_index;
                }
            }

            if containers.contains(&i.object) {
                containers.retain(|x| x != i.object);
            } else {
                containers.push(i.object.clone());
            }

            if i == self {
                if containers.is_empty() {
                    n2 = 1.0;
                } else {
                    n2 = containers.last().unwrap().get_material().refractive_index;
                }
                break;
            }
        }

        let mut normalv = self.object.normal_at(&ray.position(self.t));
        let eyev = -ray.direction;
        let inside = if normalv.dot(eyev) < 0.0 {
            normalv = -normalv;
            true
        } else {
            false
        };
        let point = ray.position(self.t);

        Record {
            t: self.t,
            object: self.object,
            point,
            eyev,
            normalv,
            reflectv: ray.direction.reflect(normalv),
            inside,
            over_point: point + normalv * 0.0001,
            under_point: point - normalv * 0.0001,
            n1,
            n2,
            schlick: {
                let cos = eyev.dot(normalv);
                if n1 > n2 {
                    let n = n1 / n2;
                    let sin2_t = n.powi(2) * (1.0 - cos.powi(2));
                    if sin2_t > 1.0 {
                        1.0
                    } else {
                        let cos = cos.abs();
                        let r0 = ((n1 - n2) / (n1 + n2)).powi(2);
                        r0 + (1.0 - r0) * (1.0 - cos).powi(5)
                    }
                } else {
                    let cos = cos.abs();
                    let r0 = ((n1 - n2) / (n1 + n2)).powi(2);
                    r0 + (1.0 - r0) * (1.0 - cos).powi(5)
                }
            }
        }
    }

}