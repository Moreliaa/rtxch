use std::fmt::Debug;

#[derive(Debug)]
pub struct Matrix {
    m: Vec<f64>,
    dim: usize,
}

impl Default for Matrix {
    fn default() -> Self {
        Self::new(4)
    }
}

impl Matrix {
    pub fn new(dim: usize) -> Self {
        let out = Self { m: vec![0.0; dim * dim], dim};
        Self::identity(out)
    }
    pub fn from_values(values: &Vec<f64>) -> Self {
        let dim = (values.len() as f64).sqrt() as usize;
        if dim * dim != values.len() {
            panic!("Matrix.from_values: invalid length {dim} in input: {:?}", values);
        }
        let mut m = Self::new(dim);
        for (i, v) in values.iter().enumerate() {
            let col = i % dim;
            let row = i / dim;
            m.set(row, col, *v);
        }
        m
    }
    pub fn identity(mut out: Self) -> Self {
        let dim = out.dim();
        for i in 0..dim*dim {
            let col = i % dim;
            let row = i / dim;
            let val = if row == col {1.0} else {0.0};
            out.set(row, col, val);
        }
        out
    }
    pub fn set(&mut self, row: usize, col: usize, val: f64) {
        let idx = self.get_idx(row, col);
        self.m[idx] = val;
    }
    pub fn get(&self, row: usize, col: usize) -> f64 {
        self.m[self.get_idx(row, col)]
    }
    pub fn mul(out: Self, a: &Self, b: &Self) -> Self {
        out
    }
    pub fn dim(&self) -> usize {
        self.dim
    }
    fn get_idx(&self, row: usize, col: usize) -> usize {
        self.dim() * row + col
    }
}