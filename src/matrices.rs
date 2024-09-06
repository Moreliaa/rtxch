use std::fmt::Debug;
use std::collections::HashMap;
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
        if a.dim() != 4 {
            panic!("Cannot use mul_tuple with dim {}", a.dim());
        }
        let mut out = Tuples::default();

        for row in 0..a.dim() {
            let mut val = 0.0;
            for col in 0..a.dim() {
                val += a.get(row, col) * b.get_at_idx(col);
            }
            out.set_at_idx(row, val);
        }
        out
    }

    pub fn transpose(a: &Self) -> Self {
        let mut out = Self::new(a.dim());
        for col in 0..out.dim() {
            for row in 0..out.dim() {
                out.set(col, row, a.get(row, col));
            }
        }
        out
    }

    pub fn inverse(a: &Self) -> Self {
        Self::new(a.dim())
    }

    pub fn det(a: &Self) -> f64 {
        if a.dim() == 2 {
            return a.get(0,0) * a.get(1, 1) - (a.get(0, 1) * a.get(1, 0));
        } else {
            let row = 0;
            let mut det = 0.0;
            for col in 0..a.dim() {
                let v = a.get(row, col);
                let cofactor = Matrix::cofactor(&a, row, col);
                det +=  v * cofactor;
            }
            return det;
        }
    }

    pub fn submatrix(a: &Self, row_to_remove: usize, col_to_remove: usize) -> Self {
        let mut out = Self::new(a.dim() - 1);
        let (mut row_new, mut col_new) = (0, 0);
        for col in 0..a.dim() {
            if col == col_to_remove {
                continue;
            }
            for row in 0..a.dim() {
                if row == row_to_remove {
                    continue;
                }

                out.set(row_new, col_new, a.get(row, col));
                row_new += 1;
            }
            col_new += 1;
            row_new = 0;
        }
        out
    }

    pub fn minor(a: &Self, row: usize, col: usize) -> f64 {
        let sm = Matrix::submatrix(a, row, col);
        Matrix::det(&sm)
    }

    pub fn cofactor(a: &Self, row: usize, col: usize) -> f64 {
        let minor = Matrix::minor(&a, row, col);
        if row + col % 2 == 0 { minor } else { -minor }
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