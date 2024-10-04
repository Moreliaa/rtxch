use crate::lights::point_light;
use crate::Intersection;
use crate::PointLight;
use crate::Sphere;
use std::rc::Rc;
use std::cell::RefCell;
use crate::Material;
use crate::Tuples;
use crate::intersections::Shape;
use crate::Matrix;
use crate::intersections::IntersectionList;
use crate::Ray;
use crate::Computations;
use crate::render;

#[derive(Debug, Default)]
pub struct World {
    objects: Vec<Rc<RefCell<Sphere>>>,
    point_lights: Vec<PointLight>,
}

impl World {
    pub fn new () -> World {
        World { objects: vec![], point_lights: vec![] }
    }

    pub fn color_at(w: &World, r: &Ray) -> Tuples {
        let il = World::intersect_world(w, r);
        let hit = IntersectionList::hit(&il);
        if let Some(i) = hit {
            let comps = Intersection::prep_computations(i, r);
            return World::shade_hit(w, &comps);
        } else {
            return Tuples::color(0.0,0.0,0.0);
        }
    }

    pub fn shade_hit(w: &World, comps: &Computations<Sphere>) -> Tuples {
        let mut color = Tuples::color(0.0,0.0,0.0);
        for light in w.get_point_lights() {
            let result = render::lighting(comps.object.borrow().get_material(), light,
            &comps.point, &comps.eye_v, &comps.normal_v);
            color.add(&result);
        }
        
        color
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

    pub fn remove_lights(&mut self) {
        self.point_lights = vec![];
    }

    pub fn get_objects(&self) -> &Vec<Rc<RefCell<Sphere>>> {
        &self.objects
    }

    pub fn get_point_lights(&self) -> &Vec<PointLight> {
        &self.point_lights
    }
}