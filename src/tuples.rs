#[derive(Debug, Default)]
pub struct Tuples { 
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64
}

impl Tuples {
    pub fn new (x: f64, y: f64, z: f64, w: f64) -> Tuples {
        Tuples {x, y, z, w}
    }

    pub fn point(x: f64, y: f64, z: f64) -> Tuples {
        Tuples {x, y, z, w: 1.0}
    }

    pub fn vector(x: f64, y: f64, z: f64) -> Tuples {
        Tuples {x, y, z, w: 0.0}
    }

    pub fn is_point(&self) -> bool {
        self.w == 1.0
    }

    pub fn is_vector(&self) -> bool {
        self.w == 0.0
    }
}