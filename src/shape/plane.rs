use uuid::Uuid;

use crate::matrix::{Matrix, memoized_inverse};
use crate::ray::{Intersection, Ray};
use crate::reflection::Material;
use crate::shape::object::Object;
use crate::tuple::Tuple;

use super::shape::Shape;

impl Object {
    pub fn new_plane() -> Object {
        Object {
            id: Uuid::new_v4(),
            transform: Matrix::new_identity_matrix(4),
            material: Material::default_material(),
            shape: Shape::Plane(),
        }
    }
}

#[cfg(test)]
mod transformation_tests {
    use super::*;
    use crate::Color;
    use crate::transformation;
    use std::f64::consts::PI;

    #[test]
    // Scenario: The normal of a plane is constant everywhere
    fn test_normal_plane() {
        let p = Object::new_plane();
        let n = p
            .shape
            .local_normal_at(p.clone(), Tuple::new_point(0.0, 0.0, 0.0));
        let n2 = p
            .shape
            .local_normal_at(p.clone(), Tuple::new_point(10.0, 0.0, -10.0));
        let n3 = p
            .shape
            .local_normal_at(p.clone(), Tuple::new_point(-5.0, 0.0, 150.0));
        assert_eq!(n, Tuple::new_vector(0.0, 1.0, 0.0));
        assert_eq!(n2, Tuple::new_vector(0.0, 1.0, 0.0));
        assert_eq!(n3, Tuple::new_vector(0.0, 1.0, 0.0));
    }

    #[test]
    //Scenario: Intersect with a ray parallel to the plane
    fn test_ray_paralle() {
        let mut p = Object::new_plane();
        let r = Ray::new(
            Tuple::new_point(0.0, 10.0, 0.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        let xs = p.shape.local_intersect(p.clone(),r);
        assert!(xs.is_empty())
    }

    #[test]
    // Scenario: Intersect with a coplanar ray
    fn test_ray_coplanaire() {
        let mut p = Object::new_plane();
        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, 0.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        let xs = p.shape.local_intersect(p.clone(),r);
        assert!(xs.is_empty())
    }

    #[test]
    // Scenario: A ray intersecting a plane from above
    fn test_intersection_from_above() {
        let mut p = Object::new_plane();
        let r = Ray::new(
            Tuple::new_point(0.0, 1.0, 0.0),
            Tuple::new_vector(0.0, -1.0, 0.0),
        );
        let xs = p.shape.local_intersect(p.clone(),r);
        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0].t, 1.0);
        assert!(xs[0].object == p);
    }

    #[test]
    fn test_intersection_from_below() {
        let mut p = Object::new_plane();
        let r = Ray::new(
            Tuple::new_point(0.0, -1.0, 0.0),
            Tuple::new_vector(0.0, 1.0, 0.0),
        );
        let xs = p.shape.local_intersect(p.clone(),r);
        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0].t, 1.0);
        assert!(xs[0].object == p);
    }
}
