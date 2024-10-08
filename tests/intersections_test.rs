extern crate rtxch_lib;

use std::collections::HashMap;
use cucumber::{given, when, then, World};
use rtxch_lib::utils::parse_values_f64;
use rtxch_lib::utils::is_equal_f64;
use rtxch_lib::*;
use rtxch_lib::intersections::Shape;
use utils::EPSILON;
use std::rc::Rc;
use std::cell::RefCell;
use rtxch_lib::Computations;


#[given(regex = r"(.+) ← (point|vector|ray|sphere|intersection|intersections|hit|prepare_computations)\((.*)\)$")]
#[when(regex = r"(.+) ← (point|vector|ray|sphere|intersection|intersections|hit|prepare_computations)\((.*)\)$")]
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
        "intersection" => {
            let v: Vec<&str> = matches[2].split(", ").collect();
            let time = v[0].parse::<f64>().unwrap();
            let obj = world.sphere.get(&v[1].to_string()).unwrap();
            world.inter_sphere.insert(t, Intersection::new(time, obj));
        },
        "intersections" => {
            let v: Vec<&str> = matches[2].split(", ").collect();
            let i: Vec<Intersection<Sphere>> = v.into_iter().map(|val| world.inter_sphere.get(&val.to_string()).unwrap().clone()).collect();
            let l = IntersectionList::intersections_from_vec(i);
            world.interlist_sphere.insert(t, l);
        },
        "hit" => {
            let il = world.interlist_sphere.get(&matches[2]).unwrap();
            let hit = intersections::IntersectionList::hit(il);
            match hit {
                Some(value) => world.hit.insert(t, Some(value.clone())),
                None => world.hit.insert(t, None),
            };
        },
        "prepare_computations" => {
            let v: Vec<&str> = matches[2].split(", ").collect();
            let i = world.inter_sphere.get(&v[0].to_string()).unwrap();
            let r = world.ray.get(&v[1].to_string()).unwrap();
            let comps = Intersection::prep_computations(i, r);
            world.comps.insert(t, comps);
        },
        _ => panic!("{func} not implemented")
    }
}

#[then(regex = r"(i|i.|xs)\.(origin|direction|t|object|count) = (.+)")]
fn check_prop(world: &mut RaysWorld, matches: &[String]) {
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
        "t" => {
            let i = world.inter_sphere.get(&matches[0]).unwrap();
            let target = matches[2].parse::<f64>().unwrap();
            assert!(is_equal_f64(i.t(), target));
        },
        "object" => {
            let i = world.inter_sphere.get(&matches[0]).unwrap();
            let target = world.sphere.get(&matches[2]).unwrap();
            assert!(Rc::ptr_eq(i.object(), &target));
        },
        "count" => {
            let i = world.interlist_sphere.get(&matches[0]).unwrap();
            let target = matches[2].parse::<usize>().unwrap();
            assert!(i.count() == target);
        }
        _ => panic!()
    }
}

#[then(regex = r"(comps)\.(t|object|point|eyev|normalv|inside) = (.+)")]
fn check_prop_comps(world: &mut RaysWorld, matches: &[String]) {
    let comps = world.comps.get(&matches[0]).unwrap();
    let prop = matches[1].as_str();
    
    match prop {
        "t" => {
            let i = world.inter_sphere.get(&"i".to_string()).unwrap();
            assert!(is_equal_f64(comps.t, i.t()));
        },
        "object" => {
            let i = world.inter_sphere.get(&"i".to_string()).unwrap();
            assert!(Rc::ptr_eq(i.object(), &comps.object));
        },
        "point" => {
            let target = world.tuple.get(&matches[2]).unwrap();
            assert!(comps.point.is_equal(&target));
        },
        "eyev" => {
            let target = world.tuple.get(&matches[2]).unwrap();
            assert!(comps.eye_v.is_equal(&target));
        },
        "normalv" => {
            let target = world.tuple.get(&matches[2]).unwrap();
            assert!(comps.normal_v.is_equal(&target));
        },
        "inside" => {
            let target = match matches[2].as_str() {
                "false" => false,
                "true" => true,
                _ => panic!(),
            };
            assert!(comps.inside == target);

        },
        _ => panic!()
    }
}

#[then(regex = r"(comps)\.(point.z|over_point.z) (<|>) (.+)")]
fn check_prop_less_than_comps(world: &mut RaysWorld, matches: &[String]) {
    let comps = world.comps.get(&matches[0]).unwrap();
    let prop = matches[1].as_str();
    let operator = matches[2].as_str();
    
    let prop_val = match prop {
        "over_point.z" => comps.over_point.z,
        "point.z" => comps.point.z,
        _ => panic!()
    };
    let target = match matches[3].as_str() {
        "-EPSILON/2" => -EPSILON / 2.0,
        "comps.over_point.z" => comps.over_point.z,
        _ => panic!(),
    };
    match operator {
        ">" => assert!(prop_val > target, "{:#?} {}", comps, target),
        "<" => assert!(prop_val < target, "{:#?} {}", comps, target),
        _ => panic!(),
    };
}

#[when("shape ← sphere() with: | transform | translation(0, 0, 1) |")]
fn sphere2_alter(world: &mut RaysWorld) {
    let sphere = Sphere::new();
    let transform = Matrix::translate(0.0,0.0,1.0);
    Sphere::set_transform(&sphere, &transform);

    world.sphere.insert("shape".to_string(), sphere);
}

#[then(regex = r"(.+)\[(.+)\]\.(origin|direction|t|object|count) = (.+)")]
fn check_sub_prop(world: &mut RaysWorld, matches: &[String]) {
    let prop = matches[2].as_str();
    let i = world.interlist_sphere.get(&matches[0]).unwrap();
    let idx = matches[1].parse::<usize>().unwrap();
    let obj = i.xs().get(idx).unwrap();

    match prop {
        "t" => {
            let target = matches[3].parse::<f64>().unwrap();
            assert!(is_equal_f64(obj.t(), target));
        },
        "object" => {
            let target = world.sphere.get(&matches[3]).unwrap();
            assert!(Rc::ptr_eq(obj.object(), &target));
        },
        _ => panic!()
    }
}

#[then(regex = r"i = (.+)")]
fn check_hit(world: &mut RaysWorld, matches: &[String]) {
    let i = world.hit.get("i").unwrap();
    let other = world.inter_sphere.get(&matches[0]).unwrap();
    match i {
        None => {
            assert!(false, "Expected other, got none.");
        },
        Some(result) => {
            assert!(result.is_equal(other));
        }
    }
}

#[then(regex = r"i is nothing")]
fn check_hit_nothing(world: &mut RaysWorld, _: &[String]) {
    let i = world.hit.get("i").unwrap();
    match i {
        None => {
            assert!(true);
        },
        Some(_) => {
            assert!(false);
        }
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
    sphere: HashMap<String, Rc<RefCell<Sphere>>>,
    inter_sphere:  HashMap<String, Intersection<Sphere>>,
    interlist_sphere: HashMap<String, IntersectionList<Sphere>>,
    hit: HashMap<String, Option<Intersection<Sphere>>>,
    comps: HashMap<String, Computations<Sphere>>,
}

fn main() {
    futures::executor::block_on(RaysWorld::run(
        "tests/features/intersections.feature",
    ));
}