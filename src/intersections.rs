use crate::intersection::Intersection;

pub trait Intersections {
    fn hit(&self) -> Option<&Intersection>;
}

impl Intersections for Vec<Intersection> {
    fn hit(&self) -> Option<&Intersection> {
        self.iter()
            .filter(|i| i.t >= 0.0)
            .min_by(|a, b| a.t.partial_cmp(&b.t)
            .unwrap_or(std::cmp::Ordering::Greater))
    }
}

#[cfg(test)]
mod tests {
    use std::vec;
    use crate::{ray::Ray, shape::Shape, tuple::Tuple};

    use super::*;

    #[test]
    fn an_intersection_encapsulates_t_and_object() {
        let s = Shape::sphere();
        let i = Intersection::new(3.5, s.clone());
        assert_eq!(i.t, 3.5);
        assert_eq!(i.object, s);
    }

    #[test]
    fn aggregating_intersections() {
        let s = Shape::sphere();
        let i1 = Intersection::new(1.0, s.clone());
        let i2 = Intersection::new(2.0, s.clone());
        let xs = vec![i1, i2];
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 1.0);
        assert_eq!(xs[1].t, 2.0);
    }

    #[test]
    fn hit_when_all_intersections_have_positive_t() {
        let s = Shape::sphere();
        let i1 = Intersection::new(1.0, s.clone());
        let i2 = Intersection::new(2.0, s.clone());
        let xs = vec![i1.clone(), i2];
        let i = xs.hit();
        assert_eq!(i.map(|i| i.t), Some(i1.t));
    }

    #[test]
    fn hit_when_some_intersections_have_negative_t() {
        let s = Shape::sphere();
        let i1 = Intersection::new(-1.0, s.clone());
        let i2 = Intersection::new(1.0, s.clone());
        let xs = vec![i1, i2.clone()];
        let i = xs.hit();
        assert_eq!(i.map(|i| i.t), Some(i2.t));
    }

    #[test]
    fn hit_when_all_intersections_have_negative_t() {
        let s = Shape::sphere();
        let i1 = Intersection::new(-2.0, s.clone());
        let i2 = Intersection::new(-1.0, s.clone());
        let xs = vec![i1, i2];
        let i = xs.hit();
        assert_eq!(i, None);
    }

    #[test]
    fn hit_is_always_the_lowest_nonnegative_intersection() {
        let s = Shape::sphere();
        let i1 = Intersection::new(5.0, s.clone());
        let i2 = Intersection::new(7.0, s.clone());
        let i3 = Intersection::new(-3.0, s.clone());
        let i4 = Intersection::new(2.0, s.clone());
        let xs = vec![i1, i2, i3, i4.clone()];
        let i = xs.hit();
        assert_eq!(i.map(|i| i.t), Some(i4.t));
    }

    #[test]
    fn precomputing_the_state_of_an_intersection() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let shape = Shape::sphere();
        let i = Intersection::new(4.0, shape.clone());
        let comps = i.prepare_computations(&r);
        assert_eq!(comps.t, i.t);
        assert_eq!(comps.object, i.object);
        assert_eq!(comps.point, Tuple::point(0.0, 0.0, -1.0));
        assert_eq!(comps.eyev, Tuple::vector(0.0, 0.0, -1.0));
        assert_eq!(comps.normalv, Tuple::vector(0.0, 0.0, -1.0));
    }

    #[test]
    fn the_hit_when_an_intersection_occurs_on_the_outside() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let shape = Shape::sphere();
        let i = Intersection::new(4.0, shape.clone());
        let comps = i.prepare_computations(&r);
        assert_eq!(comps.inside, false);
    }

    #[test]
    fn the_hit_when_an_intersection_occurs_on_the_inside() {
        let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
        let shape = Shape::sphere();
        let i = Intersection::new(1.0, shape.clone());
        let comps = i.prepare_computations(&r);
        assert_eq!(comps.point, Tuple::point(0.0, 0.0, 1.0));
        assert_eq!(comps.eyev, Tuple::vector(0.0, 0.0, -1.0));
        assert_eq!(comps.inside, true);
        assert_eq!(comps.normalv, Tuple::vector(0.0, 0.0, -1.0));
    }

    // #[test]
    // fn the_hit_should_offset_the_point() {
    //     let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
    //     let shape = Sphere::new().with_transform(Matrix::translation(0.0, 0.0, 1.0));
    //     let i = Intersection::new(5.0, &shape);
    //     let comps = i.prepare_computations(&r);
    //     assert!(comps.over_point.2 < -std::f64::EPSILON / 2.0);
    //     assert!(comps.point.2 > comps.over_point.2);
    // }
}