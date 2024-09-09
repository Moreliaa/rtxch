extern crate rtxch_lib;

use std::collections::HashMap;
use std::f64::consts::PI;
use cucumber::{given, when, then, World};
use rtxch_lib::utils::parse_values_f64;
use rtxch_lib::Matrix;
use rtxch_lib::Tuples;
use rtxch_lib::Ray;

#[given(regex = r"(.+) ← (point|vector|ray)\((.+)\)")]
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
        _ => panic!("{func} not implemented")
    }
}

#[when(regex = r"(.+) ← (point|vector|ray)\((.+)\)")]
fn when_item(world: &mut RaysWorld, matches: &[String]) {
    create_item(world, matches);
}

#[then(regex = r"(.+)\.(origin|direction) = (.+)")]
fn check_prop(world: &mut RaysWorld, matches: &[String]) {
    let r = world.ray.get(&matches[0]).unwrap();
    let prop = matches[1].as_str();
    let t = world.tuple.get(&matches[2]).unwrap();
    match prop {
        "origin" => {
            r.origin().is_equal(t);
        },
        "direction" => {
            r.direction().is_equal(t);
        },
        _ => panic!()
    }
}

#[derive(Debug, Default, World)]
struct RaysWorld {
    ray: HashMap<String, Ray>,
    tuple: HashMap<String, Tuples>
}

fn main() {
    futures::executor::block_on(RaysWorld::run(
        "tests/features/rays.feature",
    ));
}