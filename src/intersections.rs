use crate::Ray;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct IntersectionList<T: Shape> {
    count: usize,
    xs: Vec<Intersection<T>>,
}

impl<T: Shape> IntersectionList<T> {
    pub fn new(t: Vec<f64>, obj: &Rc<T>) -> IntersectionList<T> {
        let count = t.len();
        let xs = t.into_iter().map(|v| {
            Intersection::new(v, &obj)
        }).collect();
        IntersectionList { xs, count }
    }

    pub fn intersections(i1: Intersection<T>, i2: Intersection<T>) -> IntersectionList<T> {
        let xs = vec![i1, i2];
        let count = xs.len();
        IntersectionList { xs, count }
    }

    pub fn xs(&self) -> &Vec<Intersection<T>> {
        &self.xs
    }
    
    pub fn count(&self) -> usize {
        self.count
    }
}

#[derive(Debug, Clone)]
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
    fn intersect(this: &Rc<Self>, r: &Ray) -> IntersectionList<Self> where Self: Sized;
}