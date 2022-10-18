use std::vec;
use uuid::Uuid;

use crate::{
    matrix::*,
    reflection::Material,
    transformation,
    tuple::{self, *},
};

#[derive(Debug, Clone)]
pub struct Ray {
    pub origin: Tuple,
    pub direction: Tuple,
}

impl Ray {
    pub fn new(origin: Tuple, direction: Tuple) -> Ray {
        if origin.w != W::Point {
            panic!("Ray::new origin must be a point")
        }
        if direction.w != W::Vector {
            panic!("Ray::new origin must be a vector")
        }
        Ray { origin, direction }
    }

    pub fn position(&self, time: f64) -> Tuple {
        self.origin.clone() + self.direction.clone() * time
    }

    pub fn transform(&self, matrix: &Matrix) -> Ray {
        Ray {
            origin: matrix * self.origin.clone(),
            direction: matrix * self.direction.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Intersection {
    pub t: f64,
    pub object: Sphere,
}

impl Intersection {
    pub fn new(t: f64, object: &Sphere) -> Intersection {
        Intersection {
            t,
            object: object.clone(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Sphere {
    pub origin: Tuple,
    pub radius: f64,
    pub id: Uuid,
    pub transform: Matrix,
    pub material: Material,
}

impl Sphere {
    pub fn sphere() -> Sphere {
        Sphere {
            origin: Tuple::new_point(0.0, 0.0, 0.0),
            radius: 1.0,
            id: Uuid::new_v4(),
            transform: Matrix::new_identity_matrix(4),
            material: Material::material(),
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

pub fn reflect(inv: &Tuple, normal: &Tuple) -> Tuple {
    inv.clone() - normal.clone() * 2.0 * Tuple::dot_product(&inv, &normal)
}

pub fn intersect(sphere: &Sphere, ray: Ray) -> Vec<Intersection> {
    let transformed_ray = ray.transform(&sphere.transform.inverse().unwrap());
    let sphere_to_ray = transformed_ray.origin - sphere.clone().origin;
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
            Intersection::new(t1, &sphere),
            Intersection::new(t2, &sphere),
        ]
    }
}

pub fn hit_intersections(intersections: Vec<Intersection>) -> Option<Intersection> {
    let mut tmp_instersections = intersections.clone();
    tmp_instersections.retain(|value| value.t > 0.0);
    tmp_instersections.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
    if tmp_instersections.is_empty() {
        Option::None
    } else {
        Option::Some(tmp_instersections[0].clone())
    }
}

#[cfg(test)]
mod transformation_tests {
    use super::*;
    use crate::transformation;
    use std::f64::consts::PI;

    #[test]
    ///Reflecting a vector approaching at 45°
    fn reflection() {
        let v = Tuple::new_vector(1.0, -1.0, 0.0);
        let n = Tuple::new_vector(0.0, 1.0, 0.0);

        let r = reflect(&v, &n);
        assert_eq!(r, Tuple::new_vector(1.0, 1.0, 0.0));
    }

    #[test]
    ///Reflecting a vector off a slanted surface
    fn reflection_slanted() {
        let v = Tuple::new_vector(0.0, -1.0, 0.0);
        let n = Tuple::new_vector(2.0_f64.sqrt() / 2.0, 2.0_f64.sqrt() / 2.0, 0.0);

        let r = reflect(&v, &n);
        assert_eq!(r, Tuple::new_vector(1.0, 0.0, 0.0));
    }

    #[test]
    ///The normal on a sphere at a point on the x axis
    fn sphere_normal_x() {
        let s = Sphere::sphere();
        let n = s.normal_at_point(&Tuple::new_point(1.0, 0.0, 0.0));

        assert_eq!(n, Tuple::new_vector(1.0, 0.0, 0.0));
    }

    #[test]
    ///The normal on a sphere at a point on the y axis
    fn sphere_normal_y() {
        let s = Sphere::sphere();
        let n = s.normal_at_point(&Tuple::new_point(0.0, 1.0, 0.0));

        assert_eq!(n, Tuple::new_vector(0.0, 1.0, 0.0));
    }

    #[test]
    ///The normal on a sphere at a point on the z axis
    fn sphere_normal_z() {
        let s = Sphere::sphere();
        let n = s.normal_at_point(&Tuple::new_point(0.0, 0.0, 1.0));

        assert_eq!(n, Tuple::new_vector(0.0, 0.0, 1.0));
    }

    #[test]
    ///Computing the normal on a translated sphere
    fn sphere_normal_translation() {
        let mut s = Sphere::sphere();
        s.set_transform(&transformation::create_translation(0.0, 1.0, 0.0));
        let n = s.normal_at_point(&Tuple::new_point(
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
        let n = s.normal_at_point(&Tuple::new_point(
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
        let n = s.normal_at_point(&Tuple::new_point(
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
        let n = s.normal_at_point(&Tuple::new_point(
            3.0_f64.sqrt() / 3.0,
            3.0_f64.sqrt() / 3.0,
            3.0_f64.sqrt() / 3.0,
        ));

        assert_eq!(n.clone(), n.normalize());
    }

    #[test]
    ///Creating a ray
    fn ray_creation() {
        let origin = Tuple::new_point(1.0, 2.0, 3.0);
        let direction = Tuple::new_vector(4.0, 5.0, 6.0);
        let ray = Ray::new(origin.clone(), direction.clone());

        assert_eq!(ray.origin, origin);
        assert_eq!(ray.direction, direction)
    }

    #[test]
    ///Computing a point from a distance
    fn ray_computing() {
        let origin = Tuple::new_point(2.0, 3.0, 4.0);
        let direction = Tuple::new_vector(1.0, 0.0, 0.0);
        let ray = Ray::new(origin, direction);

        assert_eq!(ray.position(0.0), Tuple::new_point(2.0, 3.0, 4.0));
        assert_eq!(ray.clone().position(1.0), Tuple::new_point(3.0, 3.0, 4.0));
        assert_eq!(ray.clone().position(-1.0), Tuple::new_point(1.0, 3.0, 4.0));
        assert_eq!(ray.clone().position(2.5), Tuple::new_point(4.5, 3.0, 4.0));
    }

    #[test]
    ///A ray intersects a sphere at two points
    fn ray_intersect_1() {
        let origin = Tuple::new_point(0.0, 0.0, -5.0);
        let direction = Tuple::new_vector(0.0, 0.0, 1.0);
        let ray = Ray::new(origin, direction);
        let s = Sphere::sphere();
        let xs = intersect(&s, ray);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 4.0);
        assert_eq!(xs[0].object, s);
        assert_eq!(xs[1].t, 6.0);
        assert_eq!(xs[1].object, s);
    }

    #[test]
    ///A ray intersects a sphere at a tangent
    fn ray_intersect_2() {
        let origin = Tuple::new_point(0.0, 1.0, -5.0);
        let direction = Tuple::new_vector(0.0, 0.0, 1.0);
        let ray = Ray::new(origin, direction);
        let s = Sphere::sphere();
        let xs = intersect(&s, ray);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 5.0);
        assert_eq!(xs[1].t, 5.0);
    }

    #[test]
    ///A ray misses a sphere
    fn ray_intersect_3() {
        let origin = Tuple::new_point(0.0, 2.0, -5.0);
        let direction = Tuple::new_vector(0.0, 0.0, 1.0);
        let ray = Ray::new(origin, direction);
        let s = Sphere::sphere();
        let xs = intersect(&s, ray);

        assert_eq!(xs.len(), 0);
    }

    #[test]
    ///A ray originates inside a sphere
    fn ray_intersect_4() {
        let origin = Tuple::new_point(0.0, 0.0, 0.0);
        let direction = Tuple::new_vector(0.0, 0.0, 1.0);
        let ray = Ray::new(origin, direction);
        let s = Sphere::sphere();
        let xs = intersect(&s, ray);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -1.0);
        assert_eq!(xs[1].t, 1.0);
    }

    #[test]
    ///An intersection encapsulates t and object
    fn intersection_creation() {
        let s = Sphere::sphere();
        let i = Intersection::new(3.5, &s);

        assert_eq!(i.t, 3.5);
        assert_eq!(i.object, s);
    }

    #[test]
    ///The hit, when all intersections have positive t
    fn hit_intersections_1() {
        let s = Sphere::sphere();
        let i1 = Intersection::new(1.0, &s);
        let i2 = Intersection::new(2.0, &s);
        let intersections = vec![i1.clone(), i2];
        let i = hit_intersections(intersections).unwrap();
        assert_eq!(i, i1);
    }

    #[test]
    ///The hit, when some intersections have negative t
    fn hit_intersections_2() {
        let s = Sphere::sphere();
        let i1 = Intersection::new(-1.0, &s);
        let i2 = Intersection::new(2.0, &s);
        let intersections = vec![i1.clone(), i2.clone()];
        let i = hit_intersections(intersections).unwrap();
        assert_eq!(i, i2);
    }

    #[test]
    ///The hit, when some intersections have negative t
    fn hit_intersections_3() {
        let s = Sphere::sphere();
        let i1 = Intersection::new(-1.0, &s);
        let i2 = Intersection::new(-2.0, &s);
        let intersections = vec![i1.clone(), i2.clone()];
        let i = hit_intersections(intersections);
        assert_eq!(i, Option::None);
    }

    #[test]
    ///Scenario​: The hit is always the lowest nonnegative intersection
    fn hit_intersections_4() {
        let s = Sphere::sphere();
        let i1 = Intersection::new(5.0, &s);
        let i2 = Intersection::new(7.0, &s);
        let i3 = Intersection::new(-2.0, &s);
        let i4 = Intersection::new(2.0, &s);

        let intersections = vec![i1.clone(), i2.clone(), i3.clone(), i4.clone()];
        let i = hit_intersections(intersections).unwrap();
        assert_eq!(i, i4);
    }

    #[test]
    ///Translating a ray
    fn translation_ray() {
        let origin = Tuple::new_point(1.0, 2.0, 3.0);
        let direction = Tuple::new_vector(0.0, 1.0, 0.0);
        let ray = Ray::new(origin, direction);
        let m = transformation::create_translation(3.0, 4.0, 5.0);
        let r2 = ray.transform(&m);

        assert_eq!(r2.origin, Tuple::new_point(4.0, 6.0, 8.0));
        assert_eq!(r2.direction, Tuple::new_vector(0.0, 1.0, 0.0));
    }

    #[test]
    ///Scaling a ray
    fn scaling_ray() {
        let origin = Tuple::new_point(1.0, 2.0, 3.0);
        let direction = Tuple::new_vector(0.0, 1.0, 0.0);
        let ray = Ray::new(origin, direction);
        let m = transformation::create_scaling(2.0, 3.0, 4.0);
        let r2 = ray.transform(&m);

        assert_eq!(r2.origin, Tuple::new_point(2.0, 6.0, 12.0));
        assert_eq!(r2.direction, Tuple::new_vector(0.0, 3.0, 0.0));
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
        let xs = intersect(&s, ray);

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
        let xs = intersect(&s, ray);

        assert_eq!(xs.len(), 0);
    }
}
