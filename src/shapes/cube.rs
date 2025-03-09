use crate::{intersection::Intersection, material::Material, matrix::Matrix, ray::Ray, shape::Shape, tuple::Tuple};

#[derive(Clone, PartialEq, Debug)]
pub struct Cube {
    pub side: f64,
    transform: Matrix,
    material: Material,
}

impl Cube {
    pub fn new() -> Cube {
        Cube { side: 1.0, transform: Matrix::identity(), material: Material::new() }
    }

    pub fn local_intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let (xtmin, xtmax) = self.check_axis(ray.origin.0, ray.direction.0);
        let (ytmin, ytmax) = self.check_axis(ray.origin.1, ray.direction.1);
        let (ztmin, ztmax) = self.check_axis(ray.origin.2, ray.direction.2);

        let tmin = xtmin.max(ytmin).max(ztmin);
        let tmax = xtmax.min(ytmax).min(ztmax);

        if tmin > tmax {
            return vec![];
        }

        vec![
            Intersection::new(tmin, Shape::Cube(self.clone())),
            Intersection::new(tmax, Shape::Cube(self.clone())),
        ]
    }

    pub fn local_normal_at(&self, point: &Tuple) -> Tuple {
        let maxc = point.0.abs().max(point.1.abs()).max(point.2.abs());

        if maxc == point.0.abs() {
            Tuple::vector(point.0, 0.0, 0.0)
        } else if maxc == point.1.abs() {
            Tuple::vector(0.0, point.1, 0.0)
        } else {
            Tuple::vector(0.0, 0.0, point.2)
        }
    }

    pub fn get_transform(&self) -> &Matrix {
        &self.transform
    }

    pub fn set_transform(&mut self, transform: Matrix) {
        self.transform = transform;
    }

    pub fn with_transform(&self, transform: Matrix) -> Cube {
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

    pub fn with_material(&self, material: Material) -> Cube {
        let mut new_sphere = self.clone();
        new_sphere.set_material(material);
        new_sphere
    }

    fn check_axis(&self, origin: f64, direction: f64) -> (f64, f64) {
        let tmin_numerator = -1.0 - origin;
        let tmax_numerator = 1.0 - origin;

        let (tmin, tmax) = if direction.abs() >= 1e-6 {
            (tmin_numerator / direction, tmax_numerator / direction)
        } else {
            (tmin_numerator * f64::INFINITY, tmax_numerator * f64::INFINITY)
        };

        if tmin > tmax {
            (tmax, tmin)
        } else {
            (tmin, tmax)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{ray::Ray, tuple::Tuple};

    use super::*;

    #[test]
    fn a_ray_intersects_a_cube() {
        let c = Cube::new();
        let r = Ray::new(Tuple::point(5.0, 0.5, 0.0), Tuple::vector(-1.0, 0.0, 0.0));
        let xs = c.local_intersect(&r);
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[1].t, 6.0);
    }

    #[test]
    fn a_ray_misses_a_cube() {
        let c = Cube::new();
        let r = Ray::new(Tuple::point(-2.0, 0.0, 0.0), Tuple::vector(0.2673, 0.5345, 0.8018));
        let xs = c.local_intersect(&r);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn the_normal_on_the_surface_of_a_cube() {
        let c = Cube::new();
        let n = c.local_normal_at(&Tuple::point(1.0, 0.5, -0.8));
        assert_eq!(n, Tuple::vector(1.0, 0.0, 0.0));
    }
}