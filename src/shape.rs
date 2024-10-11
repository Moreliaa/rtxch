use crate::Matrix;
use std::rc::Rc;
use std::cell::RefCell;
use crate::IntersectionList;
use crate::Material;
use crate::Tuples;
use crate::Ray;

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