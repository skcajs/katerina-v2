use crate::{keys::ObjectKey, object_store::ObjectStore};

use crate::{
    intersection::Intersection,
    material::Material,
    matrix::Matrix,
    ray::Ray,
    shape::Shape,
    shapes::{
        cone::Cone, cube::Cube, cylinder::Cylinder, group::Group, plane::Plane, sphere::Sphere,
        test_shape::TestShape,
    },
    tuple::{Point, Vector},
};

#[derive(Clone, Debug)]
pub struct Object {
    pub key: Option<ObjectKey>,
    pub shape: Shape,
    pub transform: Matrix,
    pub material: Material,
    pub parent: Option<ObjectKey>,
}

impl Object {
    pub fn new(shape: Shape) -> Self {
        Object {
            key: None,
            shape,
            transform: Matrix::identity(),
            material: Material::new(),
            parent: None,
        }
    }

    pub fn test_shape() -> Self {
        Object::new(Shape::TestShape(TestShape::new()))
    }

    pub fn sphere() -> Self {
        Object::new(Shape::Sphere(Sphere::new()))
    }

    pub fn plane() -> Self {
        Object::new(Shape::Plane(Plane::new()))
    }

    pub fn cube() -> Self {
        Object::new(Shape::Cube(Cube::new()))
    }

    pub fn cylinder() -> Self {
        Object::new(Shape::Cylinder(Cylinder::new()))
    }

    pub fn cone() -> Self {
        Object::new(Shape::Cone(Cone::new()))
    }

    pub fn group() -> Self {
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
        if let Some(parent_key) = self.parent {
            self.shape
                .local_intersect(parent_key, &ray.transform(&self.transform.inverse()))
        } else {
            vec![]
        }
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
        let objects = ObjectStore::get_object_store();
        if let Some(parent_key) = self.parent {
            if let Some(parent) = objects.get(parent_key) {
                let point = parent.world_to_object(world_point);
                self.transform.inverse() * point
            } else {
                *world_point
            }
        } else {
            self.transform.inverse() * *world_point
        }
    }

    pub fn normal_to_world(&self, object_normal: &Vector) -> Vector {
        let objects = ObjectStore::get_object_store();
        let mut world_normal = self.transform.inverse().transpose() * *object_normal;

        world_normal.3 = 0.0;

        world_normal = world_normal.normalize();

        if let Some(parent_key) = self.parent {
            if let Some(parent) = objects.get(parent_key) {
                world_normal = parent.normal_to_world(&world_normal);
            }
        }

        world_normal
    }

    pub fn add_child(&mut self, child_key: ObjectKey) {
        if let Shape::Group(ref mut group) = self.shape {
            group.children.push(child_key);
            self.parent = Some(child_key);
        }
    }

    pub fn get_children(&self) -> &Vec<ObjectKey> {
        if let Shape::Group(ref group) = self.shape {
            &group.children
        } else {
            panic!("Object is not a group");
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    use crate::material::Material;
    use crate::matrix::Matrix;
    use crate::ray::Ray;
    use crate::transformation::Transformation;
    use crate::tuple::Tuple;

    #[test]
    fn default_transform() {
        let objects = ObjectStore::get_object_store();
        let s_key = objects.test_shape();
        if let Some(s) = objects.get(s_key) {
            assert_eq!(s.get_transform(), &Matrix::identity());
        }
    }

    #[test]
    fn assigning_a_transform() {
        let objects = ObjectStore::get_object_store();
        let s_key = objects.test_shape();
        if let Some(mut s) = objects.get(s_key) {
            s.set_transform(Matrix::translation(2.0, 3.0, 4.0));
            assert_eq!(s.get_transform(), &Matrix::translation(2.0, 3.0, 4.0));
        }
    }

    #[test]
    fn default_material() {
        let objects = ObjectStore::new();
        let s_key = objects.test_shape();
        if let Some(s) = objects.get(s_key) {
            assert_eq!(s.get_material(), &Material::new());
        }
    }

    #[test]
    fn assigning_a_material() {
        let objects = ObjectStore::new();
        let s_key = objects.test_shape();
        if let Some(mut s) = objects.get(s_key) {
            let mut m = Material::new();
            m.ambient = 1.0;
            s.set_material(m.clone());
            assert_eq!(s.get_material(), &m);
        }
    }

    #[test]
    fn intersecting_a_scaled_shape_with_a_ray() {
        let objects = ObjectStore::new();
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        if let Some(mut s) = objects.get(objects.test_shape()) {
            s.set_transform(Matrix::scaling(2.0, 2.0, 2.0));
            let xs = s.intersect(&r);
            assert_eq!(s.get_transform(), &Matrix::scaling(2.0, 2.0, 2.0));
            assert_eq!(xs.len(), 0);
        }
    }

    #[test]
    fn intersecting_a_translated_shape_with_a_ray() {
        let objects = ObjectStore::new();
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let s_key = objects.test_shape();
        if let Some(mut s) = objects.get(s_key) {
            s.set_transform(Matrix::translation(5.0, 0.0, 0.0));
            let xs = s.intersect(&r);
            assert_eq!(s.get_transform(), &Matrix::translation(5.0, 0.0, 0.0));
            assert_eq!(xs.len(), 0);
        }
    }

    #[test]
    fn computing_the_normal_on_a_translated_shape() {
        let objects = ObjectStore::new();
        let s_key = objects.test_shape();
        if let Some(mut s) = objects.get(s_key) {
            s.set_transform(Matrix::translation(0.0, 1.0, 0.0));
            let n = s.normal_at(&Tuple::point(0.0, 1.70711, -0.70711));
            let delta = 1e-5;
            assert!((n.0 - 0.0).abs() < delta);
            assert!((n.1 - 0.70711).abs() < delta);
            assert!((n.2 + 0.70711).abs() < delta);
        }
    }

    #[test]
    fn computing_the_normal_on_a_transformed_shape() {
        let objects = ObjectStore::new();
        let s_key = objects.test_shape();
        if let Some(mut s) = objects.get(s_key) {
            let m = Matrix::scaling(1.0, 0.5, 1.0) * Matrix::rotation_z(std::f64::consts::PI / 5.0);
            s.set_transform(m);
            let n = s.normal_at(&Tuple::point(
                0.0,
                2.0_f64.sqrt() / 2.0,
                -2.0_f64.sqrt() / 2.0,
            ));
            let delta = 1e-5;
            assert!((n.0 - 0.0).abs() < delta);
            assert!((n.1 - 0.97014).abs() < delta);
            assert!((n.2 + 0.24254).abs() < delta);
        }
    }

    #[test]
    fn a_shape_has_a_parent_attribute() {
        let objects = ObjectStore::new();
        let s_key = objects.test_shape();
        if let Some(s) = objects.get(s_key) {
            assert_eq!(s.parent, None);
        }
    }

    #[test]
    fn converting_a_point_from_world_to_object_space() {
        let mut objects = ObjectStore::new();

        let g1_key = objects.group();
        let g2_key = objects.group();
        let s_key = objects.sphere();

        objects.set_transform(g1_key, Matrix::rotation_y(std::f64::consts::PI / 2.0));
        objects.set_transform(g2_key, Matrix::scaling(2.0, 2.0, 2.0));
        objects.add_child(g1_key, g2_key);
        objects.set_transform(s_key, Matrix::translation(5.0, 0.0, 0.0));
        objects.add_child(g2_key, s_key);

        let p= objects.world_to_object(s_key, &Tuple::point(-2.0, 0.0, -10.0));

        let delta = 1e-5;
        assert!((p.0 - 0.0).abs() < delta);
        assert!((p.1 - 0.0).abs() < delta);
        assert!((p.2 + 1.0).abs() < delta);
    }

    #[test]
    fn converting_a_normal_from_object_to_world_space() {
        let objects = ObjectStore::new();
        let g1_key = objects.group();
        let g2_key = objects.group();
        let s_key = objects.sphere();

        if let Some(mut g1) = objects.get(g1_key) {
            g1.set_transform(Matrix::rotation_y(std::f64::consts::PI / 2.0));
        }

        if let Some(mut g2) = objects.get(g2_key) {
            g2.set_transform(Matrix::scaling(1.0, 2.0, 3.0));
        }


        if let Some(mut g1) = objects.get(g1_key) {
            g1.add_child(g2_key);
        }


        if let Some(mut s) = objects.get(s_key) {
            s.set_transform(Matrix::translation(5.0, 0.0, 0.0));
        }

        if let Some(mut g2) = objects.get(g2_key) {
            g2.add_child(s_key);
        }

        if let Some(s) = objects.get(s_key) {
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

    }

    // #[test]
    // fn just_a_quick_test() {
    //     let objects = ObjectStore::new();
    //     let g1_key = objects.group();
    //     if let Some(mut g1) = objects.get(g1_key) {
    //         g1.set_transform(Matrix::rotation_y(std::f64::consts::PI / 2.0));
    //         let g2_key = objects.group();
    //         if let Some(mut g2) = objects.get(g2_key) {
    //             g2.set_transform(Matrix::scaling(1.0, 2.0, 3.0));
    //             g1.add_child(&mut g2);
    //             println!("{:?}", g1.get_transform());
    //             println!("");

    //             let transform = g1.get_transform().clone();

    //             g1.set_transform(Matrix::translation(5., 5., 3.) * transform);
    //             println!("{:?}", g1.get_transform());
    //             println!("");
    //             if let Some(parent_key) = g2.parent {
    //                 if let Some(parent) = objects.get(parent_key) {
    //                     println!("{:?}", parent.get_transform());
    //                 }
    //             }
    //         }
    //     }
    // }
}
