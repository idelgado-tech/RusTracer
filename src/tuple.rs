use std::ops::Add;
use std::ops::Sub;

#[derive(PartialEq, Debug)]
enum W {
    POINT,
    VECTOR,
}

impl Add for W {
    type Output = W;

    fn add(self, other: W) -> W {
        match (self, other) {
            (W::POINT, W::VECTOR) => W::POINT,
            (W::VECTOR, W::POINT) => W::POINT,
            (W::VECTOR, W::VECTOR) => W::VECTOR,
            (_, _) => panic!("W ADD , case not supported"),
        }
    }
}

impl Sub for W {
    type Output = W;

    fn sub(self, other: W) -> W {
        match (self, other) {
            (W::POINT, W::VECTOR) => W::POINT,
            (W::VECTOR, W::VECTOR) => W::VECTOR,
            (W::POINT, W::POINT) => W::VECTOR,
            (_, _) => panic!("W ADD , case not supported"),
        }
    }
}

#[derive(PartialEq, Debug)]
struct Tuple {
    x: f64,
    y: f64,
    z: f64,
    w: W,
}

impl Tuple {
    fn new_point(x: f64, y: f64, z: f64) -> Tuple {
        Tuple {
            x,
            y,
            z,
            w: W::POINT,
        }
    }

    fn new_vector(x: f64, y: f64, z: f64) -> Tuple {
        Tuple {
            x,
            y,
            z,
            w: W::VECTOR,
        }
    }

    fn new_tuple(x: f64, y: f64, z: f64, w: i64) -> Tuple {
        Tuple {
            x,
            y,
            z,
            w: match w {
                0 => W::VECTOR,
                1 => W::POINT,
                _ => panic!("Tuple must be point OR vector, input w value : {}", w),
            },
        }
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

fn compare_float(value1: f64, value2: f64) -> bool {
    (value1 - value2).abs() < f64::EPSILON
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tuple_creation() {
        let tuple = Tuple::new_tuple(4.3, -4.2, 3.1, 1);
        assert!(compare_float(tuple.x, 4.3));
        assert!(compare_float(tuple.y, -4.2));
        assert!(compare_float(tuple.z, 3.1));
        assert_eq!(tuple.w, W::POINT);

        let tuple = Tuple::new_tuple(4.3, -4.2, 3.1, 0);
        assert!(compare_float(tuple.x, 4.3));
        assert!(compare_float(tuple.y, -4.2));
        assert!(compare_float(tuple.z, 3.1));
        assert_eq!(tuple.w, W::VECTOR);
    }

    #[test]
    fn point_creation() {
        let tuple = Tuple::new_point(4.0, -4.0, 3.0);
        assert!(compare_float(tuple.x, 4.0));
        assert!(compare_float(tuple.y, -4.0));
        assert!(compare_float(tuple.z, 3.0));
        assert_eq!(tuple.w, W::POINT);
    }

    #[test]
    fn vector_creation() {
        let tuple = Tuple::new_vector(4.0, -4.0, 3.0);
        assert!(compare_float(tuple.x, 4.0));
        assert!(compare_float(tuple.y, -4.0));
        assert!(compare_float(tuple.z, 3.0));
        assert_eq!(tuple.w, W::VECTOR);
    }

    #[test]
    fn adding_tuples() {
        let a1 = Tuple::new_tuple(3.0, -2.0, 5.0, 1);
        let a2 = Tuple::new_tuple(-2.0, 3.0, 1.0, 0);
        let addition = a1 + a2;
        assert_eq!(addition, Tuple::new_tuple(1.0, 1.0, 6.0, 1));
    }

}
