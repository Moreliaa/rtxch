use crate::Tuples;
use crate::Sphere;
use crate::intersections::IntersectionList;

#[derive(Debug)]
pub struct Ray {
    origin: Tuples,
    direction: Tuples,
}

impl Ray {
    pub fn new(origin: Tuples, mut direction: Tuples) -> Ray {
        Ray { origin, direction: direction.normalize() }
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

    pub fn intersect(s: &Sphere, r: &Ray) -> IntersectionList {
        let xs = vec![0.0; 0];
        IntersectionList::new(xs)
    }
}