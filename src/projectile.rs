use crate::tuples::Tuples;
use crate::environment::Environment;

pub struct Projectile {
    pub position: Tuples,
    pub velocity: Tuples,
}

impl Projectile {
    pub fn new() -> Projectile {
        Projectile {
            position: Tuples::point(0.0, 0.0, 0.0),
            velocity: Tuples::vector(0.0, 0.0, 0.0),
        }
    }

    pub fn from_values(x_pos: f64, y_pos: f64, z_pos: f64, x_dir: f64, y_dir: f64, z_dir: f64) -> Projectile {
        Projectile {
            position: Tuples::point(x_pos,y_pos,z_pos),
            velocity: Tuples::vector(x_dir,y_dir,z_dir),
        }
    }

    pub fn from_tuples(position: Tuples, direction: Tuples) -> Projectile {
        Projectile {
            position,
            velocity: direction,
        }
    }

    pub fn tick(&mut self, environment: &Environment) -> &Projectile {
        self.position.add(&self.velocity);
        self.velocity.add(&environment.gravity);
        self.velocity.add(&environment.wind);
        self
    }
}