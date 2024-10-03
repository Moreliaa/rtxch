use crate::lights::point_light;
use crate::PointLight;
use crate::Sphere;
use std::rc::Rc;
use std::cell::RefCell;
use crate::Material;
use crate::Tuples;
use crate::intersections::Shape;
use crate::Matrix;
use crate::intersections::{Intersection, IntersectionList};
use crate::Ray;

#[derive(Debug, Default)]
pub struct World {
    objects: Vec<Rc<RefCell<Sphere>>>,
    point_lights: Vec<PointLight>,
}

impl World {
    pub fn new () -> World {
        World { objects: vec![], point_lights: vec![] }
    }

    pub fn intersect_world(w: &World, r: &Ray) -> IntersectionList<Sphere> {
        let mut result: IntersectionList<Sphere> = IntersectionList::create_empty();
        for s in w.get_objects() {
            let xs = Sphere::intersect(s, r);
            result = IntersectionList::merge(result, xs);
        }
        result
    }

    pub fn default_world () -> World {
        let mut world = World::new();
        let s1 = Sphere::new();
        let mut material = Material::material();
        material.color = Tuples::color(0.8,1.0,0.6);
        material.diffuse = 0.7;
        material.specular = 0.2;
        Sphere::set_material(&s1, &material);
        world.add_object(s1);
        
        let s2 = Sphere::new();
        let transform = Matrix::scale(0.5, 0.5, 0.5);
        Sphere::set_transform(&s2, &transform);
        world.add_object(s2);

        let p = Tuples::point(-10.0,10.0,-10.0);
        let c = Tuples::color(1.0,1.0,1.0);
        let pl = point_light(&p, &c);
        world.add_point_light(pl);

        world
    }

    pub fn add_object(&mut self, sphere: Rc<RefCell<Sphere>>) {
        let _ = &self.objects.push(sphere);
    }

    pub fn add_point_light(&mut self, point_light: PointLight) {
        let _ = &self.point_lights.push(point_light);
    }

    pub fn get_objects(&self) -> &Vec<Rc<RefCell<Sphere>>> {
        &self.objects
    }

    pub fn get_point_lights(&self) -> &Vec<PointLight> {
        &self.point_lights
    }
}