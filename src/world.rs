use crate::PointLight;
use crate::Sphere;

#[derive(Debug, Default)]
pub struct World {
    objects: Vec<Sphere>,
    point_lights: Vec<PointLight>,
}

impl World {
    pub fn new () -> World {
        World { objects: vec![], point_lights: vec![] }
    }

    pub fn get_objects(&self) -> &Vec<Sphere> {
        &self.objects
    }

    pub fn get_point_lights(&self) -> &Vec<PointLight> {
        &self.point_lights
    }
}