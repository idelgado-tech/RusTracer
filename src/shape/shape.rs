use std::fmt::{Debug, Formatter};

use crate::color::Color;
use crate::matrix::memoized_inverse;
use crate::pattern::Pattern;
use crate::ray::{Intersection, Ray};
use crate::shape::object::Object;
use crate::tuple;
use crate::{matrix::Matrix, reflection};

use crate::{reflection::Material, tuple::*};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq)]
pub enum Shape {
    ShapeTest { saved_ray: Ray },
    Sphere { origin: Tuple, radius: f64 },
    Plane(),
}

impl Object {
    fn new_test_shape() -> Object {
        Object {
            transform: Matrix::new_identity_matrix(4),
            material: reflection::Material::default_material(),
            id: Uuid::new_v4(),
            shape: Shape::ShapeTest {
                saved_ray: Ray {
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
                },
            },
        }
    }
}

impl Shape {
    pub fn local_intersect(&mut self, object: Object, local_ray: Ray) -> Vec<Intersection> {
        match self {
            Shape::ShapeTest { saved_ray } => {
                *saved_ray =
                    local_ray.transform(&memoized_inverse(object.transform.clone()).unwrap());
                vec![]
            }
            Shape::Sphere { origin, radius: _ } => {
                let transformed_ray =
                    local_ray.transform(&memoized_inverse(object.transform.clone()).unwrap());
                let sphere_to_ray = transformed_ray.origin - origin.to_owned();
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
                        Intersection::new(t1, &object),
                        Intersection::new(t2, &object),
                    ]
                }
            }
            Shape::Plane() => {
                let transformed_ray =
                    local_ray.transform(&memoized_inverse(object.transform.clone()).unwrap());
                if transformed_ray.direction.y.abs() < 0.00001 {
                    vec![]
                } else {
                    let t = -transformed_ray.origin.y / transformed_ray.direction.y;
                    vec![Intersection::new(t, &object)]
                }
            }
        }
    }

    pub fn local_normal_at(&self, object: Object, point: Tuple) -> Tuple {
        match self {
            Shape::ShapeTest { saved_ray: _ } => {
                let local_point = memoized_inverse(object.transform.clone()).unwrap() * point;
                let local_normal = local_point;
                let mut world_normal =
                    memoized_inverse(object.transform.clone()).unwrap() * local_normal;
                world_normal.w = W::from_int(0);
                world_normal.normalize()
            }
            Shape::Sphere {
                origin: _,
                radius: _,
            } => {
                let object_point =
                    memoized_inverse(object.transform.clone()).unwrap() * point.clone();
                let object_normal = object_point - Tuple::new_point(0.0, 0.0, 0.0);
                let mut world_normal = memoized_inverse(object.transform.clone())
                    .unwrap()
                    .transpose()
                    * object_normal;
                world_normal.w = tuple::W::from_int(0);
                world_normal.normalize()
            }
            Shape::Plane() => {
                memoized_inverse(object.transform.clone()).unwrap()
                    * Tuple::new_vector(0.0, 1.0, 0.0)
            }
        }
    }
}

#[cfg(test)]
mod shape_tests {
    use super::*;
    use crate::Color;
    use crate::transformation;
    use std::f64::consts::PI;

    #[test]
    // Scenario: The default transformation
    fn default_test_shape() {
        let s = Object::new_test_shape();
        assert_eq!(s.transform, Matrix::new_identity_matrix(4));
    }

    #[test]
    //Assigning a transformation
    fn assign_test_shape() {
        let mut s = Object::new_test_shape();
        s.set_transform(&transformation::create_translation(2.0, 3.0, 4.0));
        assert_eq!(
            s.transform,
            transformation::create_translation(2.0, 3.0, 4.0)
        );
    }

    #[test]
    ///The default material
    fn test_shape_default_material() {
        let s = Object::new_test_shape();
        let material = s.material;
        assert_eq!(material.color, Color::new_color(1.0, 1.0, 1.0));
        assert_eq!(material.ambient, 0.1);
        assert_eq!(material.diffuse, 0.9);
        assert_eq!(material.specular, 0.9);
        assert_eq!(material.shininess, 200.0);
    }

    #[test]
    ///A sphere may be assigned a material
    fn test_shape_material_creation() {
        let mut s = Object::new_test_shape();
        let mut material = Material::default_material();
        material.ambient = 1.0;
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
        let mut s = Object::new_test_shape();
        s.set_transform(&transformation::create_scaling(2.0, 2.0, 2.0));
        let _: Vec<Intersection> = s.intersect(r);
        if let Shape::ShapeTest { saved_ray } = s.shape {
            assert_eq!(saved_ray.origin, Tuple::new_point(0.0, 0.0, -2.5));
            assert_eq!(saved_ray.direction, Tuple::new_vector(0.0, 0.0, 0.5));
        } else {
            panic!("Shuold not happend")
        }
    }

    #[test]
    // Scenario: Intersecting a translated shape with a ray
    fn test_shape_translated_intersection_ray() {
        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, -5.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        let mut s = Object::new_test_shape();
        s.set_transform(&transformation::create_translation(5.0, 0.0, 0.0));
        let _: Vec<Intersection> = s.intersect(r);

        if let Shape::ShapeTest { saved_ray } = s.shape {
            assert_eq!(saved_ray.origin, Tuple::new_point(-5.0, 0.0, -5.0));
            assert_eq!(saved_ray.direction, Tuple::new_vector(0.0, 0.0, 1.0));
        } else {
            panic!("Should not happend")
        }
    }

    #[test]
    //Scenario: Computing the normal on a translated shape
    fn test_normal_shape() {
        let mut s = Object::new_test_shape();
        s.set_transform(&transformation::create_translation(0.0, 1.0, 0.0));
        let n = Object::normal_at(&s, Tuple::new_point(0.0, 1.70711, -0.70711));
        assert_eq!(n, Tuple::new_vector(0.0, -0.38267, -0.92388));
    }

    #[test]
    // Scenario: Computing the normal on a transformed shape
    fn test_normal_shape_transform() {
        let mut s = Object::new_test_shape();
        let m = transformation::create_scaling(1.0, 0.5, 1.0)
            * transformation::create_rotation_z(PI / 5.0);
        s.set_transform(&m);
        let n = Object::normal_at(
            &s,
            Tuple::new_point(0.0, (2.0_f64.sqrt()) / 2.0, -(2.0_f64.sqrt()) / 2.0),
        );
        assert_eq!(n, Tuple::new_vector(0.795805, 0.537492, -0.27891));
    }
}
