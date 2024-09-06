use std::fmt::Debug;
use crate::Tuples;

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
        let (mut row, mut col) = (0, 0);
        for (_, v) in values.iter().enumerate() {
            m.set(row, col, *v);
            col += 1;
            if col == dim {
                col = 0;
                row += 1;
            }
        }
        m
    }
    pub fn identity(mut out: Self) -> Self {
        let dim = out.dim();
        let (mut row, mut col) = (0, 0);
        for _ in 0..dim*dim {
            let val = if row == col {1.0} else {0.0};
            out.set(row, col, val);
            col += 1;
            if col == dim {
                col = 0;
                row += 1;
            }
        }
        out
    }
    pub fn mul(a: &Self, b: &Self) -> Self {
        if a.dim() != b.dim() {
            panic!("Matrix.mul: dimensions don't match");
        }
        let mut out = Matrix::new(a.dim());
        for col in 0..out.dim() {
            for row in 0..out.dim() {
                let mut val = 0.0;
                for idx in 0..out.dim() {
                    val += a.get(row, idx) * b.get(idx, col);
                }
                out.set(row, col, val);
            }
        }
        out
    }

    pub fn mul_tuple(a: &Self, b: &Tuples) -> Tuples {
        Tuples::default()
    }

    pub fn transpose(a: &Self) -> Self {
        Self::new(a.dim())
    }

    pub fn inverse(a: &Self) -> Self {
        Self::new(a.dim())
    }

    pub fn det(a: &Self) -> i32 {
        0
    }

    pub fn submatrix(a: &Self, row: usize, col: usize) -> Self {
        Self::new(a.dim())
    }

    pub fn minor(a: &Self) -> i32 {
        0
    }

    pub fn cofactor(a: &Self) -> i32 {
        0
    }
}

// Methods
impl Matrix {
    pub fn is_equal(&self, b: &Self) -> bool {
        self.m == b.m
    }

    pub fn set(&mut self, row: usize, col: usize, val: f64) {
        let idx = self.get_idx(row, col);
        self.m[idx] = val;
    }
    pub fn get(&self, row: usize, col: usize) -> f64 {
        self.m[self.get_idx(row, col)]
    }
    pub fn dim(&self) -> usize {
        self.dim
    }
    fn get_idx(&self, row: usize, col: usize) -> usize {
        self.dim() * row + col
    }
}