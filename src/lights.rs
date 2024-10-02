use crate::Tuples;

pub fn point_light(position: &Tuples, intensity: &Tuples) -> PointLight {
    PointLight { position: position.clone(), intensity: intensity.clone() }
}

#[derive(Debug)]
pub struct PointLight {
    position: Tuples,
    intensity: Tuples,
}

impl PointLight {
    pub fn position(&self) -> &Tuples {
        &self.position
    }

    pub fn intensity(&self) -> &Tuples {
        &self.intensity
    }

    pub fn is_equal(&self, other: &PointLight) -> bool {
        self.position().is_equal(other.position()) && self.intensity().is_equal(other.intensity()) 
    }
}