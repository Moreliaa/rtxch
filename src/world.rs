use crate::lights::point_light;
use crate::Intersection;
use crate::PointLight;
use crate::Sphere;
use std::rc::Rc;
use std::cell::RefCell;
use crate::Material;
use crate::Tuples;
use crate::Shape;
use crate::Matrix;
use crate::intersections::IntersectionList;
use crate::Ray;
use crate::Computations;
use crate::render;

#[derive(Debug, Default)]
pub struct World {
    objects: Vec<Rc<RefCell<dyn Shape>>>,
    point_lights: Vec<PointLight>,
}

impl World {
    pub fn new () -> World {
        World { objects: vec![], point_lights: vec![] }
    }

    pub fn is_shadowed(w: &World, point: &Tuples, light: &PointLight) -> bool {
        let mut vector = light.position().clone().subtract(point);
        let distance = vector.clone().magnitude();
        let direction = vector.normalize();
        let ray = Ray::new(point.clone(),  direction);
        let is = World::intersect_world(&w, &ray);
        let hit = IntersectionList::hit(&is);
        if let Some(h) = hit {
            if h.t() < distance {
                return true;
            }
        }
        return false;
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

    pub fn shade_hit(w: &World, comps: &Computations) -> Tuples {
        let mut color = Tuples::color(0.0,0.0,0.0);
        for light in w.get_point_lights() {
            let in_shadow = World::is_shadowed(w, &comps.over_point, light);
            let result = render::lighting(comps.object.borrow().get_material(), light,
            &comps.point, &comps.eye_v, &comps.normal_v, in_shadow);
            color.add(&result);
        }
        
        color
    }

    pub fn intersect_world(w: &World, r: &Ray) -> IntersectionList {
        let mut result: IntersectionList = IntersectionList::create_empty();
        for s in w.get_objects() {
            let xs = <dyn Shape>::intersect(s, r);
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
        s1.borrow_mut().set_material(&material);
        world.add_object(s1);
        
        let s2 = Sphere::new();
        let transform = Matrix::scale(0.5, 0.5, 0.5);
        s2.borrow_mut().set_transform(&transform);
        world.add_object(s2);

        let p = Tuples::point(-10.0,10.0,-10.0);
        let c = Tuples::color(1.0,1.0,1.0);
        let pl = point_light(&p, &c);
        world.add_point_light(pl);

        world
    }

    pub fn add_object(&mut self, sphere: Rc<RefCell<dyn Shape>>) {
        let _ = &self.objects.push(sphere);
    }

    pub fn add_point_light(&mut self, point_light: PointLight) {
        let _ = &self.point_lights.push(point_light);
    }

    pub fn remove_lights(&mut self) {
        self.point_lights = vec![];
    }

    pub fn get_objects(&self) -> &Vec<Rc<RefCell<dyn Shape>>> {
        &self.objects
    }

    pub fn get_point_lights(&self) -> &Vec<PointLight> {
        &self.point_lights
    }
}