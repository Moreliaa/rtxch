use std::fmt::Debug;
use crate::utils::perlin_noise;
use crate::Tuples;
use crate::Shape;
use crate::Matrix;
use std::rc::Rc;
use std::cell::RefCell;

pub trait Pattern: Debug {
    fn color_a(&self) -> &Tuples; // for testing only
    fn color_b(&self) -> &Tuples; // for testing only
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
pub struct TestPattern {
    color: Tuples,
    transform: Matrix,
    transform_inverse: Matrix,
}

impl TestPattern {
    pub fn new() -> Rc<RefCell<TestPattern>> {
        Rc::new(RefCell::new(TestPattern { color: Tuples::color(0.0,0.0,0.0), transform: Matrix::new(4), transform_inverse: Matrix::new(4) }))
    }
}

impl Pattern for TestPattern {
    fn color_a(&self) -> &Tuples {
        &self.color
    }
    fn color_b(&self) -> &Tuples {
        &self.color
    }
    fn color_at(&self, position: &Tuples) -> Tuples {
        Tuples::color(position.x,position.y,position.z)
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
        let floored = x.floor();
        let reverse = floored as i32 % 2 != 0;
        let mut f = x - floored;
        if reverse {
            f = 1.0 - f;
        }
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


#[derive(Debug, Clone)]
pub struct RingPattern {
    pub a: Tuples,
    pub b: Tuples,
    transform: Matrix,
    transform_inverse: Matrix,
}

impl RingPattern {
    pub fn new(a: Tuples, b: Tuples) -> Rc<RefCell<RingPattern>> {
        Rc::new(RefCell::new(RingPattern { a, b, transform: Matrix::new(4), transform_inverse: Matrix::new(4) }))
    }
}

impl Pattern for RingPattern {
    fn color_a(&self) -> &Tuples {
        &self.a
    }
    fn color_b(&self) -> &Tuples {
        &self.b
    }
    fn color_at(&self, point: &Tuples) -> Tuples {
        let dist = (point.x.powf(2.0) + point.z.powf(2.0)).sqrt().floor() as i32;
        if dist % 2 == 0 { self.a.clone() } else { self.b.clone() }
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
pub struct RadialGradientPattern {
    pub a: Tuples,
    pub b: Tuples,
    color_distance: Tuples,
    transform: Matrix,
    transform_inverse: Matrix,
}

impl RadialGradientPattern {
    pub fn new(a: Tuples, b: Tuples) -> Rc<RefCell<RadialGradientPattern>> {
        let color_distance = b.clone().subtract(&a);
        Rc::new(RefCell::new(RadialGradientPattern { a, b, color_distance, transform: Matrix::new(4), transform_inverse: Matrix::new(4) }))
    }
}

impl Pattern for RadialGradientPattern {
    fn color_a(&self) -> &Tuples {
        &self.a
    }
    fn color_b(&self) -> &Tuples {
        &self.b
    }
    fn color_at(&self, point: &Tuples) -> Tuples {
        let x = (point.x.powf(2.0) + point.z.powf(2.0)).sqrt();
        let floored = x.floor();
        let reverse = floored as i32 % 2 != 0;
        let mut f = x - floored;
        if reverse {
            f = 1.0 - f;
        }
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

#[derive(Debug, Clone)]
pub struct CheckersPattern {
    pub a: Tuples,
    pub b: Tuples,
    transform: Matrix,
    transform_inverse: Matrix,
}

impl CheckersPattern {
    pub fn new(a: Tuples, b: Tuples) -> Rc<RefCell<CheckersPattern>> {
        Rc::new(RefCell::new(CheckersPattern { a, b, transform: Matrix::new(4), transform_inverse: Matrix::new(4) }))
    }
}

impl Pattern for CheckersPattern {
    fn color_a(&self) -> &Tuples {
        &self.a
    }
    fn color_b(&self) -> &Tuples {
        &self.b
    }
    fn color_at(&self, point: &Tuples) -> Tuples {
        // should include point.y.floor() but breaks xz planes
        let dist = (point.x.floor() + point.z.floor()) as i32;
        if dist % 2 == 0 { self.a.clone() } else { self.b.clone() }
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
pub struct NestedCheckersPattern {
    pub a: Rc<RefCell<dyn Pattern>>,
    pub b: Rc<RefCell<dyn Pattern>>,
    test_color: Tuples,
    transform: Matrix,
    transform_inverse: Matrix,
}

impl NestedCheckersPattern {
    pub fn new(a: Rc<RefCell<dyn Pattern>>, b: Rc<RefCell<dyn Pattern>>) -> Rc<RefCell<NestedCheckersPattern>> {
        Rc::new(RefCell::new(NestedCheckersPattern {
            a: Rc::clone(&a),
            b: Rc::clone(&b),
            test_color: Tuples::color(0.0,0.0,0.0),
            transform: Matrix::new(4),
            transform_inverse: Matrix::new(4)
        }))
    }
}

impl Pattern for NestedCheckersPattern {
    fn color_a(&self) -> &Tuples {
        &self.test_color
    }
    fn color_b(&self) -> &Tuples {
        &self.test_color
    }
    fn color_at(&self, point: &Tuples) -> Tuples {
        // should include point.y.floor() but breaks xz planes
        let dist = (point.x.floor() + point.z.floor()) as i32;
        if dist % 2 == 0 {
            let point_sub_pattern = self.a.borrow().get_transform_inverse() * point;
            self.a.borrow().color_at(&point_sub_pattern).clone()
        } else {
            let point_sub_pattern = self.b.borrow().get_transform_inverse() * point;
            self.b.borrow().color_at(&point_sub_pattern).clone()
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
pub struct BlendedPattern {
    pub a: Rc<RefCell<dyn Pattern>>,
    pub b: Rc<RefCell<dyn Pattern>>,
    test_color: Tuples,
    transform: Matrix,
    transform_inverse: Matrix,
}

impl BlendedPattern {
    pub fn new(a: Rc<RefCell<dyn Pattern>>, b: Rc<RefCell<dyn Pattern>>) -> Rc<RefCell<BlendedPattern>> {
        Rc::new(RefCell::new(BlendedPattern {
            a: Rc::clone(&a),
            b: Rc::clone(&b),
            test_color: Tuples::color(0.0,0.0,0.0),
            transform: Matrix::new(4),
            transform_inverse: Matrix::new(4)
        }))
    }
}

impl Pattern for BlendedPattern {
    fn color_a(&self) -> &Tuples {
        &self.test_color
    }
    fn color_b(&self) -> &Tuples {
        &self.test_color
    }
    fn color_at(&self, point: &Tuples) -> Tuples {
        let point_sub_pattern = self.a.borrow().get_transform_inverse() * point;
        let mut color = self.a.borrow().color_at(&point_sub_pattern).clone();
        let point_sub_pattern = self.b.borrow().get_transform_inverse() * point;
        let color_b = self.b.borrow().color_at(&point_sub_pattern).clone();
        color.add(&color_b).scale(0.5)
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
pub struct PerturbedPattern {
    pub pattern: Rc<RefCell<dyn Pattern>>,
    test_color: Tuples,
    transform: Matrix,
    transform_inverse: Matrix,
}

impl PerturbedPattern {
    pub fn new(pattern: Rc<RefCell<dyn Pattern>>) -> Rc<RefCell<PerturbedPattern>> {
        Rc::new(RefCell::new(PerturbedPattern {
            pattern: Rc::clone(&pattern),
            test_color: Tuples::color(0.0,0.0,0.0),
            transform: Matrix::new(4),
            transform_inverse: Matrix::new(4)
        }))
    }
}

impl Pattern for PerturbedPattern {
    fn color_a(&self) -> &Tuples {
        &self.test_color
    }
    fn color_b(&self) -> &Tuples {
        &self.test_color
    }
    fn color_at(&self, point: &Tuples) -> Tuples {
        let perturbed_point = perlin_noise(point);
        let point_sub_pattern = self.pattern.borrow().get_transform_inverse() * &perturbed_point;
        self.pattern.borrow().color_at(&point_sub_pattern).clone()
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