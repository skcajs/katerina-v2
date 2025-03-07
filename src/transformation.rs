use crate::{matrix::Matrix, tuple::{Point, Vector}};

pub trait Transformation {
    fn translation(x: f64, y: f64, z: f64) -> Self;
    fn scaling(x: f64, y: f64, z: f64) -> Self;
    fn rotation_x(r: f64) -> Self;
    fn rotation_y(r: f64) -> Self;
    fn rotation_z(r: f64) -> Self;
    fn shearing(
        xy: f64,
        xz: f64,
        yx: f64,
        yz: f64,
        zx: f64,
        zy: f64,
    ) -> Self;
    fn translate(&self, x: f64, y: f64, z: f64) -> Self;
    fn scale(&self, x: f64, y: f64, z: f64) -> Self;
    fn rotate_x(&self, r: f64) -> Self;
    fn rotate_y(&self, r: f64) -> Self;
    fn rotate_z(&self, r: f64) -> Self;
    fn shear(
        &self,
        xy: f64,
        xz: f64,
        yx: f64,
        yz: f64,
        zx: f64,
        zy: f64,
    ) -> Self;
    fn view_transform(from: Point, to: Point, up: Vector) -> Self;

}

impl Transformation for Matrix {
    fn translation(x: f64, y: f64, z: f64) -> Matrix {
        Matrix::new(vec![
            1.0, 0.0, 0.0, x,
            0.0, 1.0, 0.0, y,
            0.0, 0.0, 1.0, z,
            0.0, 0.0, 0.0, 1.0,
        ])
    }

    fn scaling(x: f64, y: f64, z: f64) -> Matrix {
        Matrix::new(vec![
            x, 0.0, 0.0, 0.0,
            0.0, y, 0.0, 0.0,
            0.0, 0.0, z, 0.0,
            0.0, 0.0, 0.0, 1.0,
        ])
    }

    fn rotation_x(r: f64) -> Matrix {
        let cos_r = r.cos();
        let sin_r = r.sin();
    
        Matrix::new(vec![
            1.0, 0.0, 0.0, 0.0,
            0.0, cos_r, -sin_r, 0.0,
            0.0, sin_r, cos_r, 0.0,
            0.0, 0.0, 0.0, 1.0,
        ])
    }

    fn rotation_y(r: f64) -> Matrix {
        let cos_r = r.cos();
        let sin_r = r.sin();
    
        Matrix::new(vec![
            cos_r, 0.0, sin_r, 0.0,
            0.0, 1.0, 0.0, 0.0,
            -sin_r, 0.0, cos_r, 0.0,
            0.0, 0.0, 0.0, 1.0,
        ])
    }
    
    fn rotation_z(r: f64) -> Matrix {
        let cos_r = r.cos();
        let sin_r = r.sin();
    
        Matrix::new(vec![
            cos_r, -sin_r, 0.0, 0.0,
            sin_r, cos_r, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        ])
    }

    fn shearing(
        xy: f64,
        xz: f64,
        yx: f64,
        yz: f64,
        zx: f64,
        zy: f64,
    ) -> Matrix {
        Matrix::new(vec![
            1.0, xy, xz, 0.0,
            yx, 1.0, yz, 0.0,
            zx, zy, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        ])
    }

    fn translate(&self, x: f64, y: f64, z: f64) -> Matrix {
        let translation = Matrix::new(vec![
            1.0, 0.0, 0.0, x,
            0.0, 1.0, 0.0, y,
            0.0, 0.0, 1.0, z,
            0.0, 0.0, 0.0, 1.0,
        ]);

        &translation * self
    }

    fn scale(&self, x: f64, y: f64, z: f64) -> Matrix {
        let scaling = Matrix::new(vec![
            x, 0.0, 0.0, 0.0,
            0.0, y, 0.0, 0.0,
            0.0, 0.0, z, 0.0,
            0.0, 0.0, 0.0, 1.0,
        ]);

        &scaling * self
    }

    fn rotate_x(&self, r: f64) -> Matrix {
        let rotation = Self::rotation_x(r);
        &rotation * self
    }
    fn rotate_y(&self, r: f64) -> Matrix {
        let rotation = Self::rotation_y(r);
        &rotation * self
    }
    fn rotate_z(&self, r: f64) -> Matrix {
        let rotation = Self::rotation_z(r);
        &rotation * self
    }
    fn shear(
        &self,
        xy: f64,
        xz: f64,
        yx: f64,
        yz: f64,
        zx: f64,
        zy: f64,
    ) -> Self {
        let shear = Self::shearing(xy, xz, yx, yz, zx, zy);
        &shear * self
    }

    fn view_transform(from: Point, to: Point, up: Vector) -> Matrix {
        let forward = (to - from).normalize();
        let left = forward.cross(up.normalize());
        let true_up = left.cross(forward);
        let orientation = Matrix::new(vec![
            left.0, left.1, left.2, 0.0,
            true_up.0, true_up.1, true_up.2, 0.0,
            -forward.0, -forward.1, -forward.2, 0.0,
            0.0, 0.0, 0.0, 1.0,
        ]);
        orientation * Matrix::translation(-from.0, -from.1, -from.2)
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::tuple::Tuple;

    #[test]
    fn multiplying_by_a_translation_matrix() {
        let transform = Matrix::translation(5.0, -3.0, 2.0);
        let p = Tuple::point(-3.0, 4.0, 5.0);
        assert_eq!(transform * p, Tuple::point(2.0, 1.0, 7.0));
    }

    #[test]
    fn multiplying_by_the_inverse_of_a_translation_matrix() {
        let transform = Matrix::translation(5.0, -3.0, 2.0);
        let inv = transform.inverse();
        let p = Tuple::point(-3.0, 4.0, 5.0);
        assert_eq!(inv * p, Tuple::point(-8.0, 7.0, 3.0));
    }

    #[test]
    fn translation_does_not_affect_vectors() {
        let transform = Matrix::translation(5.0, -3.0, 2.0);
        let v = Tuple::vector(-3.0, 4.0, 5.0);
        assert_eq!(transform * v, v);
    }

    #[test]
    fn a_scaling_matrix_applied_to_a_point() {
        let transform = Matrix::scaling(2.0, 3.0, 4.0);
        let p = Tuple::point(-4.0, 6.0, 8.0);
        assert_eq!(transform * p, Tuple::point(-8.0, 18.0, 32.0));
    }

    #[test]
    fn a_scaling_matrix_applied_to_a_vector() {
        let transform = Matrix::scaling(2.0, 3.0, 4.0);
        let v = Tuple::vector(-4.0, 6.0, 8.0);
        assert_eq!(transform * v, Tuple::vector(-8.0, 18.0, 32.0));
    }

    #[test]
    fn multiplying_by_the_inverse_of_a_scaling_matrix() {
        let transform = Matrix::scaling(2.0, 3.0, 4.0);
        let inv = transform.inverse();
        let v = Tuple::vector(-4.0, 6.0, 8.0);
        assert_eq!(inv * v, Tuple::vector(-2.0, 2.0, 2.0));
    }

    #[test]
    fn reflection_is_scaling_by_a_negative_value() {
        let transform = Matrix::scaling(-1.0, 1.0, 1.0);
        let p = Tuple::point(2.0, 3.0, 4.0);
        assert_eq!(transform * p, Tuple::point(-2.0, 3.0, 4.0));
    }

    #[test]
    fn rotating_a_point_around_the_x_axis() {
        let p = Tuple::point(0.0, 1.0, 0.0);
        let half_quarter = Matrix::rotation_x(std::f64::consts::PI / 4.0);
        let full_quarter = Matrix::rotation_x(std::f64::consts::PI / 2.0);
        let delta = 1e-10;
        let expected1 = Tuple::point(0.0, 2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0);
        let result1 = half_quarter * p;
        assert!((result1.0 - expected1.0).abs() < delta);
        assert!((result1.1 - expected1.1).abs() < delta);
        assert!((result1.2 - expected1.2).abs() < delta);
        let expected2 = Tuple::point(0.0, 0.0, 1.0);
        let result2 = full_quarter * p;
        assert!((result2.0 - expected2.0).abs() < delta);
        assert!((result2.1 - expected2.1).abs() < delta);
        assert!((result2.2 - expected2.2).abs() < delta);
    }

    #[test]
    fn the_inverse_of_an_x_rotation_rotates_in_the_opposite_direction() {
        let p = Tuple::point(0.0, 1.0, 0.0);
        let half_quarter = Matrix::rotation_x(std::f64::consts::PI / 4.0);
        let inv = half_quarter.inverse();
        let delta = 1e-10;
        let expected = Tuple::point(0.0, 2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0);
        let result = inv * p;
        assert!((result.0 - expected.0).abs() < delta);
        assert!((result.1 - expected.1).abs() < delta);
        assert!((result.2 - expected.2).abs() < delta);
    }

    #[test]
    fn rotating_a_point_around_the_y_axis() {
        let p = Tuple::point(0.0, 0.0, 1.0);
        let half_quarter = Matrix::rotation_y(std::f64::consts::PI / 4.0);
        let full_quarter = Matrix::rotation_y(std::f64::consts::PI / 2.0);
        let delta = 1e-10;
        let expected1 = Tuple::point(2.0_f64.sqrt() / 2.0, 0.0, 2.0_f64.sqrt() / 2.0);
        let result1 = half_quarter * p;
        assert!((result1.0 - expected1.0).abs() < delta);
        assert!((result1.1 - expected1.1).abs() < delta);
        assert!((result1.2 - expected1.2).abs() < delta);
        let expected2 = Tuple::point(1.0, 0.0, 0.0);
        let result2 = full_quarter * p;
        assert!((result2.0 - expected2.0).abs() < delta);
        assert!((result2.1 - expected2.1).abs() < delta);
        assert!((result2.2 - expected2.2).abs() < delta);
    }

    #[test]
    fn rotating_a_point_around_the_z_axis() {
        let p = Tuple::point(0.0, 1.0, 0.0);
        let half_quarter = Matrix::rotation_z(std::f64::consts::PI / 4.0);
        let full_quarter = Matrix::rotation_z(std::f64::consts::PI / 2.0);
        let delta = 1e-10;
        let expected1 = Tuple::point(-2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0, 0.0);
        let result1 = half_quarter * p;
        assert!((result1.0 - expected1.0).abs() < delta);
        assert!((result1.1 - expected1.1).abs() < delta);
        assert!((result1.2 - expected1.2).abs() < delta);
        let expected2 = Tuple::point(-1.0, 0.0, 0.0);
        let result2 = full_quarter * p;
        assert!((result2.0 - expected2.0).abs() < delta);
        assert!((result2.1 - expected2.1).abs() < delta);
        assert!((result2.2 - expected2.2).abs() < delta);
    }

    #[test]
    fn a_shearing_transformation_moves_x_in_proportion_to_y() {
        let transform = Matrix::shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let p = Tuple::point(2.0, 3.0, 4.0);
        assert_eq!(transform * p, Tuple::point(5.0, 3.0, 4.0));
    }

    #[test]
    fn a_shearing_transformation_moves_x_in_proportion_to_z() {
        let transform = Matrix::shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        let p = Tuple::point(2.0, 3.0, 4.0);
        assert_eq!(transform * p, Tuple::point(6.0, 3.0, 4.0));
    }

    #[test]
    fn a_shearing_transformation_moves_y_in_proportion_to_x() {
        let transform = Matrix::shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        let p = Tuple::point(2.0, 3.0, 4.0);
        assert_eq!(transform * p, Tuple::point(2.0, 5.0, 4.0));
    }

    #[test]
    fn a_shearing_transformation_moves_y_in_proportion_to_z() {
        let transform = Matrix::shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        let p = Tuple::point(2.0, 3.0, 4.0);
        assert_eq!(transform * p, Tuple::point(2.0, 7.0, 4.0));
    }

    #[test]
    fn a_shearing_transformation_moves_z_in_proportion_to_x() {
        let transform = Matrix::shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        let p = Tuple::point(2.0, 3.0, 4.0);
        assert_eq!(transform * p, Tuple::point(2.0, 3.0, 6.0));
    }

    #[test]
    fn a_shearing_transformation_moves_z_in_proportion_to_y() {
        let transform = Matrix::shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let p = Tuple::point(2.0, 3.0, 4.0);
        assert_eq!(transform * p, Tuple::point(2.0, 3.0, 7.0));
    }

    #[test]
    fn individual_transformations_are_applied_in_sequence() {
        let p = Tuple::point(1.0, 0.0, 1.0);
        let a = Matrix::rotation_x(std::f64::consts::PI / 2.0);
        let b = Matrix::scaling(5.0, 5.0, 5.0);
        let c = Matrix::translation(10.0, 5.0, 7.0);
        let delta = 1e-10;

        let p2 = a * p;
        let expected_p2 = Tuple::point(1.0, -1.0, 0.0);
        assert!((p2.0 - expected_p2.0).abs() < delta);
        assert!((p2.1 - expected_p2.1).abs() < delta);
        assert!((p2.2 - expected_p2.2).abs() < delta);

        let p3 = b * p2;
        let expected_p3 = Tuple::point(5.0, -5.0, 0.0);
        assert!((p3.0 - expected_p3.0).abs() < delta);
        assert!((p3.1 - expected_p3.1).abs() < delta);
        assert!((p3.2 - expected_p3.2).abs() < delta);

        let p4 = c * p3;
        let expected_p4 = Tuple::point(15.0, 0.0, 7.0);
        assert!((p4.0 - expected_p4.0).abs() < delta);
        assert!((p4.1 - expected_p4.1).abs() < delta);
        assert!((p4.2 - expected_p4.2).abs() < delta);
    }

    #[test]
    fn chained_transformations_must_be_applied_in_reverse_order() {
        let p = Tuple::point(1.0, 0.0, 1.0);
        let a = Matrix::rotation_x(std::f64::consts::PI / 2.0);
        let b = Matrix::scaling(5.0, 5.0, 5.0);
        let c = Matrix::translation(10.0, 5.0, 7.0);
        let t = c * b * a;
        let delta = 1e-10;
        let expected = Tuple::point(15.0, 0.0, 7.0);
        let result = t * p;
        assert!((result.0 - expected.0).abs() < delta);
        assert!((result.1 - expected.1).abs() < delta);
        assert!((result.2 - expected.2).abs() < delta);
    }

    #[test]
    fn the_transformation_matrix_for_the_default_orientation() {
        let from = Tuple::point(0.0, 0.0, 0.0);
        let to = Tuple::point(0.0, 0.0, -1.0);
        let up = Tuple::vector(0.0, 1.0, 0.0);
        let t = Matrix::view_transform(from, to, up);
        assert_eq!(t, Matrix::identity());
    }

    #[test]
    fn a_view_transformation_matrix_looking_in_positive_z_direction() {
        let from = Tuple::point(0.0, 0.0, 0.0);
        let to = Tuple::point(0.0, 0.0, 1.0);
        let up = Tuple::vector(0.0, 1.0, 0.0);
        let t = Matrix::view_transform(from, to, up);
        assert_eq!(t, Matrix::scaling(-1.0, 1.0, -1.0));
    }

    #[test]
    fn the_view_transformation_moves_the_world() {
        let from = Tuple::point(0.0, 0.0, 8.0);
        let to = Tuple::point(0.0, 0.0, 0.0);
        let up = Tuple::vector(0.0, 1.0, 0.0);
        let t = Matrix::view_transform(from, to, up);
        assert_eq!(t, Matrix::translation(0.0, 0.0, -8.0));
    }

    #[test]
    fn an_arbitrary_view_transformation() {
        let from = Tuple::point(1.0, 3.0, 2.0);
        let to = Tuple::point(4.0, -2.0, 8.0);
        let up = Tuple::vector(1.0, 1.0, 0.0);
        let t = Matrix::view_transform(from, to, up);
        let expected = Matrix::new(vec![
            -0.50709, 0.50709, 0.67612, -2.36643,
            0.76772, 0.60609, 0.12122, -2.82843,
            -0.35857, 0.59761, -0.71714, 0.00000,
            0.00000, 0.00000, 0.00000, 1.00000,
        ]);
        assert_eq!(t, expected);
    }

} 