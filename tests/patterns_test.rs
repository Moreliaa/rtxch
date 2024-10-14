extern crate rtxch_lib;

use std::collections::HashMap;
use cucumber::{given, when, then, World};
use rtxch_lib::utils::parse_values_f64;
use rtxch_lib::*;
use std::rc::Rc;
use std::cell::RefCell;

#[given(regex = r"(.+) ← (point|vector|ray|test_pattern|stripe_pattern|checkers_pattern|gradient_pattern|ring_pattern|sphere|intersect|translation|scaling|normal_at|rotation_z|material|color|point_light)\((.*)\)")]
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
        "test_pattern" => {
            world.patterns.insert(t, TestPattern::new());
        },
        "checkers_pattern" => {
            let v: Vec<&str> = matches[2].split(", ").collect();
            let o = world.tuple.get(&v[0].to_string()).unwrap();
            let d = world.tuple.get(&v[1].to_string()).unwrap();
            world.patterns.insert(t, CheckersPattern::new(o.clone(), d.clone()));
        },
        "stripe_pattern" => {
            let v: Vec<&str> = matches[2].split(", ").collect();
            let o = world.tuple.get(&v[0].to_string()).unwrap();
            let d = world.tuple.get(&v[1].to_string()).unwrap();
            world.patterns.insert(t, StripePattern::new(o.clone(), d.clone()));
        },
        "gradient_pattern" => {
            let v: Vec<&str> = matches[2].split(", ").collect();
            let o = world.tuple.get(&v[0].to_string()).unwrap();
            let d = world.tuple.get(&v[1].to_string()).unwrap();
            world.patterns.insert(t, GradientPattern::new(o.clone(), d.clone()));
        },
        "ring_pattern" => {
            let v: Vec<&str> = matches[2].split(", ").collect();
            let o = world.tuple.get(&v[0].to_string()).unwrap();
            let d = world.tuple.get(&v[1].to_string()).unwrap();
            world.patterns.insert(t, RingPattern::new(o.clone(), d.clone()));
        },
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
            let obj: Rc<RefCell<dyn Shape>> = Sphere::new();
            world.tuple.insert(t, lighting(m, &obj, pl, position, eyev, normalv, world.in_shadow));
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
#[then(regex = r"color_at\((pattern), point\((.+)\)\) = (.+)")]
fn check_color_at(world: &mut MaterialsWorld, matches: &[String]) {
    let pattern = world.patterns.get(&matches[0]).unwrap();
    let val = parse_values_f64(&matches[1]);
    let point = Tuples::point(val[0],val[1],val[2]);
    let bound_pattern = pattern.borrow();
    let result = bound_pattern.color_at(&point);
    let target = world.tuple.get(&matches[2]).unwrap();
    assert!(result.is_equal(target));
}

#[then(regex = r"color_at_object\((pattern), (object|shape), point\((.+)\)\) = (.+)")]
fn check_color_at_obj(world: &mut MaterialsWorld, matches: &[String]) {
    let obj = world.sphere.get(&matches[1]).unwrap();
    let pattern = world.patterns.get(&matches[0]).unwrap();
    let val = parse_values_f64(&matches[2]);
    let point = Tuples::point(val[0],val[1],val[2]);
    let bound_pattern = pattern.borrow();
    let result = bound_pattern.color_at_object(obj, &point);
    let target = world.tuple.get(&matches[3]).unwrap();
    assert!(result.is_equal(target));
}


#[then(regex = r"(pattern)\.(a|b|transform) = (.+)")]
fn check_prop(world: &mut MaterialsWorld, matches: &[String]) {
    let prop = matches[1].as_str();
    let pattern = world.patterns.get(&matches[0]).unwrap();
    
    match prop {
        "a" => {
            let prop = pattern.borrow().color_a().clone();
            let target = world.tuple.get(&matches[2]).unwrap();
            assert!(prop.is_equal(target));
        },
        "b" => {
            let prop = pattern.borrow().color_b().clone();
            let target = world.tuple.get(&matches[2]).unwrap();
            assert!(prop.is_equal(target));
        },
        "transform" => {
            let ident = match matches[2].as_str() {
                "identity_matrix" => {
                    Matrix::new(4)
                },
                _ => {
                    world.matrix.get(&matches[2].to_string()).unwrap().clone()
                },
            };
            let borrowed = pattern.borrow();
            let target = borrowed.get_transform();
            assert!(ident.is_equal(&target));
        },
        _ => panic!()
    }
}

fn set_transform(world: &mut MaterialsWorld, matches: &[String]) {
    let v: Vec<&str> = matches[0].split(", ").collect();
    let s = world.sphere.get(v[0]).unwrap();
    let t = world.matrix.get(v[1]).unwrap();
    s.borrow_mut().set_transform(t);
}

#[given(regex = r"set_transform\((.+)\)")]
fn given_set_transform(world: &mut MaterialsWorld, matches: &[String]) {
    set_transform(world, matches);
}

#[given(regex = r"set_pattern_transform\((.+)\)")]
#[when(regex = r"set_pattern_transform\((.+)\)")]
fn given_set_pattern_transform(world: &mut MaterialsWorld, matches: &[String]) {
    let v: Vec<&str> = matches[0].split(", ").collect();
    let s = world.patterns.get(v[0]).unwrap();
    let t = world.matrix.get(v[1]).unwrap();
    s.borrow_mut().set_transform(t.clone());
}

#[when(regex = r"set_transform\((.+)\)")]
fn when_set_transform(world: &mut MaterialsWorld, matches: &[String]) {
    set_transform(world, matches);
}

#[derive(Debug, Default, World)]
struct MaterialsWorld {
    ray: HashMap<String, Ray>,
    tuple: HashMap<String, Tuples>,
    sphere: HashMap<String, Rc<RefCell<dyn Shape>>>,
    inter:  HashMap<String, IntersectionList>,
    matrix: HashMap<String, Matrix>,
    material: HashMap<String, Material>,
    plight: HashMap<String, PointLight>,
    in_shadow: bool,
    patterns: HashMap<String, Rc<RefCell<dyn Pattern>>>,
}

fn main() {
    futures::executor::block_on(MaterialsWorld::run(
        "tests/features/patterns.feature",
    ));
}