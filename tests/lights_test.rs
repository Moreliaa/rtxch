extern crate rtxch_lib;

use std::collections::HashMap;
use cucumber::{given, when, then, World};
use rtxch_lib::utils::parse_values_f64;
use rtxch_lib::*;
use std::rc::Rc;
use std::cell::RefCell;
use rtxch_lib::Matrix;

#[given(regex = r"(.+) ← (point|vector|ray|sphere|intersect|translation|scaling|normal_at|rotation_z|color)\((.*)\)")]
fn given_item(world: &mut LightsWorld, matches: &[String]) {
    create_item(world, matches);
}

fn create_item(world: &mut LightsWorld, matches: &[String]) {
    let t = matches[0].clone();
    let func = matches[1].as_str();
    match func {
        "point" => {
            let v = parse_values_f64(&matches[2]);
            let p = Tuples::point(v[0], v[1], v[2]);
            world.tuple.insert(t, p);
        },
        "vector" => {
            let v = parse_values_f64(&matches[2]);
            let p = Tuples::vector(v[0], v[1], v[2]);
            world.tuple.insert(t, p);
        },
        "ray" => {
            let v: Vec<&str> = matches[2].split(", ").collect();
            let o = world.tuple.get(&v[0].to_string()).unwrap();
            let d = world.tuple.get(&v[1].to_string()).unwrap();
            let r = Ray::new(o.clone(), d.clone());
            world.ray.insert(t, r);
        },
        "sphere" => {
            world.sphere.insert(t, Sphere::new());
        },
        "intersect" => {
            let v: Vec<&str> = matches[2].split(", ").collect();
            let s = world.sphere.get(&v[0].to_string()).unwrap();
            let r = world.ray.get(&v[1].to_string()).unwrap();
            world.inter.insert(t, <dyn Shape>::intersect(s, r));
        },
        "translation" => {
            let v = parse_values_f64(&matches[2]);
            let m = Matrix::translate(v[0], v[1], v[2]);
            world.matrix.insert(t, m);
        },
        "scaling" =>  {
            let v = parse_values_f64(&matches[2]);
            let m = Matrix::scale(v[0], v[1], v[2]);
            world.matrix.insert(t, m);
        },
        "rotation_z" => {
            let v = parse_values_f64(&matches[2]);
            let m = Matrix::rotate_z(v[0]);
            world.matrix.insert(t, m);
        },
        "normal_at" => {
            let v: Vec<&str> = matches[2].split(", ").collect();
            let s = world.sphere.get(&v[0].to_string()).unwrap();
            let p = world.tuple.get(&v[1].to_string()).unwrap();
            world.tuple.insert(t, <dyn Shape>::normal_at(s, p));
        },
        "color" => {
            let v = parse_values_f64(&matches[2]);
            let p = Tuples::color(v[0], v[1], v[2]);
            world.tuple.insert(t, p);
        },
        "point_light" => {
            let v: Vec<&str> = matches[2].split(", ").collect();
            let pos = world.tuple.get(&v[0].to_string()).unwrap();
            let intensity = world.tuple.get(&v[1].to_string()).unwrap();
            world.plight.insert(t, lights::point_light(pos, intensity));
        },
        _ => panic!("{func} not implemented")
    }
}

#[when(regex = r"(.+) ← (point|vector|ray|sphere|intersect|translation|scaling|normal_at|point_light)\((.*)\)")]
fn when_item(world: &mut LightsWorld, matches: &[String]) {
    create_item(world, matches);
}

#[then(regex = r"([^\[\]]+)\.(origin|direction|t|object|count|position|intensity) = (.+)")]
fn check_prop(world: &mut LightsWorld, matches: &[String]) {
    let prop = matches[1].as_str();
    
    match prop {
        "origin" => {
            let r = world.ray.get(&matches[0]).unwrap();
            let target = world.tuple.get(&matches[2]).unwrap();
            assert!(r.origin().is_equal(target));
        },
        "direction" => {
            let r = world.ray.get(&matches[0]).unwrap();
            let target = world.tuple.get(&matches[2]).unwrap();
            assert!(r.direction().is_equal(target));
        },
        "position" => {
            let i = world.plight.get(&matches[0]).unwrap();
            let target = world.tuple.get(&matches[2]).unwrap();
            assert!(i.position().is_equal(target));
        },
        "intensity" => {
            let i = world.plight.get(&matches[0]).unwrap();
            let target = world.tuple.get(&matches[2]).unwrap();
            assert!(i.intensity().is_equal(target));
        },
        _ => panic!()
    }
}

#[derive(Debug, Default, World)]
struct LightsWorld {
    ray: HashMap<String, Ray>,
    tuple: HashMap<String, Tuples>,
    sphere: HashMap<String, Rc<RefCell<dyn Shape>>>,
    inter:  HashMap<String, IntersectionList>,
    matrix: HashMap<String, Matrix>,
    plight: HashMap<String, PointLight>,
}

fn main() {
    futures::executor::block_on(LightsWorld::run(
        "tests/features/lights.feature",
    ));
}