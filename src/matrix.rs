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
        return matrix;
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

        return Tuple::new_tuple(
            tuple_tmp[0],
            tuple_tmp[1],
            tuple_tmp[2],
            tuple_tmp[3] as i64,
        );
    }
}

impl Matrix {
    pub fn new_matrix(size: usize) -> Matrix {
        if size < 1 || size > 4 {
            panic!("Matrix size can only be 2; 3 or 4")
        }
        Matrix {
            size: size,
            matrix: vec![0.0; size * size],
        }
    }

    pub fn new_matrix_with_data(size: usize, data: Vec<f64>) -> Matrix {
        Matrix {
            size: size,
            matrix: data.clone(),
        }
    }

    pub fn element(&self, row: usize, column: usize) -> f64 {
        self.matrix[(row * self.size) + column]
    }

    pub fn set_element(&mut self, row: usize, column: usize, value: f64) -> () {
        self.matrix[(row * self.size) + column] = value;
    }
}

#[cfg(test)]
mod matrix_tests {
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

    //     ​ 	​Scenario​: A matrix multiplied by a tuple
    // ​ 	  ​Given​ the following matrix A:
    // ​ 	      | 1 | 2 | 3 | 4 |
    // ​ 	      | 2 | 4 | 4 | 2 |
    // ​ 	      | 8 | 6 | 4 | 1 |
    // ​ 	      | 0 | 0 | 0 | 1 |
    // ​ 	    ​And​ b ← tuple(1, 2, 3, 1)
    // ​ 	  ​Then​ A * b = tuple(18, 24, 33, 1)
}
