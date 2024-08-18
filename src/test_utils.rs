pub fn is_equal_f64(a: f64, b: f64) -> bool {
    let epsilon = 0.00001;
    (a - b).abs() < epsilon
}