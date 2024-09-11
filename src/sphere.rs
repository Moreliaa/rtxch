use crate::intersections::{Intersectable, Intersections};
use crate::Ray;

#[derive(Debug)]
pub struct Sphere {

}

impl Sphere {
    pub fn new() -> Sphere {
        Sphere {}
    }
}

impl Intersectable for Sphere {
    fn intersect(&self, r: &Ray) -> Intersections {
        let mut hits: Vec<f64> = vec![];
        Intersections::new(hits)
    }
}