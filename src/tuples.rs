use crate::utils::is_equal_f64;

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

    pub fn is_equal(&self, other: &Tuples) -> bool {
        is_equal_f64(self.x, other.x) &&
        is_equal_f64(self.y, other.y) &&
        is_equal_f64(self.z, other.z) &&
        is_equal_f64(self.w, other.w)
    }

    pub fn add(&mut self, other: &Tuples) -> &Tuples {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
        self.w += other.w;
        self
    }

    pub fn subtract(&mut self, other: &Tuples) -> &Tuples {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
        self.w -= other.w;
        self
    }
}