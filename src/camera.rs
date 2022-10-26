use crate::{matrix::Matrix, tuple::Tuple};

///virtual camera
#[derive(Debug, Clone)]
pub struct Camera {
    hsize: usize,
    vsize: usize,
    field_of_view: f64,
    transformation: Matrix,
}

impl Camera {
    fn new(hsize: usize, vsize: usize, field_of_view: f64) -> Camera {
        Camera {
            hsize,
            vsize,
            field_of_view,
            transformation: Matrix::new_identity_matrix(4),
        }
    }
}

#[cfg(test)]
mod camera_tests {
    use crate::tuple::Tuple;
    use std::f64::consts::PI;

    use super::*;

    #[test]
    ///Constructing a camera
    fn translation_multiplication() {
        let hsize = 160;
        let vsize = 120;
        let field_of_view = PI / 2.0;

        let camera = Camera::new(hsize, vsize, field_of_view);

        assert_eq!(camera.hsize, hsize);
        assert_eq!(camera.vsize, vsize);
        assert_eq!(camera.field_of_view, field_of_view);
    }

}
