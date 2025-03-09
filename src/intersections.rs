use crate::intersection::Intersection;

pub trait Intersections {
    fn hit(&self) -> Option<&Intersection>;
}

impl<'a> Intersections for Vec<Intersection<'a>> {
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
    use crate::{helper::glass_sphere, matrix::Matrix, object::Object, ray::Ray, transformation::Transformation, tuple::Tuple};

    use super::*;

    #[test]
    fn an_intersection_encapsulates_t_and_object() {
        let s = Object::sphere();
        let i = Intersection::new(3.5, &s);
        assert_eq!(i.t, 3.5);
        assert_eq!(*i.object, s);
    }

    #[test]
    fn aggregating_intersections() {
        let s = Object::sphere();
        let i1 = Intersection::new(1.0, &s);
        let i2 = Intersection::new(2.0, &s);
        let xs = vec![i1, i2];
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 1.0);
        assert_eq!(xs[1].t, 2.0);
    }

    #[test]
    fn hit_when_all_intersections_have_positive_t() {
        let s = Object::sphere();
        let i1 = Intersection::new(1.0, &s);
        let i2 = Intersection::new(2.0, &s);
        let xs = vec![i1.clone(), i2];
        let i = xs.hit();
        assert_eq!(i.map(|i| i.t), Some(i1.t));
    }

    #[test]
    fn hit_when_some_intersections_have_negative_t() {
        let s = Object::sphere();
        let i1 = Intersection::new(-1.0, &s);
        let i2 = Intersection::new(1.0, &s);
        let xs = vec![i1, i2.clone()];
        let i = xs.hit();
        assert_eq!(i.map(|i| i.t), Some(i2.t));
    }

    #[test]
    fn hit_when_all_intersections_have_negative_t() {
        let s = Object::sphere();
        let i1 = Intersection::new(-2.0, &s);
        let i2 = Intersection::new(-1.0, &s);
        let xs = vec![i1, i2];
        let i = xs.hit();
        assert_eq!(i, None);
    }

    #[test]
    fn hit_is_always_the_lowest_nonnegative_intersection() {
        let s = Object::sphere();
        let i1 = Intersection::new(5.0, &s);
        let i2 = Intersection::new(7.0, &s);
        let i3 = Intersection::new(-3.0, &s);
        let i4 = Intersection::new(2.0, &s);
        let xs = vec![i1, i2, i3, i4.clone()];
        let i = xs.hit();
        assert_eq!(i.map(|i| i.t), Some(i4.t));
    }

    #[test]
    fn precomputing_the_state_of_an_intersection() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let shape = Object::sphere();
        let i = Intersection::new(4.0, &shape);
        let comps = i.prepare_computations(&r, &vec![]);
        assert_eq!(comps.t, i.t);
        assert_eq!(comps.object, *i.object);
        assert_eq!(comps.point, Tuple::point(0.0, 0.0, -1.0));
        assert_eq!(comps.eyev, Tuple::vector(0.0, 0.0, -1.0));
        assert_eq!(comps.normalv, Tuple::vector(0.0, 0.0, -1.0));
    }

    #[test]
    fn the_hit_when_an_intersection_occurs_on_the_outside() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let shape = Object::sphere();
        let i = Intersection::new(4.0, &shape);
        let comps = i.prepare_computations(&r, &vec![]);
        assert_eq!(comps.inside, false);
    }

    #[test]
    fn the_hit_when_an_intersection_occurs_on_the_inside() {
        let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 0.0, 1.0));
        let shape = Object::sphere();
        let i = Intersection::new(1.0, &shape);
        let comps = i.prepare_computations(&r, &vec![]);
        assert_eq!(comps.point, Tuple::point(0.0, 0.0, 1.0));
        assert_eq!(comps.eyev, Tuple::vector(0.0, 0.0, -1.0));
        assert_eq!(comps.inside, true);
        assert_eq!(comps.normalv, Tuple::vector(0.0, 0.0, -1.0));
    }

    #[test]
    fn the_hit_should_offset_the_point() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let shape = Object::sphere().with_transform(Matrix::translation(0.0, 0.0, 1.0));
        let i = Intersection::new(5.0, &shape);
        let comps = i.prepare_computations(&r, &vec![]);
        assert!(comps.over_point.2 < -std::f64::EPSILON / 2.0);
        assert!(comps.point.2 > comps.over_point.2);
    }

    #[test]
    fn precomputing_the_reflection_vector() {
        let shape = Object::plane();
        let r = Ray::new(Tuple::point(0.0, 1.0, -1.0), Tuple::vector(0.0, -2_f64.sqrt() / 2.0, 2_f64.sqrt() / 2.0));
        let i = Intersection::new(2_f64.sqrt(), &shape);
        let comps = i.prepare_computations(&r, &vec![]);
        assert_eq!(comps.reflectv, Tuple::vector(0.0, 2_f64.sqrt() / 2.0, 2_f64.sqrt() / 2.0));
    }

    #[test]
    fn finding_n1_and_n2_at_various_intersections() {
        let mut a = glass_sphere()
            .with_transform(Matrix::scaling(2.0, 2.0, 2.0));
        a.set_material(a.get_material().clone().with_refractive_index(1.5));

        let mut b = glass_sphere()
            .with_transform(Matrix::translation(0.0, 0.0, -0.25));
        b.set_material(b.get_material().clone().with_refractive_index(2.0));

        let mut c = glass_sphere()
            .with_transform(Matrix::translation(0.0, 0.0, 0.25));
        c.set_material(c.get_material().clone().with_refractive_index(2.5));

        let r = Ray::new(Tuple::point(0.0, 0.0, -4.0), Tuple::vector(0.0, 0.0, 1.0));
        let xs = vec![
            Intersection::new(2.0, &a),
            Intersection::new(2.75, &b),
            Intersection::new(3.25, &c),
            Intersection::new(4.75, &b),
            Intersection::new(5.25, &c),
            Intersection::new(6.0, &a),
        ];

        let mut comps = xs[0].prepare_computations(&r, &xs);
        assert_eq!(comps.n1, 1.0);
        assert_eq!(comps.n2, 1.5);

        comps = xs[1].prepare_computations(&r, &xs);
        assert_eq!(comps.n1, 1.5);
        assert_eq!(comps.n2, 2.0);

        comps = xs[2].prepare_computations(&r, &xs);
        assert_eq!(comps.n1, 2.0);
        assert_eq!(comps.n2, 2.5);

        comps = xs[3].prepare_computations(&r, &xs);
        assert_eq!(comps.n1, 2.5);
        assert_eq!(comps.n2, 2.5);

        comps = xs[4].prepare_computations(&r, &xs);
        assert_eq!(comps.n1, 2.5);
        assert_eq!(comps.n2, 1.5);

        comps = xs[5].prepare_computations(&r, &xs);
        assert_eq!(comps.n1, 1.5);
        assert_eq!(comps.n2, 1.0);
    }

    #[test]
    fn the_under_point_is_offset_below_the_surface() {
        let r = Ray::new(Tuple::point(0.0, 0.0, -5.0), Tuple::vector(0.0, 0.0, 1.0));
        let shape = glass_sphere().with_transform(Matrix::translation(0.0, 0.0, 1.0));
        let i = Intersection::new(5.0, &shape);
        let xs = vec![i.clone()];
        let comps = i.prepare_computations(&r, &xs);
        assert!(comps.under_point.2 > std::f64::EPSILON / 2.0);
        assert!(comps.point.2 < comps.under_point.2);
    }

    #[test]
    fn the_schlick_approximation_under_total_internal_reflection() {
        let shape = glass_sphere();
        let r = Ray::new(Tuple::point(0.0, 0.0, 2_f64.sqrt() / 2.0), Tuple::vector(0.0, 1.0, 0.0));
        let xs = vec![
            Intersection::new(-2_f64.sqrt() / 2.0, &shape),
            Intersection::new(2_f64.sqrt() / 2.0, &shape),
        ];
        let comps = xs[1].prepare_computations(&r, &xs);
        let reflectance = comps.schlick;
        assert_eq!(reflectance, 1.0);
    }

    #[test]
    fn the_schlick_approximation_with_a_perpendicular_viewing_angle() {
        let shape = glass_sphere();
        let r = Ray::new(Tuple::point(0.0, 0.0, 0.0), Tuple::vector(0.0, 1.0, 0.0));
        let xs = vec![
            Intersection::new(-1.0, &shape),
            Intersection::new(1.0, &shape),
        ];
        let comps = xs[1].prepare_computations(&r, &xs);
        let reflectance = comps.schlick;
        let alpha = 1e-4;
        assert!((reflectance - 0.04).abs() < alpha);
    }

    #[test]
    fn the_schlick_approximation_with_small_angle_and_n2_greater_than_n1() {
        let shape = glass_sphere();
        let r = Ray::new(Tuple::point(0.0, 0.99, -2.0), Tuple::vector(0.0, 0.0, 1.0));
        let xs = vec![
            Intersection::new(1.8589, &shape),
        ];
        let comps = xs[0].prepare_computations(&r, &xs);
        let reflectance = comps.schlick;
        let alpha = 1e-4;
        assert!((reflectance - 0.48873).abs() < alpha);
    }
}