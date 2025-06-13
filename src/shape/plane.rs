
use uuid::Uuid;

use crate::ray::{Intersection, Ray};
use crate::reflection::Material;
use crate::tuple::{Tuple};
use crate::matrix::Matrix;

use super::shape::Shape;

#[derive(Debug, Clone, PartialEq)]
pub struct Plane {
    pub transform: Matrix,
    pub material: Material,
    pub id: Uuid,
}

impl Plane {
    pub fn plane() -> Plane {
        Plane {
            id: Uuid::new_v4(),
            transform: Matrix::new_identity_matrix(4),
            material: Material::default_material(),
        }
    }
}

impl Shape for Plane {
    fn local_normal_at(&self, p: Tuple) -> Tuple {
        self.transform.inverse().unwrap() * Tuple::new_vector(0.0, 1.0, 0.0)
    }

    fn local_intersect(&self, local_ray: Ray) -> Vec<Intersection> {
        let transformed_ray = local_ray.transform(&self.transform.inverse().unwrap());
        if transformed_ray.direction.y.abs() < 0.00001 {
            vec![]
        } else {
            let t = -transformed_ray.origin.y / transformed_ray.direction.y;
            vec![Intersection::new(t, Box::new(&self.clone()))]
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
    use crate::Color;
    use std::f64::consts::PI;

    #[test]
    // Scenario: The normal of a plane is constant everywhere
    fn test_normal_plane() {
        let p = Plane::plane();
        let n = p.local_normal_at(Tuple::new_point(0.0, 0.0, 0.0));
        let n2 = p.local_normal_at(Tuple::new_point(10.0, 0.0, -10.0));
        let n3 = p.local_normal_at(Tuple::new_point(-5.0, 0.0, 150.0));
        assert_eq!(n, Tuple::new_vector(0.0, 1.0, 0.0));
        assert_eq!(n2, Tuple::new_vector(0.0, 1.0, 0.0));
        assert_eq!(n3, Tuple::new_vector(0.0, 1.0, 0.0));
    }

    #[test]
    //Scenario: Intersect with a ray parallel to the plane
    fn test_ray_paralle() {
        let p = Plane::plane();
        let r = Ray::new(
            Tuple::new_point(0.0, 10.0, 0.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        let xs = p.local_intersect(r);
        assert!(xs.is_empty())
    }

    #[test]
    // Scenario: Intersect with a coplanar ray
    fn test_ray_coplanaire() {
        let p = Plane::plane();
        let r = Ray::new(
            Tuple::new_point(0.0, 0.0, 0.0),
            Tuple::new_vector(0.0, 0.0, 1.0),
        );
        let xs = p.local_intersect(r);
        assert!(xs.is_empty())
    }

    #[test]
    // Scenario: A ray intersecting a plane from above
    fn test_intersection_from_above(){
        let p = Plane::plane();
        let r = Ray::new(
            Tuple::new_point(0.0, 1.0, 0.0),
            Tuple::new_vector(0.0, -1.0, 0.0),
        );
        let xs = p.local_intersect(r);
        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0].t,1.0);
        assert!(xs[0].object==p.box_clone());
    }

    #[test]
    fn test_intersection_from_below(){
        let p = Plane::plane();
        let r = Ray::new(
            Tuple::new_point(0.0, -1.0, 0.0),
            Tuple::new_vector(0.0, 1.0, 0.0),
        );
        let xs = p.local_intersect(r);
        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0].t,1.0);
        assert!(xs[0].object==p.box_clone());
}

}
