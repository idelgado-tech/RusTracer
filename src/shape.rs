use crate::ray::{Ray, Intersection};
use crate::{matrix::Matrix, reflection};

use crate::{
    matrix::*,
    reflection::Material,
    transformation,
    tuple::{self, *},
};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub struct ShapeTest {
    pub transform: Matrix,
    pub material: Material,
    pub id: Uuid,
}

impl ShapeTest {
    fn Shape() -> Shape {
        Shape {
            transform: Matrix::new_identity_matrix(4),
            material: reflection::Material::material(),
            id: Uuid::new_v4(),
        }
    }

    pub fn set_transform(&mut self, new_stransform: &Matrix) -> () {
        self.transform = new_stransform.clone();
    }

    pub fn normal_at_point(&self, p: &Tuple) -> Tuple {
        let object_point = self.transform.inverse().unwrap() * p.clone();
        let object_normal = object_point - Tuple::new_point(0.0, 0.0, 0.0);
        let mut world_normal = self.transform.inverse().unwrap().transpose() * object_normal;
        world_normal.w = tuple::W::from_int(0);
        world_normal.normalize()
    }
}

pub trait Shape {
    fn intersect(shape: &Shape, ray: Ray) -> Vec<Intersection>;
}



#[cfg(test)]
mod transformation_tests {
    use super::*;
    use crate::transformation;
    use std::f64::consts::PI;

    #[test]
    ///Reflecting a vector approaching at 45°
    fn test_shape() {}
}
