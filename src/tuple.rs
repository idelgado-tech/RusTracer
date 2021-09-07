
#[derive(PartialEq,Debug)]
enum W {
    POINT,
    VECTOR,
}


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
}
