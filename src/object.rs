use crate::{intersection::Intersection, material::Material, matrix::Matrix, ray::Ray, shape::Shape, shapes::{cube::Cube, plane::Plane, sphere::Sphere}, tuple::{Point, Vector}};

#[derive(Clone, PartialEq, Debug)]
pub struct Object {
    pub shape: Shape,
    pub transform: Matrix,
    pub material: Material,
}

impl Object {
    pub fn new(shape: Shape, transform: Matrix, material: Material) -> Object {
        Object { shape, transform, material }
    }

    pub fn test_shape() -> Object {
        Object {
            shape: Shape::TestShape(crate::shapes::test_shape::TestShape::new()),
            transform: Matrix::identity(),
            material: Material::new(),
        }
    }

    pub fn sphere() -> Object {
        Object {
            shape: Shape::Sphere(Sphere::new()),
            transform: Matrix::identity(),
            material: Material::new(),
        }
    }

    pub fn plane() -> Object {
        Object {
            shape: Shape::Plane(Plane::new()),
            transform: Matrix::identity(),
            material: Material::new(),
        }
    }

    pub fn cube() -> Object {
        Object {
            shape: Shape::Cube(Cube::new()),
            transform: Matrix::identity(),
            material: Material::new(),
        }
    }

    pub fn cylinder () -> Object {
        Object {
            shape: Shape::Cylinder(crate::shapes::cylinder::Cylinder::new()),
            transform: Matrix::identity(),
            material: Material::new(),
        }
    }

    pub fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let itx = self.shape.local_intersect(&ray.transform(&self.transform.inverse()));
        itx.into_iter().map(|t| Intersection::new(t, self)).collect()
    }

    pub fn normal_at(&self, world_point: &Point) -> Vector {
        let object_point = self.transform.inverse() * *world_point;
        let object_normal = self.shape.local_normal_at(&object_point);
        let mut world_normal = self.transform.inverse().transpose() * object_normal;
        world_normal.3 = 0.0;
        world_normal.normalize()
    }

    pub fn get_transform(&self) -> &Matrix {
        &self.transform
    }

    pub fn set_transform(&mut self, transform: Matrix) {
        self.transform = transform;
    }

    pub fn with_transform(&self, transform: Matrix) -> Object {
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

    pub fn with_material(&self, material: Material) -> Object {
        let mut new_object = self.clone();
        new_object.set_material(material);
        new_object
    }

    
}

impl Default for Object {
    fn default() -> Self {
        Object {
            shape: Shape::Sphere(Sphere::new()),
            transform: Matrix::identity(),
            material: Material::new(),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    use crate::ray::Ray;
    use crate::tuple::Tuple;
    use crate::material::Material;
    use crate::matrix::Matrix;
    use crate::transformation::Transformation;

    #[test]
    fn default_transform() {
        let s = Object::test_shape();
        assert_eq!(*s.get_transform(), Matrix::identity());
    }

    #[test]
    fn assigning_a_transform() {
        let mut s = Object::test_shape();
        let t = Matrix::translation(2.0, 3.0, 4.0);
        s.set_transform(t.clone());
        assert_eq!(*s.get_transform(), t);
    }

    #[test]
    fn default_material() {
        let s = Object::test_shape();
        assert_eq!(*s.get_material(), Material::new());
    }

    #[test]
    fn assigning_a_material() {
        let mut s = Object::test_shape();
        let m = Material::new().with_ambient(1.0);
        s.set_material(m.clone());
        assert_eq!(*s.get_material(), m);
    }

    #[test]
    fn intersecting_a_scaled_shape_with_a_ray() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Object::test_shape().with_transform(Matrix::scaling(2.0, 2.0, 2.0));
        let xs = s.intersect(&r);
        assert_eq!(s.get_transform(), &Matrix::scaling(2.0, 2.0, 2.0));
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn intersecting_a_translated_shape_with_a_ray() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s = Object::test_shape().with_transform(Matrix::translation(5.0, 0.0, 0.0));
        let xs = s.intersect(&r);
        assert_eq!(s.get_transform(), &Matrix::translation(5.0, 0.0, 0.0));
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn computing_the_normal_on_a_translated_shape() {
        let s = Object::test_shape().with_transform(Matrix::translation(0.0, 1.0, 0.0));
        let n = s.normal_at(&Tuple::point(0.0, 1.70711, -0.70711));
        let delta = 1e-5;
        assert!((n.0 - 0.0).abs() < delta);
        assert!((n.1 - 0.70711).abs() < delta);
        assert!((n.2 + 0.70711).abs() < delta);
    }

    #[test]
    fn computing_the_normal_on_a_transformed_shape() {
        let s = Object::test_shape().with_transform(Matrix::scaling(1.0, 0.5, 1.0) * Matrix::rotation_z(std::f64::consts::PI / 5.0));
        let n = s.normal_at(&Tuple::point(0.0, 2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0));
        let delta = 1e-5;
        assert!((n.0 - 0.0).abs() < delta);
        assert!((n.1 - 0.97014).abs() < delta);
        assert!((n.2 + 0.24254).abs() < delta);
    }
}