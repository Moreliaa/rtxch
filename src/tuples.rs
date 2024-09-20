use crate::utils::is_equal_f64;


#[derive(Debug, Copy, Clone, PartialEq)]
enum TuplesEnum {
    Tuple, Point, Vector, Color
}

impl Default for TuplesEnum {
    fn default() -> Self {
        TuplesEnum::Tuple
    }
}

#[derive(Debug, Copy, Clone, Default, PartialEq)]
pub struct Tuples {
    kind: TuplesEnum, 
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64
}

// Static
impl Tuples {
    pub fn new (x: f64, y: f64, z: f64, w: f64) -> Tuples {
        Tuples {kind: TuplesEnum::Tuple, x, y, z, w}
    }

    pub fn point(x: f64, y: f64, z: f64) -> Tuples {
        Tuples {kind: TuplesEnum::Point, x, y, z, w: 1.0}
    }

    pub fn vector(x: f64, y: f64, z: f64) -> Tuples {
        Tuples {kind: TuplesEnum::Vector, x, y, z, w: 0.0}
    }

    pub fn color(x: f64, y: f64, z: f64) -> Tuples {
        Tuples {kind: TuplesEnum::Color, x, y, z, w: 0.0}
    }

    pub fn dot(v1: &Tuples, v2: &Tuples) -> f64 {
        v1.x * v2.x + v1.y * v2.y + v1.z * v2.z + v1.w * v2.w 
    }

    pub fn cross(v1: &Tuples, v2: &Tuples) -> Tuples {
        Tuples::vector(
            v1.y * v2.z - v1.z * v2.y,
            v1.z * v2.x - v1.x * v2.z,
            v1.x * v2.y - v1.y * v2.x,
        )
    }

    pub fn reflect(v_in: &Tuples, normal: &Tuples) -> Tuples {
        let dot = Tuples::dot(v_in, normal);
        v_in.clone().subtract(&normal.clone().scale(2.0 * dot))
    }
}

// Methods
impl Tuples {
    pub fn is_point(&self) -> bool {
        self.kind == TuplesEnum::Point
    }

    pub fn is_vector(&self) -> bool {
        self.kind == TuplesEnum::Vector
    }

    pub fn is_equal(&self, other: &Tuples) -> bool {
        is_equal_f64(self.x, other.x) &&
        is_equal_f64(self.y, other.y) &&
        is_equal_f64(self.z, other.z) &&
        is_equal_f64(self.w, other.w)
    }

    pub fn get_at_idx(&self, idx: usize) -> f64 {
        match idx {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            3 => self.w,
            _ => panic!("Index out of bounds"),
        }
    }

    pub fn set_at_idx(&mut self, idx: usize, val: f64) {
        match idx {
            0 => self.x = val,
            1 => self.y = val,
            2 => self.z = val,
            3 => self.w = val,
            _ => panic!("Index out of bounds"),
        };
    }

    pub fn add(&mut self, other: &Tuples) -> Tuples {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
        self.w += other.w;
        *self
    }

    pub fn subtract(&mut self, other: &Tuples) -> Tuples {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
        self.w -= other.w;
        *self
    }

    pub fn negate(&mut self) -> Tuples {
        self.x = -self.x;
        self.y = -self.y;
        self.z = -self.z;
        self.w = -self.w;
        *self
    }

    pub fn multiply(&mut self, other: &Tuples) -> Tuples {
        self.x *= other.x;
        self.y *= other.y;
        self.z *= other.z;
        self.w *= other.w;
        *self
    }

    pub fn scale(&mut self, scalar: f64) -> Tuples {
        self.x *= scalar;
        self.y *= scalar;
        self.z *= scalar;
        self.w *= scalar;
        *self
    }

    pub fn magnitude(&mut self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt()
    }

    pub fn normalize(&mut self) -> Tuples {
        let mag = self.magnitude();
        self.x /= mag;
        self.y /= mag;
        self.z /= mag;
        self.w /= mag;
        *self
    }
}