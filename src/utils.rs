pub fn compare_float(value1: f64, value2: f64) -> bool {
    (value1 - value2).abs() < f64::EPSILON
}

pub fn compare_float_with_threshold(value1: f64, value2: f64, threshold : f64) -> bool {
    (value1 - value2).abs() < threshold
}