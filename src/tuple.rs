#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Tuple (pub f64, pub f64, pub f64, pub f64);

impl Tuple {
    pub fn new(a: f64, b: f64, c: f64, d: f64) -> Self {
        Tuple(a, b, c, d)
    }

    pub fn is_point(&self) -> bool {
        self.3 == 1.0
    }

    pub fn is_vector(&self) -> bool {
        self.3 == 0.0
    }

    pub fn point(x: f64, y: f64, z: f64) -> Self {
        Tuple::new(x, y, z, 1.0)
    }

    pub fn vector(x: f64, y: f64, z: f64) -> Tuple {
        Tuple::new(x, y, z, 0.0)
    }

    pub fn color(r: f64, g: f64, b: f64) -> Tuple {
        Tuple::new(r, g, b, 0.0)
    }

    pub fn magnitude(&self) -> f64 {
        (self.0.powi(2) + self.1.powi(2) + self.2.powi(2) + self.3.powi(2)).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let mag = self.magnitude();
        Tuple::new(self.0 / mag, self.1 / mag, self.2 / mag, self.3 / mag)
    }

    pub fn dot(&self, other: Tuple) -> f64 {
        self.0 * other.0 + self.1 * other.1 + self.2 * other.2 + self.3 * other.3
    }

    pub fn cross(&self, other: Tuple) -> Tuple {
        Tuple::vector(
            self.1 * other.2 - self.2 * other.1,
            self.2 * other.0 - self.0 * other.2,
            self.0 * other.1 - self.1 * other.0,
        )
    }

    pub fn reflect(&self, normal: Tuple) -> Tuple {
        *self - normal * 2.0 * self.dot(normal)
    }
}

pub type Point = Tuple;
pub type Vector = Tuple;
pub type Color = Tuple;

impl std::ops::Add for Tuple {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Tuple::new(
            self.0 + other.0,
            self.1 + other.1,
            self.2 + other.2,
            self.3 + other.3,
        )
    }
}

impl std::ops::Sub for Tuple {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Tuple::new(
            self.0 - other.0,
            self.1 - other.1,
            self.2 - other.2,
            self.3 - other.3,
        )
    }
}

impl std::ops::Neg for Tuple {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Tuple::new(-self.0, -self.1, -self.2, -self.3)
    }
}

impl std::ops::Mul<f64> for Tuple {
    type Output = Self;

    fn mul(self, scalar: f64) -> Self {
        Tuple::new(
            self.0 * scalar,
            self.1 * scalar,
            self.2 * scalar,
            self.3 * scalar,
        )
    }
}

impl std::ops::Mul for Tuple {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Tuple::new(
            self.0 * other.0,
            self.1 * other.1,
            self.2 * other.2,
            self.3 * other.3,
        )
    }
}

impl std::ops::Div<f64> for Tuple {
    type Output = Self;

    fn div(self, scalar: f64) -> Self {
        Tuple::new(
            self.0 / scalar,
            self.1 / scalar,
            self.2 / scalar,
            self.3 / scalar,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tuple_with_w_1_is_a_point() {
        let a = Tuple::new(4.3, -4.2, 3.1, 1.0);
        assert!(a.is_point());
        assert!(!a.is_vector());
    }

    #[test]
    fn tuple_with_w_0_is_a_vector() {
        let a = Tuple::new(4.3, -4.2, 3.1, 0.0);
        assert!(!a.is_point());
        assert!(a.is_vector());
    }

    #[test]
    fn point_creates_tuple_with_w_1() {
        let p = Tuple::point(4.0, -4.0, 3.0);
        assert_eq!(p, Tuple::new(4.0, -4.0, 3.0, 1.0));
    }

    #[test]
    fn vector_creates_tuple_with_w_0() {
        let v = Tuple::vector(4.0, -4.0, 3.0);
        assert_eq!(v, Tuple::new(4.0, -4.0, 3.0, 0.0));
    }

    #[test]
    fn adding_two_tuples() {
        let a1 = Tuple::new(3.0, -2.0, 5.0, 1.0);
        let a2 = Tuple::new(-2.0, 3.0, 1.0, 0.0);
        assert_eq!(a1 + a2, Tuple::new(1.0, 1.0, 6.0, 1.0));
    }

    #[test]
    fn adding_two_vectors() {
        let a1 = Tuple::vector(3.0, -2.0, 5.0);
        let a2 = Tuple::vector(-2.0, 3.0, 1.0);
        assert_eq!(a1 + a2, Tuple::vector(1.0, 1.0, 6.0));
    }
    
    #[test]
    fn subtracting_two_points() {
        let p1 = Tuple::point(3.0, 2.0, 1.0);
        let p2 = Tuple::point(5.0, 6.0, 7.0);
        assert_eq!(p1 - p2, Tuple::vector(-2.0, -4.0, -6.0));
    }

    #[test]
    fn subtracting_vector_from_point() {
        let p = Tuple::point(3.0, 2.0, 1.0);
        let v = Tuple::vector(5.0, 6.0, 7.0);
        assert_eq!(p - v, Tuple::point(-2.0, -4.0, -6.0));
    }

    #[test]
    fn subtracting_two_vectors() {
        let v1 = Tuple::vector(3.0, 2.0, 1.0);
        let v2 = Tuple::vector(5.0, 6.0, 7.0);
        assert_eq!(v1 - v2, Tuple::vector(-2.0, -4.0, -6.0));
    }

    #[test]
    fn subtracting_vector_from_zero_vector() {
        let zero = Tuple::vector(0.0, 0.0, 0.0);
        let v = Tuple::vector(1.0, -2.0, 3.0);
        assert_eq!(zero - v, Tuple::vector(-1.0, 2.0, -3.0));
    }

    #[test]
    fn negating_a_tuple() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
        assert_eq!(-a, Tuple::new(-1.0, 2.0, -3.0, 4.0));
    }

    #[test]
    fn negating_a_vector() {
        let a = Tuple::vector(1.0, -2.0, 3.0);
        assert_eq!(-a, Tuple::vector(-1.0, 2.0, -3.0));
    }

    #[test]
    fn multiplying_a_tuple_by_a_scalar() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
        assert_eq!(a * 3.5, Tuple::new(3.5, -7.0, 10.5, -14.0));
    }

    #[test]
    fn multiplying_a_tuple_by_a_fraction() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
        assert_eq!(a * 0.5, Tuple::new(0.5, -1.0, 1.5, -2.0));
    }

    #[test]
    fn dividing_a_tuple_by_a_scalar() {
        let a = Tuple::new(1.0, -2.0, 3.0, -4.0);
        assert_eq!(a / 2.0, Tuple::new(0.5, -1.0, 1.5, -2.0));
    }

    #[test]
    fn magnitude_of_vector_1_0_0() {
        let v = Tuple::vector(1.0, 0.0, 0.0);
        assert_eq!(v.magnitude(), 1.0);
    }

    #[test]
    fn magnitude_of_vector_0_1_0() {
        let v = Tuple::vector(0.0, 1.0, 0.0);
        assert_eq!(v.magnitude(), 1.0);
    }

    #[test]
    fn magnitude_of_vector_0_0_1() {
        let v = Tuple::vector(0.0, 0.0, 1.0);
        assert_eq!(v.magnitude(), 1.0);
    }

    #[test]
    fn magnitude_of_vector_1_2_3() {
        let v = Tuple::vector(1.0, 2.0, 3.0);
        assert_eq!(v.magnitude(), 14.0_f64.sqrt());
    }

    #[test]
    fn magnitude_of_vector_neg_1_neg_2_neg_3() {
        let v = Tuple::vector(-1.0, -2.0, -3.0);
        assert_eq!(v.magnitude(), 14.0_f64.sqrt());
    }

    #[test]
    fn normalizing_vector_4_0_0_gives_1_0_0() {
        let v = Tuple::vector(4.0, 0.0, 0.0);
        assert_eq!(v.normalize(), Tuple::vector(1.0, 0.0, 0.0));
    }

    #[test]
    fn normalizing_vector_1_2_3() {
        let v = Tuple::vector(1.0, 2.0, 3.0);
        assert_eq!(v.normalize(), Tuple::vector(1.0 / 14.0_f64.sqrt(), 2.0 / 14.0_f64.sqrt(), 3.0 / 14.0_f64.sqrt()));
    }

    #[test]
    fn magnitude_of_normalized_vector() {
        let v = Tuple::vector(1.0, 2.0, 3.0);
        let norm = v.normalize();
        assert_eq!(norm.magnitude(), 1.0);
    }

    #[test]
    fn dot_product_of_two_tuples() {
        let a = Tuple::vector(1.0, 2.0, 3.0);
        let b = Tuple::vector(2.0, 3.0, 4.0);
        assert_eq!(a.dot(b), 20.0);
    }

    #[test]
    fn cross_product_of_two_vectors() {
        let a = Tuple::vector(1.0, 2.0, 3.0);
        let b = Tuple::vector(2.0, 3.0, 4.0);
        assert_eq!(a.cross(b), Tuple::vector(-1.0, 2.0, -1.0));
    }

    #[test]
    fn colors_are_tuples() {
        let c = Tuple::color(-0.5, 0.4, 1.7);
        assert_eq!(c, Tuple::new(-0.5, 0.4, 1.7, 0.0));
    }

    #[test]
    fn adding_colors() {
        let c1 = Tuple::color(0.9, 0.6, 0.75);
        let c2 = Tuple::color(0.7, 0.1, 0.25);
        assert_eq!(c1 + c2, Tuple::color(1.6, 0.7, 1.0));
    }

    #[test]
    fn subtracting_colors() {
        let c1 = Tuple::color(0.9, 0.6, 0.75);
        let c2 = Tuple::color(0.7, 0.1, 0.25);
        assert_eq!(c1 - c2, Tuple::color(0.2, 0.5, 0.5));
    }

    #[test]
    fn multiplying_a_color_by_a_scalar() {
        let c = Tuple::color(0.2, 0.3, 0.4);
        assert_eq!(c * 2.0, Tuple::color(0.4, 0.6, 0.8));
    }

    #[test]
    fn multiplying_colors() {
        let c1 = Tuple::color(1.0, 0.2, 0.4);
        let c2 = Tuple::color(0.9, 1.0, 0.1);
        let epsilon = 1e-10;
        let result = c1 * c2;
        assert!((result.0 - 0.9).abs() < epsilon);
        assert!((result.1 - 0.2).abs() < epsilon);
        assert!((result.2 - 0.04).abs() < epsilon);
    }

    #[test]
    fn reflecting_a_vector_approaching_at_45_degrees() {
        let v = Tuple::vector(1.0, -1.0, 0.0);
        let n = Tuple::vector(0.0, 1.0, 0.0);
        let r = v.reflect(n);
        assert_eq!(r, Tuple::vector(1.0, 1.0, 0.0));
    }

    #[test]
    fn reflecting_a_vector_off_a_slanted_surface() {
        let v = Tuple::vector(0.0, -1.0, 0.0);
        let n = Tuple::vector(2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0, 0.0);
        let r = v.reflect(n);
        let epsilon = 1e-10;
        assert!((r.0 - 1.0).abs() < epsilon);
        assert!((r.1 - 0.0).abs() < epsilon);
        assert!((r.2 - 0.0).abs() < epsilon);
    }

}
