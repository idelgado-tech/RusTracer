use std::fmt::{Debug, Formatter};

use crate::ray::{Intersection, Ray};
use crate::{matrix::Matrix, reflection};

use crate::{
    reflection::Material,
    tuple::*,
};
use uuid::Uuid;

static mut SAVED_RAY: Ray = Ray {
    direction: Tuple {
        x: 0.0,
        y: 0.0,
        z: 0.0,
        w: W::Point,
    },
    origin: Tuple {
        x: 0.0,
        y: 0.0,
        z: 0.0,
        w: W::Point,
    },
};

#[derive(Debug, Clone, PartialEq)]
pub struct ShapeTest {
    pub transform: Matrix,
    pub material: Material,
    pub id: Uuid,
}

impl ShapeTest {
    fn test_shape() -> ShapeTest {
        ShapeTest {
            transform: Matrix::new_identity_matrix(4),
            material: reflection::Material::default_material(),
            id: Uuid::new_v4(),
        }
    }
}

pub trait Shape {
    fn local_intersect(&self, local_ray: Ray) -> Vec<Intersection>;
    fn intersect(&self, ray: Ray) -> Vec<Intersection> {
        self.local_intersect(ray)
    } //remove MUT

    fn local_normal_at(&self, point: Tuple) -> Tuple;
    fn normal_at(&self, point: Tuple) -> Tuple {
        self.local_normal_at(point)
    }
    fn box_clone(&self) -> Box<dyn Shape>;

    fn get_transform(&self) -> Matrix;
    fn set_transform(&mut self, new_stransform: &Matrix);
    fn get_material(&self) -> Material;
    fn set_material(&mut self, new_material: &Material);
    fn get_id(&self) -> Uuid;
}

impl Shape for ShapeTest {
    fn local_intersect(&self, local_ray: Ray) -> Vec<Intersection> {
        unsafe {
            SAVED_RAY = local_ray.transform(&self.transform.inverse().unwrap());
        }
        vec![]
    }

    fn local_normal_at(&self, point: Tuple) -> Tuple {
        let local_point = self.transform.inverse().unwrap() * point;
        let local_normal = local_point;
        let mut world_normal = self.transform.inverse().unwrap().transpose() * local_normal;
        world_normal.w = W::from_int(0);
        world_normal.normalize()
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

impl Debug for dyn Shape {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.debug_tuple("")
            .field(&self.get_material())
            .field(&self.get_transform())
            .field(&self.get_id())
            .finish()
    }
}

impl PartialEq for dyn Shape {
    fn eq(&self, other: &Self) -> bool {
        self.get_id() == other.get_id()
            && self.get_transform() == other.get_transform()
            && self.get_material() == other.get_material()
    }
}

impl Clone for Box<dyn Shape> {
    fn clone(&self) -> Self {
        self.box_clone()
    }
}

#[cfg(test)]
mod transformation_tests {
    use super::*;
    use crate::transformation;
    use crate::Color;
    use std::f64::consts::PI;

    #[test]
    // Scenario: The default transformation
    fn default_test_shape() {
        let s = ShapeTest::test_shape();
        assert_eq!(s.transform, Matrix::new_identity_matrix(4));
    }

    #[test]
    //Assigning a transformation
    fn assign_test_shape() {
        let mut s = ShapeTest::test_shape();
        s.set_transform(&transformation::create_translation(2.0, 3.0, 4.0));
        assert_eq!(
            s.transform,
            transformation::create_translation(2.0, 3.0, 4.0)
        );
    }

    #[test]
    ///The default material
    fn test_shape_default_material() {
        let s = ShapeTest::test_shape();
        let material = s.material;
        assert_eq!(material.color, Color::new_color(1.0, 1.0, 1.0));
        assert_eq!(material.ambiant, 0.1);
        assert_eq!(material.diffuse, 0.9);
        assert_eq!(material.specular, 0.9);
        assert_eq!(material.shininess, 200.0);
    }

    #[test]
    ///A sphere may be assigned a material
    fn test_shape_material_creation() {
        let mut s = ShapeTest::test_shape();
        let mut material = Material::default_material();
        material.ambiant = 1.0;
        s.material = material.clone();
        assert_eq!(s.material, material);
    }
    #[test]
    // Scenario: Intersecting a scaled shape with a ray
    fn test_shape_intersection_ray() {
        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, -5.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        let mut s = ShapeTest::test_shape();
        s.set_transform(&transformation::create_scaling(2.0, 2.0, 2.0));
        let _: Vec<Intersection> = s.intersect(r);
        unsafe {
            assert_eq!(SAVED_RAY.origin, Tuple::new_point(0.0, 0.0, -2.5));
            assert_eq!(SAVED_RAY.direction, Tuple::new_vector(0.0, 0.0, 0.5));
        }
    }

    #[test]
    // Scenario: Intersecting a translated shape with a ray
    fn test_shape_translated_intersection_ray() {
        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, -5.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        let mut s = ShapeTest::test_shape();
        s.set_transform(&transformation::create_translation(5.0, 0.0, 0.0));
        let _: Vec<Intersection> = s.intersect(r);
        unsafe {
            assert_eq!(SAVED_RAY.origin, Tuple::new_point(-5.0, 0.0, -5.0));
            assert_eq!(SAVED_RAY.direction, Tuple::new_vector(0.0, 0.0, 1.0));
        }
    }

    #[test]
    //Scenario: Computing the normal on a translated shape
    fn test_normal_shape() {
        let mut s = ShapeTest::test_shape();
        s.set_transform(&transformation::create_translation(0.0, 1.0, 0.0));
        let n = Shape::normal_at(&s, Tuple::new_point(0.0, 1.70711, -0.70711));
        assert_eq!(n, Tuple::new_vector(0.0, 0.70711, -0.70711));
    }

    #[test]
    // Scenario: Computing the normal on a transformed shape
    fn test_normal_shape_transform() {
        let mut s = ShapeTest::test_shape();
        let m = transformation::create_scaling(1.0, 0.5, 1.0)
            * transformation::create_rotation_z(PI / 5.0);
        s.set_transform(&m);
        let n = Shape::normal_at(
            &s,
            Tuple::new_point(0.0, (2.0_f64.sqrt()) / 2.0, -(2.0_f64.sqrt()) / 2.0),
        );
        assert_eq!(n, Tuple::new_vector(0.0, 0.97014, -0.24254));
    }
}
