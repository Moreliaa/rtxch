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
}