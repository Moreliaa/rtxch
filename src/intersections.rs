use crate::Ray;
use crate::Matrix;
use crate::Tuples;
use crate::Material;
use std::rc::Rc;
use std::cell::RefCell;
use crate::Sphere;

#[derive(Debug, Clone)]
pub struct IntersectionList<T: Shape> {
    count: usize,
    xs: Vec<Intersection<T>>,
}

impl<T: Shape> IntersectionList<T> {
    pub fn create_empty() -> IntersectionList<T> {
        IntersectionList { xs: vec![], count: 0 }
    }

    pub fn new(t: Vec<f64>, obj: &Rc<RefCell<T>>) -> IntersectionList<T> {
        let count = t.len();
        let xs = t.into_iter().map(|v| {
            Intersection::new(v, &obj)
        }).collect();
        IntersectionList { xs, count }
    }

    pub fn merge(mut l1: IntersectionList<T>, mut l2: IntersectionList<T>) -> IntersectionList<T> {
        l1.xs.append(&mut l2.xs);
        l1.count = l1.xs.len();
        l1.xs.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        l1
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

    pub fn xs_mut(&mut self) -> &mut Vec<Intersection<T>> {
        &mut self.xs
    }
    
    pub fn count(&self) -> usize {
        self.count
    }
}

#[derive(Debug, Clone)]
pub struct Intersection<T: Shape> {
    t: f64,
    object: Rc<RefCell<T>>,
}

#[derive(Debug)]
pub struct Computations<T: Shape> {
    pub t: f64,
    pub object: Rc<RefCell<T>>,
    pub point: Tuples,
    pub eye_v: Tuples,
    pub normal_v: Tuples,
    pub inside: bool,
}

impl<T: Shape> Intersection<T> {
    pub fn new(t: f64, object: &Rc<RefCell<T>>) -> Intersection<T> {
        Intersection { t, object: Rc::clone(object) }
    }

    pub fn prep_computations(i: &Intersection<T>, r: &Ray) -> Computations<T> {
        let p = Ray::position(r,i.t());
        let mut normal_v = T::normal_at(&i.object(), &p);
        let eye_v = r.direction().clone().negate();
        let inside = if Tuples::dot(&eye_v, &normal_v) < 0.0 { true } else { false };
        if inside {
            normal_v.negate();
        }
        Computations {
            t: i.t(),
            object: Rc::clone(i.object()),
            point: p,
            eye_v: eye_v,
            normal_v: normal_v,
            inside: inside,
        }
    }

    pub fn t(&self) -> f64 {
        self.t
    }

    pub fn object(&self) -> &Rc<RefCell<T>> {
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
    fn intersect(this: &Rc<RefCell<Self>>, r: &Ray) -> IntersectionList<Self> where Self: Sized;
    fn set_transform(this: &Rc<RefCell<Self>>, transform: &Matrix);
    fn get_transform(&self) -> &Matrix;
    fn get_transform_inverse(&self) -> &Matrix;
    fn set_material(this: &Rc<RefCell<Self>>, material: &Material);
    fn get_material(&self) -> &Material;
    fn get_mut_material(&mut self) -> &mut Material;
    fn normal_at(this: &Rc<RefCell<Self>>, p: &Tuples) -> Tuples;
}