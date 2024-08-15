use crate::{
    matrix::*,
    shape::shape::Shape,
    tuple::*,
};

#[derive(Debug, Clone, PartialEq)]
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
            panic!("Ray::new direction must be a vector")
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

#[derive(Debug, Clone)]
pub struct Intersection {
    pub t: f64,
    pub object: Box<dyn Shape>,
}

impl PartialEq for Intersection {
    fn eq(&self, other: &Self) -> bool {
        self.t == other.t && self.object == other.object.clone()
    }
}

impl Intersection {
    pub fn new(t: f64, object: Box<&dyn Shape>) -> Intersection {
        Intersection {
            t,
            object: object.box_clone(),
        }
    }
}

pub fn reflect(inv: &Tuple, normal: &Tuple) -> Tuple {
    inv.clone() - normal.clone() * 2.0 * Tuple::dot_product(&inv, &normal)
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
    use crate::{shape::sphere::Sphere, transformation};

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
        let xs = s.intersect(ray);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, 4.0);
        assert!((xs[0].object == s.box_clone()));
        assert_eq!(xs[1].t, 6.0);
        assert!((xs[1].object == s.box_clone()));
    }

    #[test]
    ///A ray intersects a sphere at a tangent
    fn ray_intersect_2() {
        let origin = Tuple::new_point(0.0, 1.0, -5.0);
        let direction = Tuple::new_vector(0.0, 0.0, 1.0);
        let ray = Ray::new(origin, direction);
        let s = Sphere::sphere();
        let xs = s.intersect(ray);

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
        let xs = s.intersect(ray);

        assert_eq!(xs.len(), 0);
    }

    #[test]
    ///A ray originates inside a sphere
    fn ray_intersect_4() {
        let origin = Tuple::new_point(0.0, 0.0, 0.0);
        let direction = Tuple::new_vector(0.0, 0.0, 1.0);
        let ray = Ray::new(origin, direction);
        let s = Sphere::sphere();
        let xs = s.intersect(ray);

        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].t, -1.0);
        assert_eq!(xs[1].t, 1.0);
    }

    #[test]
    ///An intersection encapsulates t and object
    fn intersection_creation() {
        let s = Sphere::sphere();
        let i = Intersection::new(3.5, Box::new(&s));

        assert_eq!(i.t, 3.5);
        assert!((i.object == Box::new(s)));
    }

    #[test]
    ///The hit, when all intersections have positive t
    fn hit_intersections_1() {
        let s = Sphere::sphere();
        let i1 = Intersection::new(1.0, Box::new(&s));
        let i2 = Intersection::new(2.0, Box::new(&s));
        let intersections = vec![i1.clone(), i2];
        let i = hit_intersections(intersections).unwrap();
        assert_eq!(i, i1);
    }

    #[test]
    ///The hit, when some intersections have negative t
    fn hit_intersections_2() {
        let s = Sphere::sphere();
        let i1 = Intersection::new(-1.0, Box::new(&s));
        let i2 = Intersection::new(2.0, Box::new(&s));
        let intersections = vec![i1.clone(), i2.clone()];
        let i = hit_intersections(intersections).unwrap();
        assert_eq!(i, i2);
    }

    #[test]
    ///The hit, when some intersections have negative t
    fn hit_intersections_3() {
        let s = Sphere::sphere();
        let i1 = Intersection::new(-1.0, Box::new(&s));
        let i2 = Intersection::new(-2.0, Box::new(&s));
        let intersections = vec![i1.clone(), i2.clone()];
        let i = hit_intersections(intersections);
        assert_eq!(i, Option::None);
    }

    #[test]
    ///Scenario​: The hit is always the lowest nonnegative intersection
    fn hit_intersections_4() {
        let s = Sphere::sphere();
        let i1 = Intersection::new(5.0, Box::new(&s));
        let i2 = Intersection::new(7.0, Box::new(&s));
        let i3 = Intersection::new(-2.0, Box::new(&s));
        let i4 = Intersection::new(2.0, Box::new(&s));

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
}
