use std::ops::Add;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Sub;

use crate::utils::*;

#[derive(PartialEq, Debug, Clone)]
pub enum W {
    Point,
    Vector,
}

impl Add for W {
    type Output = W;

    fn add(self, other: W) -> W {
        match (self, other) {
            (W::Point, W::Vector) => W::Point,
            (W::Vector, W::Point) => W::Point,
            (W::Vector, W::Vector) => W::Vector,
            (_, _) => panic!("W ADD , case not supported"),
        }
    }
}

impl Sub for W {
    type Output = W;

    fn sub(self, other: W) -> W {
        match (self, other) {
            (W::Point, W::Vector) => W::Point,
            (W::Vector, W::Vector) => W::Vector,
            (W::Point, W::Point) => W::Vector,
            (_, _) => panic!("W ADD Vector + Point, it don't mean anything, case not supported"),
        }
    }
}

impl W {
    pub fn from_int(float: isize) -> W {
        match float {
            0 => W::Vector,
            1 => W::Point,
            _ => panic!("Tuple must be point OR vector, input w value : {}", float),
        }
    }

    pub fn to_int(w: W) -> isize {
        match w {
            W::Vector => 0,
            W::Point => 1,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Tuple {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: W,
}

impl Tuple {
    pub fn new_point(x: f64, y: f64, z: f64) -> Tuple {
        Tuple {
            x,
            y,
            z,
            w: W::Point,
        }
    }

    pub fn new_vector(x: f64, y: f64, z: f64) -> Tuple {
        Tuple {
            x,
            y,
            z,
            w: W::Vector,
        }
    }

    pub fn new_tuple(x: f64, y: f64, z: f64, w: i64) -> Tuple {
        Tuple {
            x,
            y,
            z,
            w: match w {
                0 => W::Vector,
                1 => W::Point,
                _ => panic!("Tuple must be point OR vector, input w value : {}", w),
            },
        }
    }

    pub fn negate(self) -> Tuple {
        let zero = Tuple::new_tuple(0.0, 0.0, 0.0, 0);
        zero - self
    }

    pub fn magnitude(&self) -> f64 {
        if self.w == W::Point {
            panic!("magnitude is only for vectors")
        }
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn is_unit(&self) -> bool {
        (self.magnitude() - 1.0).abs() < 0.0001
    }

    pub fn normalize(self) -> Tuple {
        if self.w == W::Point {
            panic!("normalisation is only for vectors")
        }
        let magnitude = self.magnitude();
        self / magnitude
    }

    pub fn dot_product(a: &Tuple, b: &Tuple) -> f64 {
        if (a.w == W::Point) || (b.w == W::Point) {
            panic!("dot product is only for vectors")
        }
        a.x * b.x + a.y * b.y + a.z * b.z
    }

    pub fn cross_product(a: &Tuple, b: &Tuple) -> Tuple {
        if (a.w == W::Point) || (b.w == W::Point) {
            panic!("cross product is only for vectors")
        }
        Tuple::new_vector(
            a.y * b.z - a.z * b.y,
            a.z * b.x - a.x * b.z,
            a.x * b.y - a.y * b.x,
        )
    }
}

impl PartialEq for Tuple {
    fn eq(&self, other: &Self) -> bool {
        compare_float(self.x, other.x)
            && compare_float(self.y, other.y)
            && compare_float(self.z, other.z)
            && self.w == other.w
    }
}

impl Add for Tuple {
    type Output = Tuple;

    fn add(self, other: Tuple) -> Tuple {
        Tuple {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w,
        }
    }
}

impl Sub for Tuple {
    type Output = Tuple;

    fn sub(self, other: Tuple) -> Tuple {
        Tuple {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
            w: self.w - other.w,
        }
    }
}

impl Mul<f64> for Tuple {
    type Output = Tuple;

    fn mul(self, scalar: f64) -> Tuple {
        Tuple {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
            w: self.w,
        }
    }
}

impl Div<f64> for Tuple {
    type Output = Tuple;

    fn div(self, scalar: f64) -> Tuple {
        Tuple {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
            w: self.w,
        }
    }
}

#[cfg(test)]
mod tuple_tests {
    use super::*;

    #[test]
    fn tuple_creation() {
        let tuple = Tuple::new_tuple(4.3, -4.2, 3.1, 1);
        assert!(compare_float(tuple.x, 4.3));
        assert!(compare_float(tuple.y, -4.2));
        assert!(compare_float(tuple.z, 3.1));
        assert_eq!(tuple.w, W::Point);

        let tuple = Tuple::new_tuple(4.3, -4.2, 3.1, 0);
        assert!(compare_float(tuple.x, 4.3));
        assert!(compare_float(tuple.y, -4.2));
        assert!(compare_float(tuple.z, 3.1));
        assert_eq!(tuple.w, W::Vector);
    }

    #[test]
    fn point_creation() {
        let tuple = Tuple::new_point(4.0, -4.0, 3.0);
        assert!(compare_float(tuple.x, 4.0));
        assert!(compare_float(tuple.y, -4.0));
        assert!(compare_float(tuple.z, 3.0));
        assert_eq!(tuple.w, W::Point);
    }

    #[test]
    fn vector_creation() {
        let tuple = Tuple::new_vector(4.0, -4.0, 3.0);
        assert!(compare_float(tuple.x, 4.0));
        assert!(compare_float(tuple.y, -4.0));
        assert!(compare_float(tuple.z, 3.0));
        assert_eq!(tuple.w, W::Vector);
    }

    #[test]
    fn adding_tuples() {
        let a1 = Tuple::new_tuple(3.0, -2.0, 5.0, 1);
        let a2 = Tuple::new_tuple(-2.0, 3.0, 1.0, 0);
        let addition = a1 + a2;
        assert_eq!(addition, Tuple::new_tuple(1.0, 1.0, 6.0, 1));
    }

    #[test]
    fn substract_points() {
        let p1 = Tuple::new_point(3.0, 2.0, 1.0);
        let p2 = Tuple::new_point(5.0, 6.0, 7.0);
        let addition = p1 - p2;
        assert_eq!(addition, Tuple::new_vector(-2.0, -4.0, -6.0));
    }

    #[test]
    fn substract_vector_from_points() {
        let p = Tuple::new_point(3.0, 2.0, 1.0);
        let v = Tuple::new_vector(5.0, 6.0, 7.0);
        let addition = p - v;
        assert_eq!(addition, Tuple::new_point(-2.0, -4.0, -6.0));
    }

    #[test]
    fn substract_vectors() {
        let v1 = Tuple::new_vector(3.0, 2.0, 1.0);
        let v2 = Tuple::new_vector(5.0, 6.0, 7.0);
        let addition = v1 - v2;
        assert_eq!(addition, Tuple::new_vector(-2.0, -4.0, -6.0));
    }

    #[test]
    fn substract_zero_vectors() {
        let zero = Tuple::new_vector(0.0, 0.0, 0.0);
        let v = Tuple::new_vector(1.0, -2.0, 3.0);
        let addition = zero - v;
        assert_eq!(addition, Tuple::new_vector(-1.0, 2.0, -3.0));
    }

    #[test]
    fn negate_tuple() {
        let tuple = Tuple::new_tuple(1.0, -2.0, 3.0, 0);
        assert_eq!(tuple.negate(), Tuple::new_tuple(-1.0, 2.0, -3.0, 0));
    }

    #[test]
    fn multiplying_a_tuple_by_a_scalar() {
        let p = Tuple::new_point(3.0, 2.0, 1.0);
        let v = Tuple::new_vector(5.0, 6.0, 7.0);
        assert_eq!(p * 3.0, Tuple::new_point(9.0, 6.0, 3.0));
        assert_eq!(v * 2.0, Tuple::new_vector(10.0, 12.0, 14.0));
    }

    #[test]
    fn multiplying_a_tuple_by_a_fraction() {
        let p = Tuple::new_point(1.0, -2.0, 3.0);
        assert_eq!(p * 0.5, Tuple::new_point(0.5, -1.0, 1.5));
    }

    #[test]
    fn divise_a_tuple_by_a_scalar() {
        let p = Tuple::new_point(1.0, -2.0, 3.0);
        assert_eq!(p / 2.0, Tuple::new_point(0.5, -1.0, 1.5));
    }

    #[test]
    fn computing_magnitude() {
        let v = Tuple::new_vector(1.0, 0.0, 0.0);
        assert_eq!(v.magnitude(), 1.0);
        assert_eq!(v.is_unit(), true);

        let v1 = Tuple::new_vector(0.0, 0.0, 1.0);
        assert_eq!(v1.magnitude(), 1.0);
        assert_eq!(v1.is_unit(), true);

        let v2 = Tuple::new_vector(0.0, 1.0, 0.0);
        assert_eq!(v2.magnitude(), 1.0);
        assert_eq!(v2.is_unit(), true);

        let v3 = Tuple::new_vector(1.0, 2.0, 3.0);
        assert_eq!(v3.magnitude(), 14f64.sqrt());

        let v4 = Tuple::new_vector(-1.0, -2.0, -3.0);
        assert_eq!(v4.magnitude(), 14f64.sqrt());
    }

    #[test]
    fn nomalizing_vector() {
        let v = Tuple::new_vector(4.0, 0.0, 0.0);
        assert_eq!(v.magnitude(), 4.0);
        assert_eq!(true, v.normalize().is_unit());

        let v3 = Tuple::new_vector(1.0, 2.0, 3.0);
        assert_eq!(v3.magnitude(), 14f64.sqrt());
        let normalized_v3 = v3.normalize();
        assert_eq!(true, normalized_v3.is_unit());
    }

    #[test]
    fn cross_product() {
        let a = Tuple::new_vector(1.0, 2.0, 3.0);
        let b = Tuple::new_vector(2.0, 3.0, 4.0);
        let cross_ab = Tuple::new_vector(-1.0, 2.0, -1.0);
        assert_eq!(Tuple::cross_product(&a, &b), cross_ab);

        let cross_ba = Tuple::new_vector(1.0, -2.0, 1.0);
        assert_eq!(Tuple::cross_product(&b, &a), cross_ba);
    }

    #[test]
    fn dot_product() {
        let a = Tuple::new_vector(1.0, 2.0, 3.0);
        let b = Tuple::new_vector(2.0, 3.0, 4.0);
        let dot_ab = 20.0;
        assert_eq!(Tuple::dot_product(&a, &b), dot_ab);
    }
}
