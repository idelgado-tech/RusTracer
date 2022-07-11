use std::vec;
use uuid::Uuid;

use crate::tuple::*;

#[derive(Debug, Clone)]
pub struct Ray {
    pub origin: Tuple,
    pub direction: Tuple,
}

impl Ray {
    pub fn new(origin: Tuple, direction: Tuple) -> Ray {
        if origin.w != W::POINT {
            panic!("Ray::new origin must be a point")
        }
        if direction.w != W::VECTOR {
            panic!("Ray::new origin must be a vector")
        }
        Ray { origin, direction }
    }

    pub fn position(&self, time: f64) -> Tuple {
        self.origin.clone() + self.direction.clone() * time
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Intersection {
    t: f64,
    object: Sphere,
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
}

pub fn sphere() -> Sphere {
    Sphere {
        origin: Tuple::new_point(0.0, 0.0, 0.0),
        radius: 1.0,
        id: Uuid::new_v4(),
    }
}

pub fn intersect(sphere: &Sphere, ray: Ray) -> Vec<Intersection> {
    let sphere_to_ray = ray.origin - sphere.clone().origin;
    let a = Tuple::dot_product(&ray.direction, &ray.direction);
    let b = 2.0 * Tuple::dot_product(&ray.direction, &sphere_to_ray);
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
        let s = sphere();
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
        let s = sphere();
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
        let s = sphere();
        let xs = intersect(&s, ray);

        assert_eq!(xs.len(), 0);
    }

    #[test]
    ///A ray originates inside a sphere
    fn ray_intersect_4() {
        let origin = Tuple::new_point(0.0, 0.0, 0.0);
        let direction = Tuple::new_vector(0.0, 0.0, 1.0);
        let ray = Ray::new(origin, direction);
        let s = sphere();
        let xs = intersect(&s, ray);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -1.0);
        assert_eq!(xs[1].t, 1.0);
    }

    #[test]
    ///An intersection encapsulates t and object
    fn intersection_creation() {
        let s = sphere();
        let i = Intersection::new(3.5, &s);

        assert_eq!(i.t, 3.5);
        assert_eq!(i.object, s);
    }

    #[test]
    ///The hit, when all intersections have positive t
    fn hit_intersections_1() {
        let s = sphere();
        let i1 = Intersection::new(1.0, &s);
        let i2 = Intersection::new(2.0, &s);
        let intersections = vec![i1.clone(), i2];
        let i = hit_intersections(intersections).unwrap();
        assert_eq!(i, i1);
    }

    #[test]
    ///The hit, when some intersections have negative t
    fn hit_intersections_2() {
        let s = sphere();
        let i1 = Intersection::new(-1.0, &s);
        let i2 = Intersection::new(2.0, &s);
        let intersections = vec![i1.clone(), i2.clone()];
        let i = hit_intersections(intersections).unwrap();
        assert_eq!(i, i2);
    }

    #[test]
    ///The hit, when some intersections have negative t
    fn hit_intersections_3() {
        let s = sphere();
        let i1 = Intersection::new(-1.0, &s);
        let i2 = Intersection::new(-2.0, &s);
        let intersections = vec![i1.clone(), i2.clone()];
        let i = hit_intersections(intersections);
        assert_eq!(i, Option::None);
    }

    #[test]
    ///Scenario​: The hit is always the lowest nonnegative intersection
    fn hit_intersections_4() {
        let s = sphere();
        let i1 = Intersection::new(5.0, &s);
        let i2 = Intersection::new(7.0, &s);
        let i3 = Intersection::new(-2.0, &s);
        let i4 = Intersection::new(2.0, &s);

        let intersections = vec![i1.clone(), i2.clone(),i3.clone(),i4.clone()];
        let i = hit_intersections(intersections).unwrap();
        assert_eq!(i, i4);
    }

}
