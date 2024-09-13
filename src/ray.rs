use crate::Tuples;
use crate::Sphere;
use crate::intersections::IntersectionList;
use std::rc::Rc;

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

    pub fn intersect(s: &Rc<Sphere>, r: &Ray) -> IntersectionList<Sphere> {
        let xs = vec![0.0; 0];
        IntersectionList::new(xs, s)
    }
}