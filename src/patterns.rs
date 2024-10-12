use std::fmt::Debug;
use crate::Tuples;

pub trait Pattern: Debug {
    fn color_a(&self) -> &Tuples;
    fn color_b(&self) -> &Tuples;
    fn color_at(&self, point: &Tuples) -> &Tuples;
}

#[derive(Debug, Clone)]
pub struct SingleColorPattern {
    pub color: Tuples,
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
}

#[derive(Debug, Clone)]
pub struct StripePattern {
    pub a: Tuples,
    pub b: Tuples,
}

impl StripePattern {
    pub fn new(a: Tuples, b: Tuples) -> StripePattern {
        StripePattern { a, b }
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
}