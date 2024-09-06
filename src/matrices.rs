use std::fmt::Debug;

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
        Self::new()
    }
}

impl Matrix for Mat2 {
    fn new() -> Self {
        let out = Self { m: [0.0; 4]};
        Self::identity(out)
    }
    fn from_values(values: &Vec<f64>) -> Self {
        from_values(values)
    }
    fn identity(out: Self) -> Self {
        identity(out)
    }
    fn set(&mut self, row: usize, col: usize, val: f64) {
        self.m[get_idx::<Self>(row, col)] = val;
    }
    fn get(&self, row: usize, col: usize) -> f64 {
        self.m[get_idx::<Self>(row, col)]
    }
    fn mul(out: Self, a: &Self, b: &Self) -> Self {
        out
    }
    fn dim() -> usize {
        2
    }
}

#[derive(Debug)]
pub struct Mat3 {
    m: [f64; 9]
}

impl Default for Mat3 {
    fn default() -> Self {
        Self::new()
    }
}

impl Matrix for Mat3 {
    fn new() -> Self {
        let out = Self { m: [0.0; 9]};
        Self::identity(out)
    }
    fn from_values(values: &Vec<f64>) -> Self {
        from_values(values)
    }
    fn identity(out: Self) -> Self {
        identity(out)
    }
    fn set(&mut self, row: usize, col: usize, val: f64) {
        self.m[get_idx::<Self>(row, col)] = val;
    }
    fn get(&self, row: usize, col: usize) -> f64 {
        self.m[get_idx::<Self>(row, col)]
    }
    fn mul(out: Self, a: &Self, b: &Self) -> Self {
        out
    }
    fn dim() -> usize {
        3
    }
}

#[derive(Debug)]
pub struct Mat4 {
    m: [f64; 16]
}

impl Default for Mat4 {
    fn default() -> Self {
        Self::new()
    }
}

impl Matrix for Mat4 {
    fn new() -> Self {
        let out = Self { m: [0.0; 16]};
        Self::identity(out)
    }
    fn from_values(values: &Vec<f64>) -> Self {
        from_values(values)
    }
    fn identity(out: Self) -> Self {
        identity(out)
    }
    fn set(&mut self, row: usize, col: usize, val: f64) {
        self.m[get_idx::<Self>(row, col)] = val;
    }
    fn get(&self, row: usize, col: usize) -> f64 {
        self.m[get_idx::<Self>(row, col)]
    }
    fn mul(out: Self, a: &Self, b: &Self) -> Self {
        out
    }
    fn dim() -> usize {
        4
    }
}

fn get_idx<T>(row: usize, col: usize) -> usize
where
    T: Matrix,
{
    T::dim() * row + col
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