use std::fmt::Debug;
use crate::Tuples;
use crate::Shape;
use crate::Matrix;
use std::rc::Rc;
use std::cell::RefCell;

pub trait Pattern: Debug {
    fn color_a(&self) -> &Tuples;
    fn color_b(&self) -> &Tuples;
    fn color_at(&self, point: &Tuples) -> Tuples;
    fn color_at_object(&self, object: &Rc<RefCell<dyn Shape>>, point_world: &Tuples) -> Tuples {
        let point_object = object.borrow().get_transform_inverse() * point_world;
        let point_pattern = self.get_transform_inverse() * &point_object;
        self.color_at(&point_pattern)
    }
    fn get_transform(&self) -> &Matrix;
    fn get_transform_inverse(&self) -> &Matrix;
    fn set_transform(&mut self, mat: Matrix);
}

#[derive(Debug, Clone)]
pub struct SingleColorPattern {
    pub color: Tuples,
    transform: Matrix,
    transform_inverse: Matrix,
}

impl SingleColorPattern {
    pub fn new(color: Tuples) -> Rc<RefCell<SingleColorPattern>> {
        Rc::new(RefCell::new(SingleColorPattern { color, transform: Matrix::new(4), transform_inverse: Matrix::new(4) }))
    }
}

impl Pattern for SingleColorPattern {
    fn color_a(&self) -> &Tuples {
        &self.color
    }
    fn color_b(&self) -> &Tuples {
        &self.color
    }
    fn color_at(&self, _: &Tuples) -> Tuples {
        self.color.clone()
    }
    fn get_transform(&self) -> &Matrix {
        &self.transform
    }
    fn get_transform_inverse(&self) -> &Matrix {
        &self.transform_inverse
    }
    fn set_transform(&mut self, mat: Matrix) {
        self.transform = mat;
        self.transform_inverse = Matrix::inverse(&self.transform).unwrap();
    }
}

#[derive(Debug, Clone)]
pub struct StripePattern {
    pub a: Tuples,
    pub b: Tuples,
    transform: Matrix,
    transform_inverse: Matrix,
}

impl StripePattern {
    pub fn new(a: Tuples, b: Tuples) -> Rc<RefCell<StripePattern>> {
        Rc::new(RefCell::new(StripePattern { a, b, transform: Matrix::new(4), transform_inverse: Matrix::new(4) }))
    }
}

impl Pattern for StripePattern {
    fn color_a(&self) -> &Tuples {
        &self.a
    }
    fn color_b(&self) -> &Tuples {
        &self.b
    }
    fn color_at(&self, point: &Tuples) -> Tuples {
        if point.x.floor() as i32 % 2 == 0 {
            self.a.clone()
        } else {
            self.b.clone()
        }
    }
    fn get_transform(&self) -> &Matrix {
        &self.transform
    }
    fn get_transform_inverse(&self) -> &Matrix {
        &self.transform_inverse
    }
    fn set_transform(&mut self, mat: Matrix) {
        self.transform = mat;
        self.transform_inverse = Matrix::inverse(&self.transform).unwrap();
    }
}

#[derive(Debug, Clone)]
pub struct GradientPattern {
    pub a: Tuples,
    pub b: Tuples,
    color_distance: Tuples,
    transform: Matrix,
    transform_inverse: Matrix,
}

impl GradientPattern {
    pub fn new(a: Tuples, b: Tuples) -> Rc<RefCell<GradientPattern>> {
        let color_distance = b.clone().subtract(&a);
        Rc::new(RefCell::new(GradientPattern { a, b, color_distance, transform: Matrix::new(4), transform_inverse: Matrix::new(4) }))
    }
}

impl Pattern for GradientPattern {
    fn color_a(&self) -> &Tuples {
        &self.a
    }
    fn color_b(&self) -> &Tuples {
        &self.b
    }
    fn color_at(&self, point: &Tuples) -> Tuples {
        let x = point.x.abs();
        let f = x - x.floor();
        let grad = self.color_distance.clone().scale(f);
        self.a.clone().add(&grad)
    }
    fn get_transform(&self) -> &Matrix {
        &self.transform
    }
    fn get_transform_inverse(&self) -> &Matrix {
        &self.transform_inverse
    }
    fn set_transform(&mut self, mat: Matrix) {
        self.transform = mat;
        self.transform_inverse = Matrix::inverse(&self.transform).unwrap();
    }
}