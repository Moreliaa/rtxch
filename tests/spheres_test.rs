extern crate rtxch_lib;

use std::collections::HashMap;
use cucumber::{given, when, then, World};
use rtxch_lib::intersections::Intersectable;
use rtxch_lib::utils::parse_values_f64;
use rtxch_lib::Tuples;
use rtxch_lib::Ray;
use rtxch_lib::Sphere;
use rtxch_lib::Intersections;


#[given(regex = r"(.+) ← (point|vector|ray|sphere|intersect)\((.*)\)")]
fn given_item(world: &mut RaysWorld, matches: &[String]) {
    create_item(world, matches);
}

fn create_item(world: &mut RaysWorld, matches: &[String]) {
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
            world.inter.insert(t, s.intersect(r));
        },
        _ => panic!("{func} not implemented")
    }
}

#[when(regex = r"(.+) ← (point|vector|ray|sphere|intersect)\((.*)\)")]
fn when_item(world: &mut RaysWorld, matches: &[String]) {
    create_item(world, matches);
}

#[then(regex = r"(.+)\.(origin|direction|count) = (.+)")]
fn check_prop(world: &mut RaysWorld, matches: &[String]) {
    let prop = matches[1].as_str();
    
    match prop {
        "origin" => {
            let r = world.ray.get(&matches[0]).unwrap();
            let t = world.tuple.get(&matches[2]).unwrap();
            assert!(r.origin().is_equal(t));
        },
        "direction" => {
            let r = world.ray.get(&matches[0]).unwrap();
            let t = world.tuple.get(&matches[2]).unwrap();
            assert!(r.direction().is_equal(t));
        },
        "count" => {
            let xs = world.inter.get(&matches[0]).unwrap();
            let t = &matches[2].parse::<usize>().unwrap();
            assert!(xs.count() == *t);
        },
        _ => panic!(),
    }
}

#[then(regex = r"xs\[(.+)\] = (.+)")]
fn check_intersect_idx(world: &mut RaysWorld, matches: &[String]) {
    let idx = &matches[0].parse::<usize>().unwrap();
    let target = &matches[1].parse::<f64>().unwrap();
    let xs = world.inter.get(&"xs".to_string()).unwrap();
    let val = xs.xs().get(*idx).unwrap();
    assert!(val == target);
}

#[then(regex = r"position\((.+), (.+)\) = point\((.+)\)")]
fn check_pos(world: &mut RaysWorld, matches: &[String]) {
    let r = world.ray.get(&matches[0]).unwrap();
    let time = matches[1].parse::<f64>().unwrap();
    let v = parse_values_f64(&matches[2]);
    let p = Tuples::point(v[0], v[1], v[2]);

    let result = Ray::position(r, time);
    assert!(result.is_equal(&p));
}

#[derive(Debug, Default, World)]
struct RaysWorld {
    ray: HashMap<String, Ray>,
    tuple: HashMap<String, Tuples>,
    sphere: HashMap<String, Sphere>,
    inter:  HashMap<String, Intersections>,
}

fn main() {
    futures::executor::block_on(RaysWorld::run(
        "tests/features/spheres.feature",
    ));
}