const EPSILON: f64 = 0.00001;

pub fn is_equal_f64(a: f64, b: f64) -> bool {
    (a - b).abs() < EPSILON
}