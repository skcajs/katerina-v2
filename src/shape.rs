use crate::intersection::Intersection;
use crate::material::Material;
use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::shapes::cube::Cube;
use crate::shapes::plane::Plane;
use crate::shapes::sphere::Sphere;
use crate::shapes::test_shape::TestShape;
use crate::tuple::{Point, Vector};

#[derive(Debug, PartialEq, Clone)]
pub enum Shape {
    Sphere(Sphere),
    TestShape(TestShape),
    Plane(Plane),
    Cube(Cube),
}

impl Shape {
    pub fn sphere() -> Shape {
        Shape::Sphere(Sphere::new())
    }

    pub fn plane() -> Shape {
        Shape::Plane(Plane::new())
    }

    pub fn test_shape() -> Shape {
        Shape::TestShape(TestShape::new())
    }

    pub fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let local_ray = ray.transform(&self.get_transform().inverse());
        match self {
            Shape::Sphere(s) => s.local_intersect(&local_ray),
            Shape::TestShape(s) => s.local_intersect(&local_ray),
            Shape::Plane(s) => s.local_intersect(&local_ray),
            Shape::Cube(s) => s.local_intersect(&local_ray),
        }
    }

    pub fn normal_at(&self, point: Point) -> Vector {
        let local_point = self.get_transform().inverse() * point;
        let local_normal = match self {
            Shape::Sphere(s) => s.local_normal_at(&local_point),
            Shape::TestShape(s) => s.local_normal_at(&local_point),
            Shape::Plane(s) => s.local_normal_at(),
            Shape::Cube(s) => s.local_normal_at(&local_point),
        };
        let mut world_normal = self.get_transform().inverse().transpose() * local_normal;
        world_normal.3 = 0.0;
        world_normal.normalize()
    }

    pub fn get_transform(&self) -> &Matrix {
        match self {
            Shape::Sphere(s) => s.get_transform(),
            Shape::TestShape(s) => s.get_transform(),
            Shape::Plane(s) => s.get_transform(),
            Shape::Cube(s) => s.get_transform(),
        }
    }

    pub fn set_transform(&mut self, transform: Matrix) {
        match self {
            Shape::Sphere(s) => s.set_transform(transform),
            Shape::TestShape(s) => s.set_transform(transform),
            Shape::Plane(s) => s.set_transform(transform),
            Shape::Cube(s) => s.set_transform(transform),
        }
    }

    pub fn with_transform(&self, transform: Matrix) -> Shape {
        match self {
            Shape::Sphere(s) => Shape::Sphere(s.with_transform(transform)),
            Shape::TestShape(s) => Shape::TestShape(s.with_transform(transform)),
            Shape::Plane(s) => Shape::Plane(s.with_transform(transform)),
            Shape::Cube(s) => Shape::Cube(s.with_transform(transform)),
        }
    }

    pub fn get_material(&self) -> &Material {
        match self {
            Shape::Sphere(s) => s.get_material(),
            Shape::TestShape(s) => s.get_material(),
            Shape::Plane(s) => s.get_material(),
            Shape::Cube(s) => s.get_material(),
        }
    }

    pub fn set_material(&mut self, material: Material) {
        match self {
            Shape::Sphere(s) => s.set_material(material),
            Shape::TestShape(s) => s.set_material(material),
            Shape::Plane(s) => s.set_material(material),
            Shape::Cube(s) => s.set_material(material),
        }
    }

    pub fn with_material(&self, material: Material) -> Shape {
        match self {
            Shape::Sphere(s) => Shape::Sphere(s.with_material(material)),
            Shape::TestShape(s) => Shape::TestShape(s.with_material(material)),
            Shape::Plane(s) => Shape::Plane(s.with_material(material)),
            Shape::Cube(s) => Shape::Cube(s.with_material(material)),
        }
    }
}

#[cfg(test)]
mod tests {

    use crate::ray::Ray;
    use crate::shape::Shape;
    use crate::tuple::Tuple;
    use crate::material::Material;
    use crate::matrix::Matrix;
    use crate::transformation::Transformation;

    #[test]
    fn default_transform() {
        let s = Shape::test_shape();
        assert_eq!(*s.get_transform(), Matrix::identity());
    }

    #[test]
    fn assigning_a_transform() {
        let mut s = Shape::test_shape();
        let t = Matrix::translation(2.0, 3.0, 4.0);
        s.set_transform(t.clone());
        assert_eq!(*s.get_transform(), t);
    }

    #[test]
    fn default_material() {
        let s = Shape::test_shape();
        assert_eq!(*s.get_material(), Material::new());
    }

    #[test]
    fn assigning_a_material() {
        let mut s = Shape::test_shape();
        let m = Material::new().with_ambient(1.0);
        s.set_material(m.clone());
        assert_eq!(*s.get_material(), m);
    }

    #[test]
    fn intersecting_a_scaled_shape_with_a_ray() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Shape::test_shape().with_transform(Matrix::scaling(2.0, 2.0, 2.0));
        let xs = s.intersect(&r);
        assert_eq!(s.get_transform(), &Matrix::scaling(2.0, 2.0, 2.0));
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn intersecting_a_translated_shape_with_a_ray() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Shape::test_shape().with_transform(Matrix::translation(5.0, 0.0, 0.0));
        let xs = s.intersect(&r);
        assert_eq!(s.get_transform(), &Matrix::translation(5.0, 0.0, 0.0));
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn computing_the_normal_on_a_translated_shape() {
        let s = Shape::test_shape().with_transform(Matrix::translation(0.0, 1.0, 0.0));
        let n = s.normal_at(Tuple::point(0.0, 1.70711, -0.70711));
        let delta = 1e-5;
        assert!((n.0 - 0.0).abs() < delta);
        assert!((n.1 - 0.70711).abs() < delta);
        assert!((n.2 + 0.70711).abs() < delta);
    }

    #[test]
    fn computing_the_normal_on_a_transformed_shape() {
        let s = Shape::test_shape().with_transform(Matrix::scaling(1.0, 0.5, 1.0) * Matrix::rotation_z(std::f64::consts::PI / 5.0));
        let n = s.normal_at(Tuple::point(0.0, 2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0));
        let delta = 1e-5;
        assert!((n.0 - 0.0).abs() < delta);
        assert!((n.1 - 0.97014).abs() < delta);
        assert!((n.2 + 0.24254).abs() < delta);
    }
}