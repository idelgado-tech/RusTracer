use crate::matrix::Matrix;
use crate::tuple::Tuple;

fn translation(x: f64, y: f64, z: f64) -> Matrix {
    let mut m = Matrix::new_identity_matrix(4);
    m.set_element(0, 3, x);
    m.set_element(1, 3, y);
    m.set_element(2, 3, z);
    m
}

fn scaling(x: f64, y: f64, z: f64) -> Matrix {
    let mut m = Matrix::new_identity_matrix(4);
    m.set_element(0, 0, x);
    m.set_element(1, 1, y);
    m.set_element(2, 2, z);
    m
}

#[cfg(test)]
mod transformation_tests {
    use super::*;

    #[test]
    ///Multiplying by a translation matrix
    fn translation_multiplication() {
        let transform = translation(5.0, -3.0, 2.0);
        let point = Tuple::new_point(-3.0, 4.0, 5.0);
        assert_eq!(transform * point, Tuple::new_point(2.0, 1.0, 7.0))
    }

    #[test]
    //Multiplying by the inverse of a translation matrix
    fn translation_inverse() {
        let transform = translation(5.0, -3.0, 2.0);
        let inv = transform.inverse().unwrap();
        let point = Tuple::new_point(-3.0, 4.0, 5.0);
        assert_eq!(inv * point, Tuple::new_point(-8.0, 7.0, 3.0))
    }

    #[test]
    //Multiplying by the inverse of a translation matrix
    fn translation_verctor() {
        let transform = translation(5.0, -3.0, 2.0);
        let vecteur = Tuple::new_vector(-3.0, 4.0, 5.0);
        assert_eq!(transform * vecteur.clone(), vecteur)
    }


    
}
