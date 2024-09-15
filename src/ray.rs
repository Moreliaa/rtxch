use crate::Tuples;
use crate::Matrix;

#[derive(Debug, Clone)]
pub struct Ray {
    origin: Tuples,
    direction: Tuples,
}

impl Ray {
    pub fn new(origin: Tuples, direction: Tuples) -> Ray {
        Ray { origin, direction }
    }

    pub fn transform(r: &Ray, m: &Matrix) -> Ray {
        let origin = m * r.origin();
        let direction = m * r.direction();
        Ray {origin, direction }
    }

    pub fn position(r: &Ray, time: f64) -> Tuples {
        let mut scaled = r.direction().clone();
        scaled.scale(time);
        let mut out = r.origin().clone();
        out.add(&scaled);
        out
    }

    pub fn origin(&self) -> &Tuples {
        &self.origin
    }

    pub fn direction(&self) -> &Tuples {
        &self.direction
    }
}