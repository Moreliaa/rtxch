extern crate rtxch_lib;

use std::collections::HashMap;
use cucumber::{given, when, then, World};
use intersections::Shape;
use rtxch_lib::*;
use rtxch_lib::utils::parse_values_f64;
use std::rc::Rc;
use std::cell::RefCell;

#[given(regex = r"w ← world()")]
fn given_world(world: &mut WorldWorld, _: &[String]) {
    world.world = rtxch_lib::World::new();
}

#[when(regex = r"w ← default_world()")]
fn given_default_world(world: &mut WorldWorld, _: &[String]) {
    world.world = rtxch_lib::World::default_world();
}

#[given(regex = r"(.+) ← (point|vector|ray|intersect|translation|scaling|normal_at|rotation_z|color|point_light)\((.*)\)")]
fn given_item(world: &mut WorldWorld, matches: &[String]) {
    create_item(world, matches);
}

#[given("s1 ← sphere() with: material.color(0.8, 1.0, 0.6) | material.diffuse(0.7) | material.specular(0.2)")]
fn sphere1(world: &mut WorldWorld) {
    let sphere = Sphere::new();
    let mut material = Material::material();
    material.color = Tuples::color(0.8,1.0,0.6);
    material.diffuse = 0.7;
    material.specular = 0.2;
    Sphere::set_material(&sphere, &material);

    world.sphere.insert("s1".to_string(), sphere);
}

#[given("s2 ← sphere() with: transform | scaling(0.5, 0.5, 0.5)")]
fn sphere2(world: &mut WorldWorld) {
    let sphere = Sphere::new();
    let transform = Matrix::scale(0.5, 0.5, 0.5);
    Sphere::set_transform(&sphere, &transform);

    world.sphere.insert("s2".to_string(), sphere);
}

fn create_item(world: &mut WorldWorld, matches: &[String]) {
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
fn when_item(world: &mut WorldWorld, matches: &[String]) {
    create_item(world, matches);
}

#[then(regex = r"w.light = light")]
fn then_light(world: &mut WorldWorld, _: &[String]) {
    let light = world.plight.get(&"light".to_string()).unwrap();
    let world_light = world.world.get_point_lights().get(0).unwrap();
    assert!(world_light.is_equal(&light));
}

#[then(regex = r"w contains (s\d)")]
fn contains(world: &mut WorldWorld, matches: &[String]) {
    let sphere = world.sphere.get(&matches[0]).unwrap();
    let world_objects = world.world.get_objects();
    let mut result = false;
    for o in world_objects {
        if o.borrow().is_equal(&sphere) {
            result = true;
            break;
        }
    }
    assert!(result);
}

#[then(regex = r"w contains no objects")]
fn no_obj(world: &mut WorldWorld, _: &[String]) {
    assert!(world.world.get_objects().len() == 0);
}

#[then(regex = r"w has no light source")]
fn no_light(world: &mut WorldWorld, _: &[String]) {
    assert!(world.world.get_point_lights().len() == 0);
}

    


#[derive(Debug, Default, World)]
struct WorldWorld {
    world: rtxch_lib::World,
    plight: HashMap<String, PointLight>,
    tuple: HashMap<String, Tuples>,
    sphere: HashMap<String, Rc<RefCell<Sphere>>>,
}



fn main() {
    futures::executor::block_on(WorldWorld::run(
        "tests/features/world.feature",
    ));
}