use std::sync::Arc;

use crate::{intersection::Intersection, material::Material, matrix::Matrix, ray::Ray, shape::Shape, shapes::{cone::Cone, cube::Cube, cylinder::Cylinder, group::Group, plane::Plane, sphere::Sphere, test_shape::TestShape}, tuple::{Point, Vector}};

#[derive(Clone, PartialEq, Debug)]
pub struct Object {
    pub shape: Shape,
    pub transform: Matrix,
    pub material: Material,
    pub parent: Option<Arc<Object>>
}

impl Object {
    pub fn new(shape: Shape) -> Object {
        Object { shape, transform: Matrix::identity(), material: Material::new(), parent: None }
    }

    pub fn test_shape() -> Object {
        Object::new(Shape::TestShape(TestShape::new()))
    }

    pub fn sphere() -> Object {
        Object::new(Shape::Sphere(Sphere::new()))
    }

    pub fn plane() -> Object {
        Object::new(Shape::Plane(Plane::new()))
    }

    pub fn cube() -> Object {
        Object::new(Shape::Cube(Cube::new()))
    }

    pub fn cylinder() -> Object {
        Object::new(Shape::Cylinder(Cylinder::new()))
    }

    pub fn cone() -> Object {
        Object::new(Shape::Cone(Cone::new()))
    }

    pub fn group() -> Object {
        Object::new(Shape::Group(Group::new()))
    }

    pub fn as_group(&mut self) -> Option<&mut Group> {
        if let Shape::Group(ref mut group) = self.shape {
            Some(group)
        } else {
            None
        }
    }

    pub fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        self.shape.local_intersect(self, &ray.transform(&self.transform.inverse()))
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

    pub fn world_to_object(&self, world_point: &Point) -> Point {
        if let Some(parent) = &self.parent {
            let point = parent.world_to_object(world_point);
            self.transform.inverse() * point
        } else {
            self.transform.inverse() * *world_point
        }
    }

    pub fn normal_to_world(&self, object_normal: &Vector) -> Vector {
        let mut world_normal = self.transform.inverse().transpose() * *object_normal;
        
        world_normal.3 = 0.0;
        
        world_normal = world_normal.normalize();
    
        if let Some(parent) = &self.parent {
            world_normal = parent.normal_to_world(&world_normal);
        }
    
        world_normal
    }

    pub fn add_child(&mut self, child: &mut Object) {
        let parent_clone = Arc::new(self.clone());
        if let Shape::Group(ref mut group) = self.shape {
            child.parent = Some(parent_clone);
            group.children.push(child.clone());
        }
    }

    pub fn get_children(&self) -> Option<&Vec<Object>> {
        if let Shape::Group(ref group) = self.shape {
            Some(&group.children)
        } else {
            None
        }
    }

    
}

impl Default for Object {
    fn default() -> Self {
        Object {
            shape: Shape::Sphere(Sphere::new()),
            transform: Matrix::identity(),
            material: Material::new(),
            parent: None,
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

    #[test]
    fn a_shape_has_a_parent_attribute() {
        let s = Object::test_shape();
        assert!(s.parent.is_none());
    }

    #[test]
    fn converting_a_point_from_world_to_object_space() {
        let mut g1 = Object::group();
        g1.set_transform(Matrix::rotation_y(std::f64::consts::PI / 2.0));
        let mut g2 = Object::group();
        g2.set_transform(Matrix::scaling(2.0, 2.0, 2.0));
        g1.add_child(&mut g2);
        let mut s = Object::sphere().with_transform(Matrix::translation(5.0, 0.0, 0.0));
        g2.add_child(&mut s);
        let p = s.world_to_object(&Tuple::point(-2.0, 0.0, -10.0));
        let delta = 1e-5;
        assert!((p.0 - 0.0).abs() < delta);
        assert!((p.1 - 0.0).abs() < delta);
        assert!((p.2 + 1.0).abs() < delta);
    }

    #[test]
    fn converting_a_normal_from_object_to_world_space() {
        let mut g1 = Object::group();
        g1.set_transform(Matrix::rotation_y(std::f64::consts::PI / 2.0));
        let mut g2 = Object::group();
        g2.set_transform(Matrix::scaling(1.0, 2.0, 3.0));
        g1.add_child(&mut g2);
        let mut s = Object::sphere().with_transform(Matrix::translation(5.0, 0.0, 0.0));
        g2.add_child(&mut s);
        let n = s.normal_to_world(&Tuple::vector(
            (3.0_f64).sqrt() / 3.0,
            (3.0_f64).sqrt() / 3.0,
            (3.0_f64).sqrt() / 3.0,
        ));
        let delta = 1e-4;
        assert!((n.0 - 0.28571).abs() < delta);
        assert!((n.1 - 0.42857).abs() < delta);
        assert!((n.2 + 0.85714).abs() < delta);
    }

    #[test]
    fn just_a_quick_test() {
        let mut g1 = Object::group();
        g1.set_transform(Matrix::rotation_y(std::f64::consts::PI / 2.0));
        let mut g2 = Object::group();
        g2.set_transform(Matrix::scaling(1.0, 2.0, 3.0));
        g1.add_child(&mut g2);
        println!("{:?}", g1.get_transform());
        println!("");
        g1.set_transform(&Matrix::translation(5.,5.,3.) * g1.get_transform());
        println!("{:?}", g1.get_transform());
        println!("");
        println!("{:?}", g2.parent.as_ref().unwrap().get_transform());
    }
}