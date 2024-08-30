pub trait Matrix {
    fn new() -> Self;
    fn from_values(values: &Vec<f64>) -> Self;
    fn get(row: usize, col: usize) -> f64;
    fn mul(out: &mut Self, a: &Self, b: &Self) -> Self;
}