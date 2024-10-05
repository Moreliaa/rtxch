extern crate rtxch_lib;

use std::collections::HashMap;
use cucumber::{given, when, then, World};
use rtxch_lib::intersections::Shape;
use rtxch_lib::utils::{parse_values_f64, is_equal_f64};
use rtxch_lib::*;
use std::rc::Rc;
use std::cell::RefCell;

#[given(regex = r"(.+) ← (point|vector|ray|sphere|intersect|translation|scaling|normal_at|rotation_z|material|color|point_light)\((.*)\)")]
fn given_item(world: &mut MaterialsWorld, matches: &[String]) {
    create_item(world, matches);
}

#[given(regex = r"in_shadow ← (.+)")]
fn in_shadow_set(world: &mut MaterialsWorld, matches: &[String]) {
    match matches[0].as_str() {
        "false" => world.in_shadow = false,
        "true" => world.in_shadow = true,
        _ =>  panic!()
    };
}

fn create_item(world: &mut MaterialsWorld, matches: &[String]) {
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
        "color" => {
            let v = parse_values_f64(&matches[2]);
            let p = Tuples::color(v[0], v[1], v[2]);
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
            world.inter.insert(t, Sphere::intersect(s, r));
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
            world.tuple.insert(t, Sphere::normal_at(s, p));
        },
        "material" => {
            world.material.insert(t, Material::material());
        },
        "point_light" => {
            let v: Vec<&str> = matches[2].split(", ").collect();
            let pos = world.tuple.get(&v[0].to_string()).unwrap();
            let intensity = world.tuple.get(&v[1].to_string()).unwrap();
            world.plight.insert(t, lights::point_light(pos, intensity));
        },
        "lighting" => {
            let v: Vec<&str> = matches[2].split(", ").collect();
            let m = world.material.get(&v[0].to_string()).unwrap();
            let pl = world.plight.get(&v[1].to_string()).unwrap();
            let position = world.tuple.get(&v[2].to_string()).unwrap();
            let eyev = world.tuple.get(&v[3].to_string()).unwrap();
            let normalv = world.tuple.get(&v[4].to_string()).unwrap();
            world.tuple.insert(t, lighting(m, pl, position, eyev, normalv, world.in_shadow));
        }
        _ => panic!("{func} not implemented")
    }
}

#[when(regex = r"(.+) ← (point|vector|ray|sphere|intersect|translation|scaling|normal_at|lighting)\((.*)\)")]
fn when_item(world: &mut MaterialsWorld, matches: &[String]) {
    create_item(world, matches);
}

#[then(regex = r"result = color\((.+)\)")]
fn check_result(world: &mut MaterialsWorld, matches: &[String]) {
    let val = parse_values_f64(&matches[0]);
    let col = Tuples::color(val[0], val[1], val[2]);
    let r = world.tuple.get(&"result".to_string()).unwrap();
    r.is_equal(&col);
}

#[then(regex = r"([^\[\]]+)\.(origin|direction|t|object|count|position|intensity|color|ambient|diffuse|specular|shininess) = (.+)")]
fn check_prop(world: &mut MaterialsWorld, matches: &[String]) {
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
        "color" => {
            let i = world.material.get(&matches[0]).unwrap();
            let target = world.tuple.get(&matches[2]).unwrap();
            assert!(i.color.is_equal(target));
        },
        "ambient" => {
            let i = world.material.get(&matches[0]).unwrap();
            let target = &matches[2].parse::<f64>().unwrap();
            assert!(is_equal_f64(i.ambient, *target));
        },
        "diffuse" => {
            let i = world.material.get(&matches[0]).unwrap();
            let target = &matches[2].parse::<f64>().unwrap();
            assert!(is_equal_f64(i.diffuse, *target));
        },
        "specular" => {
            let i = world.material.get(&matches[0]).unwrap();
            let target = &matches[2].parse::<f64>().unwrap();
            assert!(is_equal_f64(i.specular, *target));
        },
        "shininess" => {
            let i = world.material.get(&matches[0]).unwrap();
            let target = &matches[2].parse::<f64>().unwrap();
            assert!(is_equal_f64(i.shininess, *target));
        },
        _ => panic!()
    }
}

#[derive(Debug, Default, World)]
struct MaterialsWorld {
    ray: HashMap<String, Ray>,
    tuple: HashMap<String, Tuples>,
    sphere: HashMap<String, Rc<RefCell<Sphere>>>,
    inter:  HashMap<String, IntersectionList<Sphere>>,
    matrix: HashMap<String, Matrix>,
    material: HashMap<String, Material>,
    plight: HashMap<String, PointLight>,
    in_shadow: bool,
}

fn main() {
    futures::executor::block_on(MaterialsWorld::run(
        "tests/features/materials.feature",
    ));
}