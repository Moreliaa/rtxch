use crate::Matrix;
use std::rc::Rc;
use std::cell::RefCell;
use crate::IntersectionList;
use crate::Material;
use crate::Tuples;
use crate::Ray;
use std::fmt::Debug;

pub trait Shape: Debug {
    fn intersect_local(&self, r: &Ray) -> Vec<f64>;
    fn set_transform(&mut self, transform: &Matrix);
    fn get_transform(&self) -> &Matrix;
    fn get_transform_inverse(&self) -> &Matrix;
    fn set_material(&mut self, material: &Material);
    fn get_material(&self) -> &Material;
    fn get_mut_material(&mut self) -> &mut Material;
    fn normal_at_local(&self, p: &Tuples) -> Tuples;
}

impl dyn Shape {
    pub fn intersect(s: &Rc<RefCell<dyn Shape>>, r: &Ray) -> IntersectionList {
        let r = Ray::transform(r, s.borrow().get_transform_inverse());
        let t = s.borrow().intersect_local(&r);
        IntersectionList::new(t, &s)
    }

    pub fn normal_at(s: &Rc<RefCell<dyn Shape>>, p: &Tuples) -> Tuples {
        s.borrow().normal_at_local(p)
    }

    pub fn is_equal(a: &Rc<RefCell<dyn Shape>>, b: &Rc<RefCell<dyn Shape>>) -> bool {
        a.borrow().get_material().is_equal(&b.borrow().get_material()) &&
        a.borrow().get_transform().is_equal(&b.borrow().get_transform())
    }
}