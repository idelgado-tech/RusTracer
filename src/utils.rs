pub fn compare_float(value1: f64, value2: f64) -> bool {
    (value1 - value2).abs() < f64::EPSILON
}