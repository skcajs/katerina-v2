use crate::tuple::Tuple;

#[derive(Clone, Debug, PartialEq)]
pub struct Matrix {
    size: usize,
    pub data: Vec<f64>,
}

impl Matrix {
    pub fn new(data: Vec<f64>) -> Matrix {
        let size = (data.len() as f64).sqrt() as usize;
        if size * size != data.len() {
            panic!("Matrix data must be square");
        }
        Matrix { size, data }
    }

    pub fn identity() -> Matrix {
        Matrix::new(vec![
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0,
        ])
    }

    pub fn transpose(&self) -> Matrix {
        let mut data = vec![0.0; self.size * self.size];

        let sd = &self.data;

        match self.size {
            4 => {
                data[0] = sd[0];
                data[1] = sd[4];
                data[2] = sd[8];
                data[3] = sd[12];
                data[4] = sd[1];
                data[5] = sd[5];
                data[6] = sd[9];
                data[7] = sd[13];
                data[8] = sd[2];
                data[9] = sd[6];
                data[10] = sd[10];
                data[11] = sd[14];
                data[12] = sd[3];
                data[13] = sd[7];
                data[14] = sd[11];
                data[15] = sd[15];
            }
            3 => {
                data[0] = sd[0];
                data[1] = sd[3];
                data[2] = sd[6];
                data[3] = sd[1];
                data[4] = sd[4];
                data[5] = sd[7];
                data[6] = sd[2];
                data[7] = sd[5];
                data[8] = sd[8];
            }
            2 => {
                data[0] = sd[0];
                data[1] = sd[2];
                data[2] = sd[1];
                data[3] = sd[3];
            }
            _ => panic!("Cannot transpose matrix of size {}", self.size),
        }

        Matrix::new(data)
    }

    pub fn determinant(&self) -> f64 {
        if self.size == 2 {
            return self.data[0] * self.data[3] - self.data[1] * self.data[2];
        }

        let mut det = 0.0;
        for c in 0..self.size {
            det += self.data[c] * self.cofactor(0, c);
        }
        det
    }

    pub fn submatrix(&self, row: usize, col: usize) -> Matrix {
        let mut data = vec![0.0; (self.size - 1) * (self.size - 1)];

        let mut dest = 0;
        for r in 0..self.size {
            if r == row {
                continue;
            }
            for c in 0..self.size {
                if c == col {
                    continue;
                }
                data[dest] = self.data[r * self.size + c];
                dest += 1;
            }
        }

        Matrix::new(data)
    }

    pub fn minor(&self, row: usize, col: usize) -> f64 {
        self.submatrix(row, col).determinant()
    }

    pub fn cofactor(&self, row: usize, col: usize) -> f64 {
        let minor = self.minor(row, col);
        if (row + col) % 2 == 0 {
            minor
        } else {
            -minor
        }
    }

    pub fn is_invertible(&self) -> bool {
        self.determinant() != 0.0
    }

    pub fn inverse(&self) -> Matrix {
        if !self.is_invertible() {
            panic!("Matrix is not invertible");
        }

        let mut data = vec![0.0; self.size * self.size];
        let det = self.determinant();

        match self.size {
            4 => {
                data[0] = self.cofactor(0, 0) / det;
                data[1] = self.cofactor(1, 0) / det;
                data[2] = self.cofactor(2, 0) / det;
                data[3] = self.cofactor(3, 0) / det;
                data[4] = self.cofactor(0, 1) / det;
                data[5] = self.cofactor(1, 1) / det;
                data[6] = self.cofactor(2, 1) / det;
                data[7] = self.cofactor(3, 1) / det;
                data[8] = self.cofactor(0, 2) / det;
                data[9] = self.cofactor(1, 2) / det;
                data[10] = self.cofactor(2, 2) / det;
                data[11] = self.cofactor(3, 2) / det;
                data[12] = self.cofactor(0, 3) / det;
                data[13] = self.cofactor(1, 3) / det;
                data[14] = self.cofactor(2, 3) / det;
                data[15] = self.cofactor(3, 3) / det;
            }
            3 => {
                data[0] = self.cofactor(0, 0) / det;
                data[1] = self.cofactor(1, 0) / det;
                data[2] = self.cofactor(2, 0) / det;
                data[3] = self.cofactor(0, 1) / det;
                data[4] = self.cofactor(1, 1) / det;
                data[5] = self.cofactor(2, 1) / det;
                data[6] = self.cofactor(0, 2) / det;
                data[7] = self.cofactor(1, 2) / det;
                data[8] = self.cofactor(2, 2) / det;
            }
            2 => {
                data[0] = self.cofactor(0, 0) / det;
                data[1] = self.cofactor(1, 0) / det;
                data[2] = self.cofactor(0, 1) / det;
                data[3] = self.cofactor(1, 1) / det;
            }
            _ => panic!("Cannot invert matrix of size {}", self.size),
        }

        Matrix::new(data)
    }
}

impl std::ops::Mul<&Matrix> for &Matrix {
    type Output = Matrix;

    fn mul(self, other: &Matrix) -> Matrix {
        if self.size != other.size {
            panic!("Cannot multiply matrices of different sizes");
        }

        let mut data = vec![0.0; self.size * self.size];
        let sd = &self.data;
        let od = &other.data;

        match self.size {
            4 => {
                data[0] = sd[0] * od[0] + sd[1] * od[4] + sd[2] * od[8] + sd[3] * od[12];
                data[1] = sd[0] * od[1] + sd[1] * od[5] + sd[2] * od[9] + sd[3] * od[13];
                data[2] = sd[0] * od[2] + sd[1] * od[6] + sd[2] * od[10] + sd[3] * od[14];
                data[3] = sd[0] * od[3] + sd[1] * od[7] + sd[2] * od[11] + sd[3] * od[15];

                data[4] = sd[4] * od[0] + sd[5] * od[4] + sd[6] * od[8] + sd[7] * od[12];
                data[5] = sd[4] * od[1] + sd[5] * od[5] + sd[6] * od[9] + sd[7] * od[13];
                data[6] = sd[4] * od[2] + sd[5] * od[6] + sd[6] * od[10] + sd[7] * od[14];
                data[7] = sd[4] * od[3] + sd[5] * od[7] + sd[6] * od[11] + sd[7] * od[15];

                data[8] = sd[8] * od[0] + sd[9] * od[4] + sd[10] * od[8] + sd[11] * od[12];
                data[9] = sd[8] * od[1] + sd[9] * od[5] + sd[10] * od[9] + sd[11] * od[13];
                data[10] = sd[8] * od[2] + sd[9] * od[6] + sd[10] * od[10] + sd[11] * od[14];
                data[11] = sd[8] * od[3] + sd[9] * od[7] + sd[10] * od[11] + sd[11] * od[15];

                data[12] = sd[12] * od[0] + sd[13] * od[4] + sd[14] * od[8] + sd[15] * od[12];
                data[13] = sd[12] * od[1] + sd[13] * od[5] + sd[14] * od[9] + sd[15] * od[13];
                data[14] = sd[12] * od[2] + sd[13] * od[6] + sd[14] * od[10] + sd[15] * od[14];
                data[15] = sd[12] * od[3] + sd[13] * od[7] + sd[14] * od[11] + sd[15] * od[15];
            }
            // Handle other sizes...
            _ => panic!("Cannot multiply matrices of size {}", self.size),
        }

        Matrix::new(data)
    }
}

impl std::ops::Mul for Matrix {
    type Output = Matrix;

    fn mul(self, other: Matrix) -> Matrix {
        &self * &other
    }
}

impl std::ops::Mul<&Tuple> for &Matrix {
    type Output = Tuple;

    fn mul(self, other: &Tuple) -> Tuple {

        if self.size != 4 {
            panic!("Matrix must be 4x4 to multiply with a tuple");
        }

        let mut result = Tuple::new(0.0, 0.0, 0.0, 0.0);

        let od = other;
        let sd = &self.data;

        result.0 = sd[0] * od.0 + sd[1] * od.1 + sd[2] * od.2 + sd[3] * od.3;
        result.1 = sd[4] * od.0 + sd[5] * od.1 + sd[6] * od.2 + sd[7] * od.3;
        result.2 = sd[8] * od.0 + sd[9] * od.1 + sd[10] * od.2 + sd[11] * od.3;
        result.3 = sd[12] * od.0 + sd[13] * od.1 + sd[14] * od.2 + sd[15] * od.3;

        result
    }
}

impl std::ops::Mul<Tuple> for Matrix {
    type Output = Tuple;

    fn mul(self, other: Tuple) -> Tuple {
        &self * &other
    }
}

#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn creating_a_matrix() {
        let m = Matrix::new(vec![
            1.0, 2.0, 3.0, 4.0,
            5.5, 6.5, 7.5, 8.5,
            9.0, 10.0, 11.0, 12.0,
            13.5, 14.5, 15.5, 16.5,
        ]);

        assert_eq!(m.size, 4);
        assert_eq!(m.data[0], 1.0);
        assert_eq!(m.data[1], 2.0);
        assert_eq!(m.data[4], 5.5);
        assert_eq!(m.data[7], 8.5);
        assert_eq!(m.data[8], 9.0);
        assert_eq!(m.data[10], 11.0);
        assert_eq!(m.data[13], 14.5);
        assert_eq!(m.data[15], 16.5);
    }

    #[test]
    fn creating_a_2x2_matrix() {
        let m = Matrix::new(vec![
            -3.0, 5.0,
            1.0, -2.0,
        ]);

        assert_eq!(m.size, 2);
        assert_eq!(m.data[0], -3.0);
        assert_eq!(m.data[1], 5.0);
        assert_eq!(m.data[2], 1.0);
        assert_eq!(m.data[3], -2.0);
    }

    #[test]
    fn creating_a_3x3_matrix() {
        let m = Matrix::new(vec![
            -3.0, 5.0, 0.0,
            1.0, -2.0, -7.0,
            0.0, 1.0, 1.0,
        ]);

        assert_eq!(m.size, 3);
        assert_eq!(m.data[0], -3.0);
        assert_eq!(m.data[1], 5.0);
        assert_eq!(m.data[2], 0.0);
        assert_eq!(m.data[3], 1.0);
        assert_eq!(m.data[4], -2.0);
        assert_eq!(m.data[5], -7.0);
        assert_eq!(m.data[6], 0.0);
        assert_eq!(m.data[7], 1.0);
        assert_eq!(m.data[8], 1.0);
    }

    #[test]
    fn matrix_equality_with_identical_matrices() {
        let a = Matrix::new(vec![
            1.0, 2.0, 3.0, 4.0,
            5.0, 6.0, 7.0, 8.0,
            9.0, 8.0, 7.0, 6.0,
            5.0, 4.0, 3.0, 2.0,
        ]);

        let b = Matrix::new(vec![
            1.0, 2.0, 3.0, 4.0,
            5.0, 6.0, 7.0, 8.0,
            9.0, 8.0, 7.0, 6.0,
            5.0, 4.0, 3.0, 2.0,
        ]);

        assert_eq!(a, b);
    }

    #[test]
    fn matrix_equality_with_different_matrices() {
        let a = Matrix::new(vec![
            1.0, 2.0, 3.0, 4.0,
            5.0, 6.0, 7.0, 8.0,
            9.0, 8.0, 7.0, 6.0,
            5.0, 4.0, 3.0, 2.0,
        ]);

        let b = Matrix::new(vec![
            2.0, 3.0, 4.0, 5.0,
            6.0, 7.0, 8.0, 9.0,
            8.0, 7.0, 6.0, 5.0,
            4.0, 3.0, 2.0, 1.0,
        ]);

        assert_ne!(a, b);
    }

    #[test]
    fn multiplying_two_matrices() {
        let a = Matrix::new(vec![
            1.0, 2.0, 3.0, 4.0,
            5.0, 6.0, 7.0, 8.0,
            9.0, 8.0, 7.0, 6.0,
            5.0, 4.0, 3.0, 2.0,
        ]);

        let b = Matrix::new(vec![
            -2.0, 1.0, 2.0, 3.0,
            3.0, 2.0, 1.0, -1.0,
            4.0, 3.0, 6.0, 5.0,
            1.0, 2.0, 7.0, 8.0,
        ]);

        let c = Matrix::new(vec![
            20.0, 22.0, 50.0, 48.0,
            44.0, 54.0, 114.0, 108.0,
            40.0, 58.0, 110.0, 102.0,
            16.0, 26.0, 46.0, 42.0,
        ]);

        assert_eq!(a * b, c);
    }

    #[test]
    fn a_matrix_multiplied_by_a_tuple() {
        let a = Matrix::new(vec![
            1.0, 2.0, 3.0, 4.0,
            2.0, 4.0, 4.0, 2.0,
            8.0, 6.0, 4.0, 1.0,
            0.0, 0.0, 0.0, 1.0,
        ]);

        let b = Tuple::point(1.0, 2.0, 3.0);

        assert_eq!(a * b, Tuple::point(18.0, 24.0, 33.0));
    }

    #[test]
    fn multiplying_a_matrix_by_the_identity_matrix() {
        let a = Matrix::new(vec![
            0.0, 1.0, 2.0, 4.0,
            1.0, 2.0, 4.0, 8.0,
            2.0, 4.0, 8.0, 16.0,
            4.0, 8.0, 16.0, 32.0,
        ]);

        assert_eq!(&a * &Matrix::identity(), a);
    }

    #[test]
    fn multiplying_the_identity_matrix_by_a_tuple() {
        let a = Tuple::point(1.0, 2.0, 3.0);

        assert_eq!(Matrix::identity() * a, a);
    }

    #[test]
    fn transposing_a_matrix() {
        let a = Matrix::new(vec![
            0.0, 9.0, 3.0, 0.0,
            9.0, 8.0, 0.0, 8.0,
            1.0, 8.0, 5.0, 3.0,
            0.0, 0.0, 5.0, 8.0,
        ]);

        let b = Matrix::new(vec![
            0.0, 9.0, 1.0, 0.0,
            9.0, 8.0, 8.0, 0.0,
            3.0, 0.0, 5.0, 5.0,
            0.0, 8.0, 3.0, 8.0,
        ]);

        assert_eq!(a.transpose(), b);
    }

    #[test]
    fn transposing_the_identity_matrix() {
        assert_eq!(Matrix::identity().transpose(), Matrix::identity());
    }

    #[test]
    fn calculating_the_determinant_of_a_2x2_matrix() {
        let a = Matrix::new(vec![
            1.0, 5.0,
            -3.0, 2.0,
        ]);

        assert_eq!(a.determinant(), 17.0);
    }

    #[test]
    fn a_submatrix_of_a_3x3_matrix_is_a_2x2_matrix() {
        let a = Matrix::new(vec![
            1.0, 5.0, 0.0,
            -3.0, 2.0, 7.0,
            0.0, 6.0, -3.0,
        ]);

        let b = Matrix::new(vec![
            -3.0, 2.0,
            0.0, 6.0,
        ]);

        assert_eq!(a.submatrix(0, 2), b);
    }

    #[test]
    fn a_submatrix_of_a_4x4_matrix_is_a_3x3_matrix() {
        let a = Matrix::new(vec![
            -6.0, 1.0, 1.0, 6.0,
            -8.0, 5.0, 8.0, 6.0,
            -1.0, 0.0, 8.0, 2.0,
            -7.0, 1.0, -1.0, 1.0,
        ]);

        let b = Matrix::new(vec![
            -6.0, 1.0, 6.0,
            -8.0, 8.0, 6.0,
            -7.0, -1.0, 1.0,
        ]);

        assert_eq!(a.submatrix(2, 1), b);
    }

    #[test]
    fn calculating_a_minor_of_a_3x3_matrix() {
        let a = Matrix::new(vec![
            3.0, 5.0, 0.0,
            2.0, -1.0, -7.0,
            6.0, -1.0, 5.0,
        ]);

        let b = a.submatrix(1, 0);
        assert_eq!(b.determinant(), 25.0);
        assert_eq!(a.minor(1, 0), 25.0);
    }

    #[test]
    fn calculating_a_cofactor_of_a_3x3_matrix() {
        let a = Matrix::new(vec![
            3.0, 5.0, 0.0,
            2.0, -1.0, -7.0,
            6.0, -1.0, 5.0,
        ]);

        assert_eq!(a.minor(0, 0), -12.0);
        assert_eq!(a.cofactor(0, 0), -12.0);
        assert_eq!(a.minor(1, 0), 25.0);
        assert_eq!(a.cofactor(1, 0), -25.0);
    }

    #[test]
    fn calculating_the_determinant_of_a_3x3_matrix() {
        let a = Matrix::new(vec![
            1.0, 2.0, 6.0,
            -5.0, 8.0, -4.0,
            2.0, 6.0, 4.0,
        ]);

        assert_eq!(a.cofactor(0, 0), 56.0);
        assert_eq!(a.cofactor(0, 1), 12.0);
        assert_eq!(a.cofactor(0, 2), -46.0);
        assert_eq!(a.determinant(), -196.0);
    }

    #[test]
    fn calculating_the_determinant_of_a_4x4_matrix() {
        let a = Matrix::new(vec![
            -2.0, -8.0, 3.0, 5.0,
            -3.0, 1.0, 7.0, 3.0,
            1.0, 2.0, -9.0, 6.0,
            -6.0, 7.0, 7.0, -9.0,
        ]);

        assert_eq!(a.cofactor(0, 0), 690.0);
        assert_eq!(a.cofactor(0, 1), 447.0);
        assert_eq!(a.cofactor(0, 2), 210.0);
        assert_eq!(a.cofactor(0, 3), 51.0);
        assert_eq!(a.determinant(), -4071.0);
    }

    #[test]
    fn testing_an_invertible_matrix_for_invertibility() {
        let a = Matrix::new(vec![
            6.0, 4.0, 4.0, 4.0,
            5.0, 5.0, 7.0, 6.0,
            4.0, -9.0, 3.0, -7.0,
            9.0, 1.0, 7.0, -6.0,
        ]);

        assert_eq!(a.determinant(), -2120.0);
        assert!(a.is_invertible());
    }

    #[test]
    fn testing_a_noninvertible_matrix_for_invertibility() {
        let a = Matrix::new(vec![
            -4.0, 2.0, -2.0, -3.0,
            9.0, 6.0, 2.0, 6.0,
            0.0, -5.0, 1.0, -5.0,
            0.0, 0.0, 0.0, 0.0,
        ]);

        assert_eq!(a.determinant(), 0.0);
        assert!(!a.is_invertible());
    }

    #[test]
    fn calculating_the_inverse_of_a_matrix() {
        let a = Matrix::new(vec![
            -5.0, 2.0, 6.0, -8.0,
            1.0, -5.0, 1.0, 8.0,
            7.0, 7.0, -6.0, -7.0,
            1.0, -3.0, 7.0, 4.0,
        ]);

        let b = a.inverse();

        assert_eq!(a.determinant(), 532.0);
        assert_eq!(a.cofactor(2, 3), -160.0);
        assert_eq!(b.data[3*4+2], -160.0 / 532.0);
        assert_eq!(a.cofactor(3, 2), 105.0);
        assert_eq!(b.data[2*4+3], 105.0 / 532.0);

        let c = Matrix::new(vec![
            0.21805, 0.45113, 0.24060, -0.04511,
            -0.80827, -1.45677, -0.44361, 0.52068,
            -0.07895, -0.22368, -0.05263, 0.19737,
            -0.52256, -0.81391, -0.30075, 0.30639,
        ]);

        for (i, &val) in c.data.iter().enumerate() {
            assert!((b.data[i] - val).abs() < 1e-5);
        }
    }

    #[test]
    fn calculating_the_inverse_of_another_matrix() {
        let a = Matrix::new(vec![
            8.0, -5.0, 9.0, 2.0,
            7.0, 5.0, 6.0, 1.0,
            -6.0, 0.0, 9.0, 6.0,
            -3.0, 0.0, -9.0, -4.0,
        ]);

        let b = a.inverse();

        let c = Matrix::new(vec![
            -0.15385, -0.15385, -0.28205, -0.53846,
            -0.07692, 0.12308, 0.02564, 0.03077,
            0.35897, 0.35897, 0.43590, 0.92308,
            -0.69231, -0.69231, -0.76923, -1.92308,
        ]);

        for (i, &val) in c.data.iter().enumerate() {
            assert!((b.data[i] - val).abs() < 1e-5);
        }
    }

    #[test]
    fn calculating_the_inverse_of_a_third_matrix() {
        let a = Matrix::new(vec![
            9.0, 3.0, 0.0, 9.0,
            -5.0, -2.0, -6.0, -3.0,
            -4.0, 9.0, 6.0, 4.0,
            -7.0, 6.0, 6.0, 2.0,
        ]);

        let b = a.inverse();

        let c = Matrix::new(vec![
            -0.04074, -0.07778, 0.14444, -0.22222,
            -0.07778, 0.03333, 0.36667, -0.33333,
            -0.02901, -0.14630, -0.10926, 0.12963,
            0.17778, 0.06667, -0.26667, 0.33333,
        ]);

        for (i, &val) in c.data.iter().enumerate() {
            assert!((b.data[i] - val).abs() < 1e-5);
        }
    }

    #[test]
    fn multiplying_a_product_by_its_inverse() {
        let a = Matrix::new(vec![
            3.0, -9.0, 7.0, 3.0,
            3.0, -8.0, 2.0, -9.0,
            -4.0, 4.0, 4.0, 1.0,
            -6.0, 5.0, -1.0, 1.0,
        ]);

        let b = Matrix::new(vec![
            8.0, 2.0, 2.0, 2.0,
            3.0, -1.0, 7.0, 0.0,
            7.0, 0.0, 5.0, 4.0,
            6.0, -2.0, 0.0, 5.0,
        ]);

        let c = &a * &b;
        let epsilon = 1e-5;
        for (a_val, c_val) in a.data.iter().zip((&c * &b.inverse()).data.iter()) {
            assert!((a_val - c_val).abs() < epsilon);
        }
    }

}
