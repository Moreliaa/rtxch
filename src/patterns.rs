use std::fmt::Debug;
use crate::Tuples;
use crate::Shape;
use crate::Matrix;
use std::rc::Rc;
use std::cell::RefCell;

pub trait Pattern: Debug {
    fn color_a(&self) -> &Tuples;
    fn color_b(&self) -> &Tuples;
    fn color_at(&self, point: &Tuples) -> &Tuples;
    fn color_at_object(&self, object: &Rc<RefCell<dyn Shape>>, point: &Tuples) -> &Tuples;
    fn get_transform(&self) -> &Matrix;
    fn set_transform(&mut self, mat: Matrix);
}

#[derive(Debug, Clone)]
pub struct SingleColorPattern {
    pub color: Tuples,
    transform: Matrix,
}

impl SingleColorPattern {
    pub fn new(color: Tuples) -> Rc<RefCell<SingleColorPattern>> {
        Rc::new(RefCell::new(SingleColorPattern { color, transform: Matrix::new(4) }))
    }
}

impl Pattern for SingleColorPattern {
    fn color_a(&self) -> &Tuples {
        &self.color
    }
    fn color_b(&self) -> &Tuples {
        &self.color
    }
    fn color_at(&self, _: &Tuples) -> &Tuples {
        &self.color
    }   
    fn color_at_object(&self, _: &Rc<RefCell<dyn Shape>>, _: &Tuples) -> &Tuples {
        &self.color
    }
    fn get_transform(&self) -> &Matrix {
        &self.transform
    }
    fn set_transform(&mut self, mat: Matrix) {
        self.transform = mat;
    }
}

#[derive(Debug, Clone)]
pub struct StripePattern {
    pub a: Tuples,
    pub b: Tuples,
    transform: Matrix,
}

impl StripePattern {
    pub fn new(a: Tuples, b: Tuples) -> Rc<RefCell<StripePattern>> {
        Rc::new(RefCell::new(StripePattern { a, b, transform: Matrix::new(4) }))
    }
}

impl Pattern for StripePattern {
    fn color_a(&self) -> &Tuples {
        &self.a
    }
    fn color_b(&self) -> &Tuples {
        &self.b
    }
    fn color_at(&self, point: &Tuples) -> &Tuples {
        if point.x.floor() as i32 % 2 == 0 {
            &self.a
        } else {
            &self.b
        }
    }
    fn color_at_object(&self, object: &Rc<RefCell<dyn Shape>>, point: &Tuples) -> &Tuples {
        &self.b
    }
    fn get_transform(&self) -> &Matrix {
        &self.transform
    }
    fn set_transform(&mut self, mat: Matrix) {
        self.transform = mat;
    }
}