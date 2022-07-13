use crate::error;
use crate::error::ErrorKind;
use crate::tuple::*;
use crate::utils::*;

use std::ops::Mul;

///Represent a square matrix
#[derive(Debug, Clone)]
pub struct Matrix {
    size: usize,
    matrix: Vec<f64>,
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        if self.size != other.size {
            return false;
        } else {
            for m_tuple in self.matrix.iter().zip(other.matrix.iter()) {
                let (am, bm) = m_tuple;
                if !compare_float(*am, *bm) {
                    return false;
                }
            }
        }
        true
    }
}

impl Mul for Matrix {
    type Output = Matrix;

    fn mul(self, other: Matrix) -> Matrix {
        let mut matrix = Matrix::new_matrix(self.size);

        for row in 0..self.size {
            for col in 0..self.size {
                let mut val = 0.0;
                for i in 0..self.size {
                    val += self.element(row, i) * other.element(i, col);
                }
                matrix.set_element(row, col, val);
            }
        }
        matrix
    }
}

impl Mul<Tuple> for Matrix {
    type Output = Tuple;

    fn mul(self, other: Tuple) -> Tuple {
        let other_as_vec = vec![other.x, other.y, other.z, W::to_int(other.w) as f64];
        let mut tuple_tmp = vec![0.0, 0.0, 0.0, 0.0];

        for row in 0..self.size {
            let mut val = 0.0;
            for col in 0..self.size {
                val += self.element(row, col) * other_as_vec[col];
            }

            tuple_tmp[row] = val;
        }

        Tuple::new_tuple(
            tuple_tmp[0],
            tuple_tmp[1],
            tuple_tmp[2],
            tuple_tmp[3] as i64,
        )
    }
}

impl Matrix {
    pub fn new_matrix(size: usize) -> Matrix {
        if !(1..=4).contains(&size) {
            panic!("Matrix size can only be 2; 3 or 4")
        }
        Matrix {
            size,
            matrix: vec![0.0; size * size],
        }
    }

    pub fn new_matrix_with_data(size: usize, data: Vec<f64>) -> Matrix {
        Matrix { size, matrix: data }
    }

    pub fn new_identity_matrix(size: usize) -> Matrix {
        if !(1..=4).contains(&size) {
            panic!("Matrix size can only be 2; 3 or 4")
        }
        let mut matrix = Matrix {
            size,
            matrix: vec![0.0; size * size],
        };

        for row in 0..size {
            matrix.set_element(row, row, 1.0);
        }

        matrix
    }

    pub fn element(&self, row: usize, column: usize) -> f64 {
        self.matrix[(row * self.size) + column]
    }

    pub fn set_element(&mut self, row: usize, column: usize, value: f64) {
        self.matrix[(row * self.size) + column] = value;
    }

    pub fn transpose(&self) -> Matrix {
        let mut matrix = Matrix::new_matrix(self.size);

        for row in 0..self.size {
            for col in 0..self.size {
                matrix.set_element(row, col, self.element(col, row));
            }
        }
        matrix
    }

    pub fn determinant(&self) -> f64 {
        let mut determinant = 0.0;
        if self.size == 2 {
            determinant =
                self.element(0, 0) * self.element(1, 1) - self.element(1, 0) * self.element(0, 1);
        } else {
            for col in 0..self.size {
                determinant += self.element(0, col) * self.cofactor(0, col);
            }
        }
        determinant
    }

    pub fn sub_matix(&self, row: usize, col: usize) -> Matrix {
        let mut data_vec = vec![];

        for irow in 0..self.size {
            for icol in 0..self.size {
                if row != irow && col != icol {
                    data_vec.push(self.element(irow, icol));
                }
            }
        }
        Matrix::new_matrix_with_data(self.size - 1, data_vec)
    }

    pub fn minor(&self, row: usize, col: usize) -> f64 {
        let sub_matrix = self.sub_matix(row, col);
        sub_matrix.determinant()
    }

    pub fn cofactor(&self, row: usize, col: usize) -> f64 {
        let minor = self.minor(row, col);
        let factor = if (row + col) % 2 == 1 { -1.0 } else { 1.0 };
        minor * factor
    }

    pub fn is_invertible(&self) -> bool {
        self.determinant() != 0.0
    }

    pub fn inverse(&self) -> Result<Matrix, error::TracerError> {
        if !self.is_invertible() {
            Err(error::TracerError::new_simple(ErrorKind::NotInversible))
        } else {
            let mut m2 = Matrix::new_matrix(self.size);

            for row in 0..self.size {
                for col in 0..self.size {
                    let c = self.cofactor(row, col);
                    m2.set_element(col, row, c / self.determinant());
                }
            }
            Ok(m2)
        }
    }
}

#[cfg(test)]
mod matrix_tests {
    use crate::utils;

    use super::*;

    #[test]
    ///Constructing and inspecting a 4x4 matrix
    fn matrix_creation() {
        let data_vector = vec![
            1.0, 2.0, 3.0, 4.0, 5.5, 6.5, 7.5, 8.5, 9.0, 10.0, 11.0, 12.0, 13.5, 14.5, 15.5, 16.5,
        ];
        let m = Matrix::new_matrix_with_data(4, data_vector);

        assert_eq!(m.element(0, 0), 1.0);
        assert_eq!(m.element(0, 3), 4.0);
        assert_eq!(m.element(1, 0), 5.5);
        assert_eq!(m.element(1, 2), 7.5);
        assert_eq!(m.element(2, 2), 11.0);
        assert_eq!(m.element(3, 0), 13.5);
        assert_eq!(m.element(3, 2), 15.5);
    }

    #[test]
    ///A 2x2 matrix ought to be representable
    fn matrix_2x2_creation() {
        let data_vector = vec![-3.0, 5.0, 1.0, -2.0];
        let m = Matrix::new_matrix_with_data(2, data_vector);

        assert_eq!(m.element(0, 0), -3.0);
        assert_eq!(m.element(0, 1), 5.0);
        assert_eq!(m.element(1, 0), 1.0);
        assert_eq!(m.element(1, 1), -2.0);
    }

    #[test]
    ///A 3x matrix ought to be representable
    fn matrix_3x3_creation() {
        let data_vector = vec![-3.0, 5.0, 0.0, 1.0, -2.0, -7.0, 0.0, 1.0, 1.0];
        let m = Matrix::new_matrix_with_data(3, data_vector);

        assert_eq!(m.element(0, 0), -3.0);
        assert_eq!(m.element(1, 1), -2.0);
        assert_eq!(m.element(2, 2), 1.0);
    }

    #[test]
    ///Matrix equality with identical matrices
    fn matrix_equality() {
        let data_vector_a = vec![
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0,
        ];
        let ma = Matrix::new_matrix_with_data(4, data_vector_a);
        let data_vector_b = vec![
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0,
        ];
        let mb = Matrix::new_matrix_with_data(4, data_vector_b);

        assert_eq!(ma, mb);
    }

    #[test]
    ///Matrix equality with different matrices
    fn matrix_inequality() {
        let data_vector_a = vec![
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0,
        ];
        let ma = Matrix::new_matrix_with_data(4, data_vector_a);
        let data_vector_b = vec![
            1.0, 6.0, 7.0, 4.0, 2.0, 3.0, 4.0, 5.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 3.0, 2.0,
        ];
        let mb = Matrix::new_matrix_with_data(4, data_vector_b);

        assert_ne!(ma, mb);
    }

    #[test]
    /// Multiplying two matrices
    fn matrix_multiplication() {
        let data_vector_a = vec![
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0,
        ];
        let ma = Matrix::new_matrix_with_data(4, data_vector_a);
        let data_vector_b = vec![
            -2.0, 1.0, 2.0, 3.0, 3.0, 2.0, 1.0, -1.0, 4.0, 3.0, 6.0, 5.0, 1.0, 2.0, 7.0, 8.0,
        ];
        let mb = Matrix::new_matrix_with_data(4, data_vector_b);

        let data_vector_result = vec![
            20.0, 22.0, 50.0, 48.0, 44.0, 54.0, 114.0, 108.0, 40.0, 58.0, 110.0, 102.0, 16.0, 26.0,
            46.0, 42.0,
        ];
        let m_result = Matrix::new_matrix_with_data(4, data_vector_result);

        assert_eq!(ma * mb, m_result);
    }

    #[test]
    /// Multiplying a matrix by the identity matrix
    fn matrix_multiplication_by_identity() {
        let data_vector_a = vec![
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 8.0, 7.0, 6.0, 5.0, 4.0, 3.0, 2.0,
        ];
        let ma = Matrix::new_matrix_with_data(4, data_vector_a);
        let data_vector_identity = vec![
            1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        ];
        let mb = Matrix::new_matrix_with_data(4, data_vector_identity);

        let result = ma.clone();
        assert_eq!(ma * mb, result);
    }

    #[test]
    /// Multiplying a matrix by a tuple
    fn matrix_multiplication_by_tuple() {
        let data_vector_a = vec![
            1.0, 2.0, 3.0, 4.0, 2.0, 4.0, 4.0, 2.0, 8.0, 6.0, 4.0, 1.0, 0.0, 0.0, 0.0, 1.0,
        ];
        let ma = Matrix::new_matrix_with_data(4, data_vector_a);

        let tuple = Tuple::new_point(1.0, 2.0, 3.0);
        let tuple_res = Tuple::new_point(18.0, 24.0, 33.0);

        assert_eq!(ma * tuple, tuple_res);
    }

    #[test]
    /// Multiplying a matrix by a tuple
    fn matrix_determinant_2x2_matrix() {
        let data_vector_a = vec![1.0, 5.0, -3.0, 2.0];
        let ma = Matrix::new_matrix_with_data(2, data_vector_a);

        assert_eq!(ma.determinant(), 17.0);
    }

    #[test]
    /// Multiplying a matrix by a tuple
    fn matrix_transpose() {
        let data_vector_a = vec![
            0.0, 9.0, 3.0, 0.0, 9.0, 8.0, 0.0, 8.0, 1.0, 8.0, 5.0, 3.0, 0.0, 0.0, 5.0, 8.0,
        ];
        let ma = Matrix::new_matrix_with_data(4, data_vector_a);

        let data_vector_b = vec![
            0.0, 9.0, 1.0, 0.0, 9.0, 8.0, 8.0, 0.0, 3.0, 0.0, 5.0, 5.0, 0.0, 8.0, 3.0, 8.0,
        ];
        let mb = Matrix::new_matrix_with_data(4, data_vector_b);

        assert_eq!(ma.transpose(), mb);

        let data_vector_identity = vec![
            1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 1.0,
        ];
        let m_ident = Matrix::new_matrix_with_data(4, data_vector_identity);

        assert_eq!(m_ident.transpose(), m_ident);
    }

    #[test]
    /// Subtracting matrixes
    fn sub_matrix() {
        let data_vector_a = vec![1.0, 5.0, 0.0, -3.0, 2.0, 7.0, 0.0, 6.0, -3.0];
        let ma = Matrix::new_matrix_with_data(3, data_vector_a);

        let data_vector_b = vec![-3.0, 2.0, 0.0, 6.0];
        let mb = Matrix::new_matrix_with_data(2, data_vector_b);

        assert_eq!(ma.sub_matix(0, 2), mb);

        let data_vector_c = vec![
            0.0, 9.0, 3.0, 0.0, 9.0, 8.0, 0.0, 8.0, 1.0, 8.0, 5.0, 3.0, 0.0, 0.0, 5.0, 8.0,
        ];
        let mc = Matrix::new_matrix_with_data(4, data_vector_c);

        let data_vector_d = vec![0.0, 3.0, 0.0, 9.0, 0.0, 8.0, 0.0, 5.0, 8.0];
        let md = Matrix::new_matrix_with_data(3, data_vector_d);
        assert_eq!(mc.sub_matix(2, 1), md);
    }

    #[test]
    ///Calculating a minor of a 3x3 matrix
    fn minor() {
        let data_vector_a = vec![1.0, 5.0, 0.0, -3.0, 2.0, 7.0, 0.0, 6.0, -3.0];
        let ma = Matrix::new_matrix_with_data(3, data_vector_a);
        let mb = ma.sub_matix(1, 0);

        assert_eq!(ma.minor(1, 0), mb.determinant());
    }

    #[test]
    ///Calculating a minor of a 3x3 matrix
    fn cofactor() {
        let data_vector_a = vec![3.0, 5.0, 0.0, 2.0, -1.0, -7.0, 6.0, -1.0, 5.0];
        let ma = Matrix::new_matrix_with_data(3, data_vector_a);

        assert_eq!(ma.minor(0, 0), -12.0);
        assert_eq!(ma.cofactor(0, 0), -12.0);

        assert_eq!(ma.minor(1, 0), 25.0);
        assert_eq!(ma.cofactor(1, 0), -25.0);
    }

    #[test]
    ///Calculating the determinant of a 3x3 matrix
    fn determinant() {
        let data_vector_a = vec![1.0, 2.0, 6.0, -5.0, 8.0, -4.0, 2.0, 6.0, 4.0];
        let ma = Matrix::new_matrix_with_data(3, data_vector_a);

        assert_eq!(ma.cofactor(0, 0), 56.0);
        assert_eq!(ma.cofactor(0, 1), 12.0);
        assert_eq!(ma.cofactor(0, 2), -46.0);
        assert_eq!(ma.determinant(), -196.0);

        let data_vector_b = vec![
            -2.0, -8.0, 3.0, 5.0, -3.0, 1.0, 7.0, 3.0, 1.0, 2.0, -9.0, 6.0, -6.0, 7.0, 7.0, -9.0,
        ];
        let mb = Matrix::new_matrix_with_data(4, data_vector_b);

        assert_eq!(mb.cofactor(0, 0), 690.0);
        assert_eq!(mb.cofactor(0, 1), 447.0);
        assert_eq!(mb.cofactor(0, 2), 210.0);
        assert_eq!(mb.cofactor(0, 3), 51.0);
        assert_eq!(mb.determinant(), -4071.0);
    }

    #[test]
    ///Testing matrix for invertibility
    fn is_invertible() {
        let data_vector_a = vec![
            -2.0, -8.0, 3.0, 5.0, -3.0, 1.0, 7.0, 3.0, 1.0, 2.0, -9.0, 6.0, -6.0, 7.0, 7.0, -9.0,
        ];
        let ma = Matrix::new_matrix_with_data(4, data_vector_a);
        assert_eq!(ma.determinant(), -4071.0);
        assert!(ma.is_invertible());

        let data_vector_b = vec![
            -4.0, 2.0, -2.0, -3.0, 9.0, 6.0, 2.0, 6.0, 0.0, -5.0, 1.0, -5.0, 0.0, 0.0, 0.0, 0.0,
        ];
        let mb = Matrix::new_matrix_with_data(4, data_vector_b);
        assert_eq!(mb.determinant(), 0.0);
        assert!(!mb.is_invertible());
    }

    #[test]
    ///Calculating the inverse of a matrix
    fn inversion() {
        let data_vector_a = vec![
            -5.0, 2.0, 6.0, -8.0, 1.0, -5.0, 1.0, 8.0, 7.0, 7.0, -6.0, -7.0, 1.0, -3.0, 7.0, 4.0,
        ];
        let ma = Matrix::new_matrix_with_data(4, data_vector_a);
        let mb = ma.inverse().unwrap();

        assert_eq!(ma.determinant(), 532.0);
        assert!(ma.is_invertible());

        let data_vector_b_test = vec![
            0.21804511278195488,
            0.45112781954887216,
            0.24060150375939848,
            -0.045112781954887216,
            -0.8082706766917294,
            -1.4567669172932332,
            -0.44360902255639095,
            0.5206766917293233,
            -0.07894736842105263,
            -0.2236842105263158,
            -0.05263157894736842,
            0.19736842105263158,
            -0.5225563909774437,
            -0.8139097744360902,
            -0.3007518796992481,
            0.30639097744360905,
        ];
        let mb_test = Matrix::new_matrix_with_data(4, data_vector_b_test);
        assert_eq!(mb, mb_test);
    }
}
