use crate::{matrix::Matrix, tuple::Tuple};

pub fn create_translation(x: f64, y: f64, z: f64) -> Matrix {
    let mut m = Matrix::new_identity_matrix(4);
    m.set_element(0, 3, x);
    m.set_element(1, 3, y);
    m.set_element(2, 3, z);
    m
}

pub fn create_scaling(x: f64, y: f64, z: f64) -> Matrix {
    let mut m = Matrix::new_identity_matrix(4);
    m.set_element(0, 0, x);
    m.set_element(1, 1, y);
    m.set_element(2, 2, z);
    m
}

pub fn create_rotation_x(radians: f64) -> Matrix {
    let mut m = Matrix::new_identity_matrix(4);
    m.set_element(1, 1, radians.cos());
    m.set_element(1, 2, -radians.sin());
    m.set_element(2, 1, radians.sin());
    m.set_element(2, 2, radians.cos());
    m
}

pub fn create_rotation_y(radians: f64) -> Matrix {
    let mut m = Matrix::new_identity_matrix(4);
    m.set_element(0, 0, radians.cos());
    m.set_element(0, 2, radians.sin());
    m.set_element(2, 0, -radians.sin());
    m.set_element(2, 2, radians.cos());
    m
}

pub fn create_rotation_z(radians: f64) -> Matrix {
    let mut m = Matrix::new_identity_matrix(4);
    m.set_element(0, 0, radians.cos());
    m.set_element(0, 1, -radians.sin());
    m.set_element(1, 0, radians.sin());
    m.set_element(1, 1, radians.cos());
    m
}

pub fn create_shearing(x_y: f64, x_z: f64, y_x: f64, y_z: f64, z_x: f64, z_y: f64) -> Matrix {
    let mut m = Matrix::new_identity_matrix(4);
    m.set_element(0, 1, x_y);
    m.set_element(0, 2, x_z);
    m.set_element(1, 0, y_x);
    m.set_element(1, 2, y_z);
    m.set_element(2, 0, z_x);
    m.set_element(2, 1, z_y);
    m
}

impl Matrix {
    pub fn translation(self, x: f64, y: f64, z: f64) -> Matrix {
        let translation = create_translation(x, y, z);
        translation * self
    }

    pub fn scaling(self, x: f64, y: f64, z: f64) -> Matrix {
        let scaling = create_scaling(x, y, z);
        scaling * self
    }

    pub fn rotation_x(self, radians: f64) -> Matrix {
        let rotation_x = create_rotation_x(radians);
        rotation_x * self
    }

    pub fn rotation_y(self, radians: f64) -> Matrix {
        let rotation_y = create_rotation_y(radians);
        rotation_y * self
    }

    pub fn rotation_z(self, radians: f64) -> Matrix {
        let rotation_z = create_rotation_z(radians);
        rotation_z * self
    }

    pub fn shearing(self, x_y: f64, x_z: f64, y_x: f64, y_z: f64, z_x: f64, z_y: f64) -> Matrix {
        let mut m = Matrix::new_identity_matrix(4);
        m.set_element(0, 1, x_y);
        m.set_element(0, 2, x_z);
        m.set_element(1, 0, y_x);
        m.set_element(1, 2, y_z);
        m.set_element(2, 0, z_x);
        m.set_element(2, 1, z_y);
        m
    }
}

pub fn view_transform(from: &Tuple, to: &Tuple, up: &Tuple) -> Matrix {
    let forward = (to.clone() - from.clone()).normalize();
    let upn = up.normalize();
    let left = Tuple::cross_product(&forward, &upn);
    let true_up = Tuple::cross_product(&left, &forward);
    let orientation = Matrix::new_matrix_with_data(
        4,
        vec![
            left.x, left.y, left.z, 0.0, true_up.x, true_up.y, true_up.z, 0.0, -forward.x,
            -forward.y, -forward.z, 0.0, 0.0, 0.0, 0.0, 1.0,
        ],
    );
    orientation * create_translation(-from.x, -from.y, -from.z)
}

#[cfg(test)]
mod transformation_tests {
    use crate::tuple::Tuple;
    use std::f64::consts::PI;

    use super::*;

    #[test]
    ///Multiplying by a translation matrix
    fn translation_multiplication() {
        let transform = create_translation(5.0, -3.0, 2.0);
        let point = Tuple::new_point(-3.0, 4.0, 5.0);
        assert_eq!(transform * point, Tuple::new_point(2.0, 1.0, 7.0))
    }

    #[test]
    ///Multiplying by the inverse of a translation matrix
    fn translation_inverse() {
        let transform = create_translation(5.0, -3.0, 2.0);
        let inv = transform.inverse().unwrap();
        let point = Tuple::new_point(-3.0, 4.0, 5.0);
        assert_eq!(inv * point, Tuple::new_point(-8.0, 7.0, 3.0))
    }

    #[test]
    ///Multiplying by the inverse of a translation matrix
    fn translation_vector() {
        let transform = create_translation(5.0, -3.0, 2.0);
        let vecteur = Tuple::new_vector(-3.0, 4.0, 5.0);
        assert_eq!(transform * vecteur.clone(), vecteur)
    }

    #[test]
    ///A scaling matrix applied to a point
    fn scaling_point() {
        let scaling = create_scaling(2.0, 3.0, 4.0);
        let vecteur = Tuple::new_point(-4.0, 6.0, 8.0);
        assert_eq!(scaling * vecteur, Tuple::new_point(-8.0, 18.0, 32.0))
    }

    #[test]
    ///A scaling matrix applied to a vector
    fn scaling_vector() {
        let scaling = create_scaling(2.0, 3.0, 4.0);
        let vecteur = Tuple::new_vector(-4.0, 6.0, 8.0);
        assert_eq!(scaling * vecteur, Tuple::new_vector(-8.0, 18.0, 32.0))
    }

    #[test]
    ///Multiplying by the inverse of a scaling matrix
    fn inverse_scaling_vector() {
        let scaling = create_scaling(2.0, 3.0, 4.0);
        let inv = scaling.inverse().unwrap();
        let vecteur = Tuple::new_vector(-4.0, 6.0, 8.0);
        assert_eq!(inv * vecteur, Tuple::new_vector(-2.0, 2.0, 2.0))
    }

    #[test]
    ///Reflection is scaling by a negative value
    fn reflection_test() {
        let scaling = create_scaling(-1.0, 1.0, 1.0);
        let point = Tuple::new_point(2.0, 3.0, 4.0);
        assert_eq!(scaling * point, Tuple::new_point(-2.0, 3.0, 4.0))
    }

    #[test]
    ///Rotating a point around the x axis
    fn rotation_x_test() {
        let point = Tuple::new_point(0.0, 1.0, 0.0);
        let half_quarter = create_rotation_x(PI / 4.0);
        let full_quarter = create_rotation_x(PI / 2.0);
        assert_eq!(
            half_quarter * point.clone(),
            Tuple::new_point(0.0, 2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0)
        );
        assert_eq!(full_quarter * point, Tuple::new_point(0.0, 0.0, 1.0));
    }

    #[test]
    ///The inverse of an x-rotation rotates in the opposite direction
    fn rotation_x_inv_test() {
        let point = Tuple::new_point(0.0, 1.0, 0.0);
        let half_quarter = create_rotation_x(PI / 4.0);
        let inv = half_quarter.inverse().unwrap();

        assert_eq!(
            inv * point,
            Tuple::new_point(0.0, 2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0)
        );
    }

    #[test]
    ///Rotating a point around the y axis
    fn rotation_y_test() {
        let point = Tuple::new_point(0.0, 0.0, 1.0);
        let half_quarter = create_rotation_y(PI / 4.0);
        let full_quarter = create_rotation_y(PI / 2.0);
        assert_eq!(
            half_quarter * point.clone(),
            Tuple::new_point(2.0_f64.sqrt() / 2.0, 0.0, 2.0_f64.sqrt() / 2.0)
        );
        assert_eq!(full_quarter * point, Tuple::new_point(1.0, 0.0, 0.0));
    }

    #[test]
    /// Rotating a point around the z axis
    fn rotation_z_test() {
        let point = Tuple::new_point(0.0, 1.0, 0.0);
        let half_quarter = create_rotation_z(PI / 4.0);
        let full_quarter = create_rotation_z(PI / 2.0);
        assert_eq!(
            half_quarter * point.clone(),
            Tuple::new_point(-2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0, 0.0)
        );
        assert_eq!(full_quarter * point, Tuple::new_point(-1.0, 0.0, 0.0));
    }

    #[test]
    ///A shearing transformation moves x in proportion to y
    fn shearing_x_y() {
        let shearing = create_shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0);
        let point = Tuple::new_point(2.0, 3.0, 4.0);
        assert_eq!(shearing * point, Tuple::new_point(5.0, 3.0, 4.0))
    }

    #[test]
    ///A shearing transformation moves x in proportion to z
    fn shearing_x_z() {
        let shearing = create_shearing(0.0, 1.0, 0.0, 0.0, 0.0, 0.0);
        let point = Tuple::new_point(2.0, 3.0, 4.0);
        assert_eq!(shearing * point, Tuple::new_point(6.0, 3.0, 4.0))
    }

    #[test]
    ///A shearing transformation moves y in proportion to x
    fn shearing_y_x() {
        let shearing = create_shearing(0.0, 0.0, 1.0, 0.0, 0.0, 0.0);
        let point = Tuple::new_point(2.0, 3.0, 4.0);
        assert_eq!(shearing * point, Tuple::new_point(2.0, 5.0, 4.0))
    }

    #[test]
    ///A shearing transformation moves y in proportion to z
    fn shearing_y_z() {
        let shearing = create_shearing(0.0, 0.0, 0.0, 1.0, 0.0, 0.0);
        let point = Tuple::new_point(2.0, 3.0, 4.0);
        assert_eq!(shearing * point, Tuple::new_point(2.0, 7.0, 4.0))
    }

    #[test]
    ///A shearing transformation moves z in proportion to x
    fn shearing_z_x() {
        let shearing = create_shearing(0.0, 0.0, 0.0, 0.0, 1.0, 0.0);
        let point = Tuple::new_point(2.0, 3.0, 4.0);
        assert_eq!(shearing * point, Tuple::new_point(2.0, 3.0, 6.0))
    }

    #[test]
    ///A shearing transformation moves z in proportion to y
    fn shearing_z_y() {
        let shearing = create_shearing(0.0, 0.0, 0.0, 0.0, 0.0, 1.0);
        let point = Tuple::new_point(2.0, 3.0, 4.0);
        assert_eq!(shearing * point, Tuple::new_point(2.0, 3.0, 7.0))
    }

    #[test]
    ///A shearing transformation moves z in proportion to y
    fn individual_tranformations() {
        let point = Tuple::new_point(1.0, 0.0, 1.0);
        let a = create_rotation_x(PI / 2.0);
        let b = create_scaling(5.0, 5.0, 5.0);
        let c = create_translation(10.0, 5.0, 7.0);
        // apply rotation first
        let point2 = a * point;
        assert_eq!(point2, Tuple::new_point(1.0, -1.0, 0.0));
        // then apply scaling​
        let point3 = b * point2;
        assert_eq!(point3, Tuple::new_point(5.0, -5.0, 0.0));
        // then apply translation​
        let point4 = c * point3;
        assert_eq!(point4, Tuple::new_point(15.0, 0.0, 7.0));
    }

    #[test]
    ///Chained transformations must be applied in reverse order
    fn chained_tranformations() {
        let point = Tuple::new_point(1.0, 0.0, 1.0);
        let a = create_rotation_x(PI / 2.0);
        let b = create_scaling(5.0, 5.0, 5.0);
        let c = create_translation(10.0, 5.0, 7.0);

        let t = c * b * a;
        let point2 = t * point;

        assert_eq!(point2, Tuple::new_point(15.0, 0.0, 7.0));
    }

    #[test]
    ///Chained transformations must be applied in reverse order
    fn chained_fluent_tranformations() {
        let point = Tuple::new_point(1.0, 0.0, 1.0);
        let t = create_rotation_x(PI / 2.0)
            .scaling(5.0, 5.0, 5.0)
            .translation(10.0, 5.0, 7.0);
        let point2 = t * point;

        assert_eq!(point2, Tuple::new_point(15.0, 0.0, 7.0));
    }

    #[test]
    ///The transformation matrix for the default orientation    
    fn view_tranformations() {
        let from = Tuple::new_point(0.0, 0.0, 0.0);
        let to = Tuple::new_point(0.0, 0.0, -1.0);
        let up = Tuple::new_vector(0.0, 1.0, 0.0);

        let t = view_transform(&from, &to, &up);

        assert_eq!(Matrix::new_identity_matrix(4), t);
    }

    #[test]
    ///A view transformation matrix looking in positive z direction
    fn view_tranformations_z_positive() {
        let from = Tuple::new_point(0.0, 0.0, 0.0);
        let to = Tuple::new_point(0.0, 0.0, 1.0);
        let up = Tuple::new_vector(0.0, 1.0, 0.0);

        let t = view_transform(&from, &to, &up);

        assert_eq!(create_scaling(-1.0, 1.0, -1.0), t);
    }

    #[test]
    ///A view transformation matrix looking in positive z direction
    fn view_tranformations_move_world() {
        let from = Tuple::new_point(0.0, 0.0, 8.0);
        let to = Tuple::new_point(0.0, 0.0, 0.0);
        let up = Tuple::new_vector(0.0, 1.0, 0.0);

        let t = view_transform(&from, &to, &up);

        assert_eq!(create_translation(0.0, 0.0, -8.0), t);
    }

    #[test]
    ///An arbitrary view transformation
    fn view_tranformations_arbitrary() {
        let from = Tuple::new_point(1.0, 3.0, 2.0);
        let to = Tuple::new_point(4.0, -2.0, 8.0);
        let up = Tuple::new_vector(1.0, 1.0, 0.0);

        let t = view_transform(&from, &to, &up);

        assert_eq!(
            Matrix::new_matrix_with_data(
                4,
                vec![
                    -0.5070925528371099,
                    0.5070925528371099,
                    0.6761234037828132,
                    -2.366431913239846,
                    0.7677159338596801,
                    0.6060915267313263,
                    0.12121830534626524,
                    -2.8284271247461894,
                    -0.35856858280031806,
                    0.5976143046671968,
                    -0.7171371656006361,
                    0.0,
                    0.0,
                    0.0,
                    0.0,
                    1.0
                ]
            ),
            t
        );
    }
}