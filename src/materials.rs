use crate::{utils::is_equal_f64, Tuples};
use crate::patterns::*;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Material {
    pub pattern: Rc<dyn Pattern>,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
}

impl Material {
    pub fn material() -> Material {
        Material {
            pattern: Rc::new(SingleColorPattern {color: Tuples::color(1.0,1.0,1.0)}),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
        }
    }

    pub fn is_equal(&self, other: &Material) -> bool {
        // TODO check pattern
        is_equal_f64(self.ambient, other.ambient) &&
        is_equal_f64(self.diffuse, other.diffuse) &&
        is_equal_f64(self.specular, other.specular) &&
        is_equal_f64(self.shininess, other.shininess)
    }
}