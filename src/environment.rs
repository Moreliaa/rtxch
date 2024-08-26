use crate::tuples::Tuples;

pub struct Environment {
    pub gravity: Tuples,
    pub wind: Tuples,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            gravity: Tuples::vector(0.0, 0.0, 0.0),
            wind: Tuples::vector(0.0, 0.0, 0.0),
        }
    }

    pub fn from_values(x_grav: f64, y_grav: f64, z_grav: f64, x_wind: f64, y_wind: f64, z_wind: f64) -> Environment {
        Environment {
            gravity: Tuples::vector(x_grav,y_grav,z_grav),
            wind: Tuples::vector(x_wind,y_wind,z_wind),
        }
    }
}