use uuid::Uuid;

use crate::ray::{Intersection, Ray};
use crate::reflection::Material;
use crate::tuple::{self, Tuple};
use crate::{matrix::Matrix};

use super::shape::Shape;

#[derive(Debug, Clone, PartialEq)]
pub struct Sphere {
    pub origin: Tuple,
    pub radius: f64,
    pub transform: Matrix,
    pub material: Material,
    pub id: Uuid,
}

impl Sphere {
    pub fn sphere() -> Sphere {
        Sphere {
            origin: Tuple::new_point(0.0, 0.0, 0.0),
            radius: 1.0,
            id: Uuid::new_v4(),
            transform: Matrix::new_identity_matrix(4),
            material: Material::default_material(),
        }
    }
}

impl Shape for Sphere {
    fn local_normal_at(&self, p: Tuple) -> Tuple {
        let object_point = self.transform.inverse().unwrap() * p.clone();
        let object_normal = object_point - Tuple::new_point(0.0, 0.0, 0.0);
        let mut world_normal = self.transform.inverse().unwrap().transpose() * object_normal;
        world_normal.w = tuple::W::from_int(0);
        world_normal.normalize()
    }

    fn local_intersect(&self, local_ray: Ray) -> Vec<Intersection> {
        let transformed_ray = local_ray.transform(&self.transform.inverse().unwrap());
        let sphere_to_ray = transformed_ray.origin - self.clone().origin;
        let a = Tuple::dot_product(&transformed_ray.direction, &transformed_ray.direction);
        let b = 2.0 * Tuple::dot_product(&transformed_ray.direction, &sphere_to_ray);
        let c = Tuple::dot_product(&sphere_to_ray, &sphere_to_ray) - 1.0;
        let discriminant = b.powi(2) - 4.0 * a * c;

        if discriminant < 0.0 {
            vec![]
        } else {
            let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
            let t2 = (-b + discriminant.sqrt()) / (2.0 * a);
            vec![
                Intersection::new(t1, Box::new(self)),
                Intersection::new(t2, Box::new(self)),
            ]
        }
    }

    // Default implem
    fn set_transform(&mut self, new_stransform: &Matrix) {
        self.transform = new_stransform.clone();
    }
    fn get_transform(&self) -> Matrix {
        self.transform.clone()
    }

    fn set_material(&mut self, new_material: &Material) {
        self.material = new_material.clone();
    }
    fn get_material(&self) -> Material {
        self.material.clone()
    }

    fn get_id(&self) -> Uuid {
        self.id
    }

    fn box_clone(&self) -> Box<dyn Shape> {
        Box::new((*self).clone())
    }
}

#[cfg(test)]
mod transformation_tests {
    use super::*;
    use crate::transformation;
    use std::f64::consts::PI;

    #[test]
    ///The normal on a sphere at a point on the x axis
    fn sphere_normal_x() {
        let s = Sphere::sphere();
        let n = s.local_normal_at(Tuple::new_point(1.0, 0.0, 0.0));

        assert_eq!(n, Tuple::new_vector(1.0, 0.0, 0.0));
    }

    #[test]
    ///The normal on a sphere at a point on the y axis
    fn sphere_normal_y() {
        let s = Sphere::sphere();
        let n = s.local_normal_at(Tuple::new_point(0.0, 1.0, 0.0));

        assert_eq!(n, Tuple::new_vector(0.0, 1.0, 0.0));
    }

    #[test]
    ///The normal on a sphere at a point on the z axis
    fn sphere_normal_z() {
        let s = Sphere::sphere();
        let n = s.local_normal_at(Tuple::new_point(0.0, 0.0, 1.0));

        assert_eq!(n, Tuple::new_vector(0.0, 0.0, 1.0));
    }

    #[test]
    ///Computing the normal on a translated sphere
    fn sphere_normal_translation() {
        let mut s = Sphere::sphere();
        s.set_transform(&transformation::create_translation(0.0, 1.0, 0.0));
        let n = s.local_normal_at(Tuple::new_point(
            0.0,
            1.7071067811865475,
            -0.7071067811865476,
        ));

        assert_eq!(
            n,
            Tuple::new_vector(0.0, 0.7071067811865475, -0.7071067811865476)
        );
    }

    #[test]
    ///Computing the normal on a translated sphere
    fn sphere_normal_transformed() {
        let mut s = Sphere::sphere();
        let transformation = transformation::create_scaling(1.0, 0.5, 1.0)
            * transformation::create_rotation_z(PI / 5.0);
        s.set_transform(&transformation);
        let n = s.local_normal_at(Tuple::new_point(
            0.0,
            2.0_f64.sqrt() / 2.0,
            -2.0_f64.sqrt() / 2.0,
        ));

        assert_eq!(
            n,
            Tuple::new_vector(0.0, 0.9701425001453319, -0.24253562503633294)
        );
    }

    #[test]
    ///The normal on a sphere at a point on the y axis
    fn sphere_normal_nonaxial() {
        let s = Sphere::sphere();
        let n = s.local_normal_at(Tuple::new_point(
            3.0_f64.sqrt() / 3.0,
            3.0_f64.sqrt() / 3.0,
            3.0_f64.sqrt() / 3.0,
        ));

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
        let s = Sphere::sphere();
        let n = s.local_normal_at(Tuple::new_point(
            3.0_f64.sqrt() / 3.0,
            3.0_f64.sqrt() / 3.0,
            3.0_f64.sqrt() / 3.0,
        ));

        assert_eq!(n.clone(), n.normalize());
    }

    #[test]
    ///A sphere's default transformation
    fn sphere_default() {
        let s = Sphere::sphere();
        assert_eq!(s.transform, Matrix::new_identity_matrix(4));
    }

    #[test]
    ///A sphere's default transformation
    fn sphere_tranformation() {
        let mut s = Sphere::sphere();
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
        let mut s = Sphere::sphere();
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
        let mut s = Sphere::sphere();
        s.set_transform(&transformation::create_translation(5.0, 0.0, 0.0));
        let xs = s.intersect(ray);

        assert_eq!(xs.len(), 0);
    }
}
