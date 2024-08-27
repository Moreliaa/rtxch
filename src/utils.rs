const EPSILON: f64 = 0.00001;

pub fn is_equal_f64(a: f64, b: f64) -> bool {
    (a - b).abs() < EPSILON
}

pub fn parse_values_f64(input: &String) -> Vec<f64> {
    input.split(", ").into_iter().map(|m| m.parse::<f64>().unwrap()).collect()
}

pub fn parse_values_u64(input: &String) -> Vec<u64> {
    input.split(", ").map(|m| m.parse::<u64>().unwrap()).collect()
}

pub fn parse_values_usize(input: &String) -> Vec<usize> {
    input.split(", ").map(|m| m.parse::<usize>().unwrap()).collect()
}