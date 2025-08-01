use uuid::Uuid;

use crate::matrix::{Matrix, memoized_inverse};
use crate::ray::{Intersection, Ray};
use crate::reflection::Material;
use crate::shape::object::Object;
use crate::tuple::{self, Tuple};

use super::shape::Shape;

impl Object {
    pub fn new_sphere() -> Object {
        Object {
            shape: Shape::Sphere {
                origin: Tuple::new_point(0.0, 0.0, 0.0),
                radius: 1.0,
            },
            id: Uuid::new_v4(),
            transform: Matrix::new_identity_matrix(4),
            material: Material::default_material(),
            shadow: true,
        }
    }

    pub fn new_glass_sphere() -> Object {
        let mut material = Material::default_material();
        material.transparency = 1.0;
        material.refractive_index = 1.5;

        Object {
            shape: Shape::Sphere {
                origin: Tuple::new_point(0.0, 0.0, 0.0),
                radius: 1.0,
            },
            id: Uuid::new_v4(),
            transform: Matrix::new_identity_matrix(4),
            material,
            shadow: true,
        }
    }
}

#[cfg(test)]
mod sphere_tests {
    use super::*;
    use crate::transformation;
    use std::f64::consts::PI;

    #[test]
    ///The normal on a sphere at a point on the x axis
    fn glassy_sphere_test() {
        let s = Object::new_glass_sphere();

        assert_eq!(s.transform, Matrix::new_identity_matrix(4));
        assert_eq!(s.material.transparency, 1.0);
        assert_eq!(s.material.refractive_index, 1.5);
    }

    #[test]
    ///The normal on a sphere at a point on the x axis
    fn sphere_normal_x() {
        let s = Object::new_sphere();
        let n = s
            .shape
            .local_normal_at(s.clone(), Tuple::new_point(1.0, 0.0, 0.0));

        assert_eq!(n, Tuple::new_vector(1.0, 0.0, 0.0));
    }

    #[test]
    ///The normal on a sphere at a point on the y axis
    fn sphere_normal_y() {
        let s = Object::new_sphere();
        let n = s
            .shape
            .local_normal_at(s.clone(), Tuple::new_point(0.0, 1.0, 0.0));

        assert_eq!(n, Tuple::new_vector(0.0, 1.0, 0.0));
    }

    #[test]
    ///The normal on a sphere at a point on the z axis
    fn sphere_normal_z() {
        let s = Object::new_sphere();
        let n = s
            .shape
            .local_normal_at(s.clone(), Tuple::new_point(0.0, 0.0, 1.0));

        assert_eq!(n, Tuple::new_vector(0.0, 0.0, 1.0));
    }

    #[test]
    ///Computing the normal on a translated sphere
    fn sphere_normal_translation() {
        let mut s = Object::new_sphere();
        s.set_transform(&transformation::create_translation(0.0, 1.0, 0.0));
        let n = s.shape.local_normal_at(
            s.clone(),
            Tuple::new_point(0.0, 1.7071067811865475, -0.7071067811865476),
        );

        assert_eq!(
            n,
            Tuple::new_vector(0.0, 0.7071067811865475, -0.7071067811865476)
        );
    }

    #[test]
    ///Computing the normal on a translated sphere
    fn sphere_normal_transformed() {
        let mut s = Object::new_sphere();
        let transformation = transformation::create_scaling(1.0, 0.5, 1.0)
            * transformation::create_rotation_z(PI / 5.0);
        s.set_transform(&transformation);
        let n = s.shape.local_normal_at(
            s.clone(),
            Tuple::new_point(0.0, 2.0_f64.sqrt() / 2.0, -2.0_f64.sqrt() / 2.0),
        );

        assert_eq!(
            n,
            Tuple::new_vector(0.0, 0.9701425001453319, -0.24253562503633294)
        );
    }

    #[test]
    ///The normal on a sphere at a point on the y axis
    fn sphere_normal_nonaxial() {
        let s = Object::new_sphere();
        let n = s.shape.local_normal_at(
            s.clone(),
            Tuple::new_point(
                3.0_f64.sqrt() / 3.0,
                3.0_f64.sqrt() / 3.0,
                3.0_f64.sqrt() / 3.0,
            ),
        );

        assert_eq!(
            n,
            Tuple::new_vector(
                3.0_f64.sqrt() / 3.0,
                3.0_f64.sqrt() / 3.0,
                3.0_f64.sqrt() / 3.0
            )
        );
    }

    #[test]
    ///The normal on a sphere at a point on the y axis
    fn sphere_normalized() {
        let s = Object::new_sphere();
        let n = s.shape.local_normal_at(
            s.clone(),
            Tuple::new_point(
                3.0_f64.sqrt() / 3.0,
                3.0_f64.sqrt() / 3.0,
                3.0_f64.sqrt() / 3.0,
            ),
        );

        assert_eq!(n.clone(), n.normalize());
    }

    #[test]
    ///A sphere's default transformation
    fn sphere_default() {
        let s = Object::new_sphere();
        assert_eq!(s.transform, Matrix::new_identity_matrix(4));
    }

    #[test]
    ///A sphere's default transformation
    fn sphere_tranformation() {
        let mut s = Object::new_sphere();
        let t = transformation::create_translation(2.0, 3.0, 4.0);
        s.set_transform(&t);

        assert_eq!(s.transform, t.clone());
    }

    #[test]
    ///Intersecting a scaled sphere with a ray
    fn sphere_scaled() {
        let origin = Tuple::new_point(0.0, 0.0, -5.0);
        let direction = Tuple::new_vector(0.0, 0.0, 1.0);
        let ray = Ray::new(origin, direction);
        let mut s = Object::new_sphere();
        s.set_transform(&transformation::create_scaling(2.0, 2.0, 2.0));
        let xs = s.intersect(ray);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 3.0);
        assert_eq!(xs[1].t, 7.0);
    }

    #[test]
    ///Intersecting a scaled sphere with a ray
    fn sphere_translated() {
        let origin = Tuple::new_point(0.0, 0.0, -5.0);
        let direction = Tuple::new_vector(0.0, 0.0, 1.0);
        let ray = Ray::new(origin, direction);
        let mut s = Object::new_sphere();
        s.set_transform(&transformation::create_translation(5.0, 0.0, 0.0));
        let xs = s.intersect(ray);

        assert_eq!(xs.len(), 0);
    }
}
