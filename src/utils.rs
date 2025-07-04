pub fn compare_float(value1: f64, value2: f64) -> bool {
    (value1 - value2).abs() < 0.00001
}

pub fn compare_float_with_threshold(value1: f64, value2: f64, threshold : f64) -> bool {
    (value1 - value2).abs() < threshold
}

pub fn pos_from_index(index: usize,width : usize) -> (usize, usize) {
    let y = index / width;
    let x = index % width;
    (x, y)
}

pub fn index_from_pos(x: usize, y: usize,width : usize) -> usize {
    (y * width) + x
}