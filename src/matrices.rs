pub trait Matrix {
    fn new() -> Self;
    fn from_values(values: &Vec<f64>) -> Self;
    fn identity(out: Self) -> Self;
    fn set(&mut self, row: usize, col: usize, val: f64);
    fn get(&self, row: usize, col: usize) -> f64;
    fn mul(out: Self, a: &Self, b: &Self) -> Self;
    fn dim() -> usize;
}

#[derive(Debug)]
pub struct Mat2 {
    m: [f64; 4],
}

impl Default for Mat2 {
    fn default() -> Self {
        Mat2::new()
    }
}


pub struct Mat3 {
    m: [f64; 9]
}

pub struct Mat4 {
    m: [f64; 16]
}

impl Matrix for Mat2 {
    fn new() -> Mat2 {
        let out = Mat2 { m: [0.0; 4]};
        Mat2::identity(out)
    }
    fn from_values(values: &Vec<f64>) -> Mat2 {
        from_values::<Mat2>(values)
    }
    fn identity(out: Self) -> Self {
        identity(out)
    }
    fn set(&mut self, row: usize, col: usize, val: f64) {
        let step = 2;
        let idx = step * row + col;
        self.m[idx] = val;
    }
    fn get(&self, row: usize, col: usize) -> f64 {
        let step = 2;
        let idx = step * row + col;
        self.m[idx]
    }
    fn mul(out: Mat2, a: &Mat2, b: &Mat2) -> Mat2 {
        out
    }
    fn dim() -> usize {
        2
    }
}

fn from_values<T>(values: &Vec<f64>) -> T
where
    T: Matrix,
{
    let mut m = T::new();
    let dim = T::dim();
    for (i, v) in values.iter().enumerate() {
        let col = i % dim;
        let row = i / dim;
        m.set(row, col, *v);
    }
    m
}


fn identity<T>(mut out: T) -> T 
where
    T: Matrix,
{
    let dim = T::dim();
    for i in 0..dim*dim {
        let col = i % dim;
        let row = i / dim;
        let val = if row == col {1.0} else {0.0};
        out.set(row, col, val);
    }
    out
}

/*impl Matrix for Mat3 {
    fn new() -> Self;
    fn from_values(values: &Vec<f64>) -> Self;
    fn identity(out: Self) -> Self;
    fn get(row: usize, col: usize) -> f64;
    fn mul(out: &mut Self, a: &Self, b: &Self) -> Self;
}*/

/*impl Matrix for Mat4 {
    fn new() -> Self;
    fn from_values(values: &Vec<f64>) -> Self;
    fn get(row: usize, col: usize) -> f64;
    fn mul(out: &mut Self, a: &Self, b: &Self) -> Self;   
}*/