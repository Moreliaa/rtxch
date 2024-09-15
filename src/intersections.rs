use crate::Ray;
use crate::Matrix;
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
        IntersectionList::intersections_from_vec(vec![i1, i2])
    }

    pub fn intersections_from_vec(mut i: Vec<Intersection<T>>) -> IntersectionList<T> {
        // sort from lowest to highest t
        i.sort_by(|a, b| a.t().partial_cmp(&b.t()).unwrap());
        let count = i.len();
        let xs = i.into_iter().map(|val| val).collect();
        IntersectionList { xs, count }
    }

    pub fn hit(il: &IntersectionList<T>) -> Option<&Intersection<T>> {
        for i in il.xs() {
            if i.t() >= 0.0 {
                return Some(i);
            }
        }
        None
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

    pub fn is_equal(&self, other: &Intersection<T>) -> bool {
        if self.t() != other.t() {
            return false;
        }
        Rc::ptr_eq(self.object(), other.object())
    }
}

pub trait Shape {
    fn intersect(this: &Rc<Self>, r: &Ray) -> IntersectionList<Self> where Self: Sized;
    fn set_transform(this: &Rc<Self>, transform: &Matrix);
}