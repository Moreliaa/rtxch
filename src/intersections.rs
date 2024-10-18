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
    pub under_point: Tuples,
    pub reflect_v: Tuples,
    pub n1: f64,
    pub n2: f64,
}

impl Intersection {
    pub fn new(t: f64, object: &Rc<RefCell<dyn Shape>>) -> Intersection {
        Intersection { t, object: Rc::clone(object) }
    }

    pub fn prep_computations(hit: &Intersection, r: &Ray, xs: &IntersectionList) -> Computations {
        let point = Ray::position(r,hit.t());
        let mut normal_v = <dyn Shape>::normal_at(&hit.object(), &point);
        let eye_v = r.direction().clone().negate().normalize();
        let inside = if Tuples::dot(&eye_v, &normal_v) < 0.0 { true } else { false };
        if inside {
            normal_v.negate();
        }
        let normal_v_offset = normal_v.clone().scale(EPSILON);
        let over_point = point.clone().add(&normal_v_offset);
        let under_point = point.clone().subtract(&normal_v_offset);
        let reflect_v = Tuples::reflect(r.direction(), &normal_v);

        // refractive indices
        let mut n1 = 1.0;
        let mut n2 = 1.0;

        let mut containers: Vec<Rc<RefCell<dyn Shape>>> = vec![];
        for i_xs in xs.xs() {
            let is_hit = i_xs.is_equal(hit);
            if is_hit {
                if containers.len() == 0 {
                    n1 = 1.0;
                } else {
                    let obj = containers.last().unwrap();
                    n1 = obj.borrow().get_material().refractive_index;
                }
            }
        
            if let Some(index) = containers.iter().position(|ci| Rc::ptr_eq(ci, i_xs.object())) {
                containers.splice(index..index + 1, vec![]); // exiting
            } else {
                containers.push(Rc::clone(i_xs.object())); // entering
            }

            if is_hit {
                if containers.len() == 0 {
                    n2 = 1.0;
                } else {
                    let obj = containers.last().unwrap();
                    n2 = obj.borrow().get_material().refractive_index;
                }
                break;
            }
        }

        Computations {
            t: hit.t(),
            object: Rc::clone(hit.object()),
            point,
            eye_v,
            normal_v,
            inside,
            over_point,
            under_point,
            reflect_v,
            n1,
            n2,
        }
    }

    pub fn t(&self) -> f64 {
        self.t
    }

    pub fn object(&self) -> &Rc<RefCell<dyn Shape>> {
        &self.object
    }

    pub fn is_equal(&self, other: &Intersection) -> bool {
        if !crate::utils::is_equal_f64(self.t(), other.t()) {
            return false;
        }
        Rc::ptr_eq(self.object(), other.object())
    }
}