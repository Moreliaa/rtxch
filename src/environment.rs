use crate::tuples::Tuples;

pub struct Environment {
    pub gravity: Tuples,
    pub wind: Tuples,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            gravity: Tuples::point(0.0, 0.0, 0.0),
            wind: Tuples::vector(0.0, 0.0, 0.0),
        }
    }

    pub fn from_values(x_pos: f64, y_pos: f64, z_pos: f64, x_dir: f64, y_dir: f64, z_dir: f64) -> Environment {
        Environment {
            gravity: Tuples::point(x_pos,y_pos,z_pos),
            wind: Tuples::vector(x_dir,y_dir,z_dir),
        }
    }
}