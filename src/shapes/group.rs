use crate::{intersection::Intersection, object::Object, ray::Ray};

#[derive(Debug, PartialEq, Clone)]
pub struct Group {  
    children: Vec<Object>
}

impl Group {
    pub fn new() -> Group {
        Group {
            children: vec![]
        }
    }

    pub fn local_intersect(&self, ray: &Ray) -> Vec<Intersection> {
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
        let g = Object::group();
        let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
        let xs = g.intersect( &r);
        assert_eq!(xs.len(), 0);
    }

    #[test]
    fn intersecting_a_ray_with_a_nonempty_group() {
        let mut g = Object::group();
        let s1 = Object::sphere();
        let s2 = Object::sphere().with_transform(Matrix::translation(0.,0.,-3.));
        let s3 = Object::sphere().with_transform(Matrix::translation(5.0, 0.0, 0.0));
        
        if let Some(group) = g.as_group() {
            group.add_child(s1.clone());
            group.add_child(s2.clone());
            group.add_child(s3.clone());

            let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
            let xs = g.intersect(&r);
            assert_eq!(xs.len(), 4);
            assert_eq!(xs[0].object, &s2);
            assert_eq!(xs[1].object, &s2);
            assert_eq!(xs[2].object, &s1);
            assert_eq!(xs[3].object, &s1);
        }
    }

    #[test]
    fn intersecting_a_transformed_group() {
        let mut g = Object::group()
            .with_transform(Matrix::scaling(2.0, 2.0, 2.0));
        let s = Object::sphere().with_transform(Matrix::translation(5.0, 0.0, 0.0));
        if let Some(group) = g.as_group() {
            group.add_child(s.clone());
            let r = Ray::new(Tuple::point(10.0, 0.0, -10.0), Tuple::vector(0.0, 0.0, 1.0));
            let xs = g.intersect(&r);
            assert_eq!(xs.len(), 2);
        }
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

