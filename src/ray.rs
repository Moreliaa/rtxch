use crate::Tuples;

#[derive(Debug)]
pub struct Ray {
    origin: Tuples,
    direction: Tuples,
}

impl Ray {
    pub fn new(origin: Tuples, direction: Tuples) -> Ray {
        Ray { origin, direction }
    }

    pub fn position(r: &Ray, time: f64) -> Tuples {
        Tuples::point(0.0,0.0,0.0)
    }

    pub fn origin(&self) -> &Tuples {
        &self.origin
    }

    pub fn direction(&self) -> &Tuples {
        &self.direction
    }
}