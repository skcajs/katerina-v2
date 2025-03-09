use crate::{intersection::Intersection, matrix::Matrix, object::Object, ray::Ray};

#[derive(Debug, PartialEq, Clone)]
pub struct Group {  // Add lifetime parameter here
    children: Vec<Object>, // Adjust for Object lifetime
    transform: Matrix,
}

impl Group {  // Make the lifetime parameter explicit here
    pub fn new() -> Group {
        Group {
            children: vec![],
            transform: Matrix::identity(),
        }
    }

    pub fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        let mut xs: Vec<Intersection> = vec![];
        for child in &self.children {
            let mut child_xs = child.intersect(ray);
            xs.append(&mut child_xs);
        }
        xs.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        xs
    }

    pub fn get_children(&self) -> &Vec<Object> {
        &self.children
    }

    pub fn add_child(&mut self, child: Object) {
        self.children.push(child);
    }

    pub fn get_transform(&self) -> &Matrix {
        &self.transform
    }

    pub fn set_transform(&mut self, transform: Matrix) {
        self.transform = transform;
    }

    pub fn with_transform(&self, transform: Matrix) -> Group {
        let mut new_sphere = self.clone();
        new_sphere.set_transform(transform);
        new_sphere
    }
}

#[cfg(test)]
mod tests {
    use crate::{matrix::Matrix, ray::Ray, transformation::Transformation, tuple::Tuple};

    use super::*;

    #[test]
    fn creating_a_new_group() {
        let g = Group::new();
        assert_eq!(g.get_children().len(), 0);
    }

    #[test]
    fn adding_a_child_to_a_group() {
        let mut g = Group::new();
        let s = Object::test_shape();
        g.add_child(s.clone());
        assert_eq!(g.get_children().len(), 1);
        assert_eq!(g.get_children()[0], s);
    }

    #[test]
    fn intersecting_a_ray_with_an_empty_group() {
        let g = Group::new();
        let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
        let xs = g.intersect( &r);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn intersecting_a_ray_with_a_nonempty_group() {
        let mut g = Group::new();
        let s1 = Object::sphere();
        let s2 = Object::sphere().with_transform(Matrix::translation(0.,0.,-3.));
        let s3 = Object::sphere().with_transform(Matrix::translation(5.0, 0.0, 0.0));
        g.add_child(s1.clone());
        g.add_child(s2.clone());
        g.add_child(s3.clone());
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let xs = g.intersect(&r);
        assert_eq!(xs.len(), 4);
        assert_eq!(xs[0].object, &s2);
        assert_eq!(xs[1].object, &s2);
        assert_eq!(xs[2].object, &s1);
        assert_eq!(xs[3].object, &s1);
    }

    #[test]
    fn intersecting_a_transformed_group() {
        let mut g = Group::new();
        g.set_transform(Matrix::scaling(2.0, 2.0, 2.0));
        let s = Object::sphere().with_transform(Matrix::translation(5.0, 0.0, 0.0));
        g.add_child(s.clone());
        let r = Ray::new(Tuple::point(10.0, 0.0, -10.0), Tuple::vector(0.0, 0.0, 1.0));
        let xs = g.intersect(&r);
        assert_eq!(xs.len(), 2);
    }
}

