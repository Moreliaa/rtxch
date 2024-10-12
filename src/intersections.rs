use crate::utils::EPSILON;
use crate::Ray;
use crate::Tuples;
use std::rc::Rc;
use std::cell::RefCell;
use crate::Shape;

#[derive(Debug, Clone)]
pub struct IntersectionList {
    count: usize,
    xs: Vec<Intersection>,
}

impl IntersectionList {
    pub fn create_empty() -> IntersectionList {
        IntersectionList { xs: vec![], count: 0 }
    }

    pub fn new(t: Vec<f64>, obj: &Rc<RefCell<dyn Shape>>) -> IntersectionList {
        let count = t.len();
        let xs = t.into_iter().map(|v| {
            Intersection::new(v, &obj)
        }).collect();
        IntersectionList { xs, count }
    }

    pub fn merge(mut l1: IntersectionList, mut l2: IntersectionList) -> IntersectionList {
        l1.xs.append(&mut l2.xs);
        l1.count = l1.xs.len();
        l1.xs.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
        l1
    }

    pub fn intersections(i1: Intersection, i2: Intersection) -> IntersectionList {
        IntersectionList::intersections_from_vec(vec![i1, i2])
    }

    pub fn intersections_from_vec(mut i: Vec<Intersection>) -> IntersectionList {
        // sort from lowest to highest t
        i.sort_by(|a, b| a.t().partial_cmp(&b.t()).unwrap());
        let count = i.len();
        let xs = i.into_iter().map(|val| val).collect();
        IntersectionList { xs, count }
    }

    pub fn hit(il: &IntersectionList) -> Option<&Intersection> {
        for i in il.xs() {
            if i.t() >= 0.0 {
                return Some(i);
            }
        }
        None
    }

    pub fn xs(&self) -> &Vec<Intersection> {
        &self.xs
    }

    pub fn xs_mut(&mut self) -> &mut Vec<Intersection> {
        &mut self.xs
    }
    
    pub fn count(&self) -> usize {
        self.count
    }
}

#[derive(Debug, Clone)]
pub struct Intersection {
    t: f64,
    object: Rc<RefCell<dyn Shape>>,
}

#[derive(Debug)]
pub struct Computations {
    pub t: f64,
    pub object: Rc<RefCell<dyn Shape>>,
    pub point: Tuples,
    pub eye_v: Tuples,
    pub normal_v: Tuples,
    pub inside: bool,
    pub over_point: Tuples,
}

impl Intersection {
    pub fn new(t: f64, object: &Rc<RefCell<dyn Shape>>) -> Intersection {
        Intersection { t, object: Rc::clone(object) }
    }

    pub fn prep_computations(i: &Intersection, r: &Ray) -> Computations {
        let p = Ray::position(r,i.t());
        let mut normal_v = <dyn Shape>::normal_at(&i.object(), &p);
        let eye_v = r.direction().clone().negate();
        let inside = if Tuples::dot(&eye_v, &normal_v) < 0.0 { true } else { false };
        if inside {
            normal_v.negate();
        }
        let normal_v_offset = normal_v.clone().scale(EPSILON);
        let over_point = p.clone().add(&normal_v_offset);
        Computations {
            t: i.t(),
            object: Rc::clone(i.object()),
            point: p,
            eye_v: eye_v,
            normal_v: normal_v,
            inside: inside,
            over_point,
        }
    }

    pub fn t(&self) -> f64 {
        self.t
    }

    pub fn object(&self) -> &Rc<RefCell<dyn Shape>> {
        &self.object
    }

    pub fn is_equal(&self, other: &Intersection) -> bool {
        if self.t() != other.t() {
            return false;
        }
        Rc::ptr_eq(self.object(), other.object())
    }
}