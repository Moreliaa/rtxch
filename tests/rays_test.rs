extern crate rtxch_lib;

use std::collections::HashMap;
use cucumber::{given, when, then, World};
use rtxch_lib::utils::parse_values_f64;
use rtxch_lib::Tuples;
use rtxch_lib::Ray;
use rtxch_lib::Matrix;

#[given(regex = r"(.+) ← (point|vector|ray|translation|scaling|transform)\((.+)\)")]
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
        "translation" => {
            let v: Vec<f64> = parse_values_f64(&matches[2]);
            world.matrix.insert(t, Matrix::translate(v[0], v[1], v[2]));
        },
        "scaling" => {
            let v: Vec<f64> = parse_values_f64(&matches[2]);
            world.matrix.insert(t, Matrix::scale(v[0], v[1], v[2]));
        },
        "transform" => {
            let v: Vec<&str> = matches[2].split(", ").collect();
            let r = world.ray.get(&v[0].to_string()).unwrap();
            let m = world.matrix.get(&v[1].to_string()).unwrap();
            world.ray.insert(t,Ray::transform(r, m));
        }
        _ => panic!("{func} not implemented")
    }
}

#[when(regex = r"(.+) ← (point|vector|ray|translation|scaling|transform)\((.+)\)")]
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
            assert!(r.origin().is_equal(t));
        },
        "direction" => {
            assert!(r.direction().is_equal(t));
        },
        _ => panic!()
    }
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
    matrix: HashMap<String, Matrix>,
}

fn main() {
    futures::executor::block_on(RaysWorld::run(
        "tests/features/rays.feature",
    ));
}