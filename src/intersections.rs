use crate::Ray;
use std::rc::Rc;

#[derive(Debug)]
pub struct IntersectionList {
    count: usize,
    xs: Vec<f64>,
}

impl IntersectionList {
    pub fn new(xs: Vec<f64>) -> IntersectionList {
        let count = xs.len();
        IntersectionList { xs, count }
    }

    pub fn xs(&self) -> &Vec<f64> {
        &self.xs
    }
    
    pub fn count(&self) -> usize {
        self.count
    }
}

#[derive(Debug)]
pub struct Intersection<T: Shape> {
    t: f64,
    object: Rc<T>,
}

impl<T: Shape> Intersection<T> {
    pub fn new(t: f64, object: &Rc<T>) -> Intersection<T> {
        Intersection { t, object: Rc::clone(object) }
    }

    pub fn t(&self) -> f64 {
        self.t
    }

    pub fn object(&self) -> &Rc<T> {
        &self.object
    }
}

pub trait Shape {
    fn intersect(&self, r: &Ray) -> IntersectionList;
}