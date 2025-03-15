use crate::keys::ObjectKey;
use crate::object_store::ObjectStore;
use crate::{intersection::Intersection, ray::Ray};

#[derive(Debug, PartialEq, Clone)]
pub struct Group {
    pub children: Vec<ObjectKey>,
}

impl Group {
    pub fn new() -> Group {
        Group { children: vec![] }
    }

    pub fn local_intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let objects = ObjectStore::get_object_store();
        let mut xs: Vec<Intersection> = vec![];
        for child_key in &self.children {
            if let Some(child) = objects.get(*child_key) {
                let mut child_xs = child.intersect(ray);
                xs.append(&mut child_xs);
            }
        }
        xs.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        xs
    }

    pub fn get_children(&self) -> &Vec<ObjectKey> {
        &self.children
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        matrix::Matrix, object::Object, ray::Ray, transformation::Transformation, tuple::Tuple,
    };

    use super::*;

    #[test]
    fn creating_a_new_group() {
        let g = Group::new();
        assert_eq!(g.get_children().len(), 0);
    }

    #[test]
    fn adding_a_child_to_a_group() {
        let objects = ObjectStore::get_object_store();
        let g_key = objects.group();
        let s_key = objects.test_shape();
        let mut g = objects.get(g_key).unwrap();
        let mut s = objects.get(s_key).unwrap();
        g.add_child(&mut s);
        assert_eq!(g.get_children().len(), 1);
        assert_eq!(g.get_children()[0], s_key);
    }

    #[test]
    fn intersecting_a_ray_with_an_empty_group() {
        let objects = ObjectStore::get_object_store();
        let g_key = objects.group();
        let g = objects.get(g_key).unwrap();
        let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
        let xs = g.intersect(&r);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn intersecting_a_ray_with_a_nonempty_group() {
        let objects = ObjectStore::get_object_store();
        let g_key = objects.group();
        let mut g = objects.get(g_key).unwrap();
        let s1_key = objects.sphere();
        let mut s1 = objects.get(s1_key).unwrap();
        let s2_key = objects.add(Object::sphere().with_transform(Matrix::translation(0., 0., -3.)));
        let mut s2 = objects.get(s2_key).unwrap();
        let s3_key =
            objects.add(Object::sphere().with_transform(Matrix::translation(5.0, 0.0, 0.0)));
        let mut s3 = objects.get(s3_key).unwrap();

        g.add_child(&mut s1);
        g.add_child(&mut s2);
        g.add_child(&mut s3);

        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let xs = g.intersect(&r);
        assert_eq!(xs.len(), 4);
        assert_eq!(xs[0].object_key, s2_key);
        assert_eq!(xs[1].object_key, s2_key);
        assert_eq!(xs[2].object_key, s1_key);
        assert_eq!(xs[3].object_key, s1_key);
    }

    #[test]
    fn intersecting_a_transformed_group() {
        let objects = ObjectStore::get_object_store();
        let g_key = objects.group();
        let mut g = objects.get(g_key).unwrap();

        g.set_transform(Matrix::scaling(2.0, 2.0, 2.0));
        let mut s = Object::sphere().with_transform(Matrix::translation(5.0, 0.0, 0.0));

        g.add_child(&mut s);
        let r = Ray::new(Tuple::point(10.0, 0.0, -10.0), Tuple::vector(0.0, 0.0, 1.0));

        let xs = g.intersect(&r);
        assert_eq!(xs.len(), 2);
    }

    // #[test]
    // fn converting_a_point_from_world_to_object_space() {
    //     let mut g1 = Group::new();
    //     g1.set_transform(Matrix::rotation_y(std::f64::consts::PI / 2.0));
    //     let mut g2 = Group::new();
    //     g2.set_transform(Matrix::scaling(2.0, 2.0, 2.0));
    //     g1.add_child(g2);
    //     let s = Object::sphere().with_transform(Matrix::translation(5.0, 0.0, 0.0));
    //     g2.add_child(s.clone());
    //     let p = s.world_to_object(&Tuple::point(-2.0, 0.0, -10.0));
    //     assert_eq!(p, Tuple::point(0.0, 0.0, -1.0));
    // }
}
