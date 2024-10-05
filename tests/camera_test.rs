extern crate rtxch_lib;

use std::collections::HashMap;
use std::f64::consts::PI;
use cucumber::{given, when, then, World};
use render::render;
use rtxch_lib::*;
use rtxch_lib::utils::*;

#[given(regex = r"c ← camera\((.+)\)")]
#[when(regex = r"c ← camera\((.+)\)")]
fn cam(world: &mut CameraWorld, matches: &[String]) {
    let val = parse_values_f64(&matches[0]);
    world.camera = Camera::new(val[0] as usize,  val[1] as usize, val[2] as f64);
}

#[when(regex = r"r ← ray_for_pixel\(c, (.+)\)")]
fn ray_for_pixel(world: &mut CameraWorld, matches: &[String]) {
    let camera = &world.camera;
    let val = parse_values_usize(&matches[0]);
    let ray = Camera::ray_for_pixel(camera, val[0], val[1]);
    world.ray.insert("r".to_string(), ray);
}

#[then(regex = r"(.+)\.(origin|direction) = (.+)")]
fn check_prop(world: &mut CameraWorld, matches: &[String]) {
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

#[given("c.transform ← view_transform(from, to, up)")]
fn transform_2(world: &mut CameraWorld) {
    let from = world.tuple.get(&"from".to_string()).unwrap();
    let to = world.tuple.get(&"to".to_string()).unwrap();
    let up = world.tuple.get(&"up".to_string()).unwrap();
    let t = Matrix::view_transform(from, to, up);
    world.camera.transform = t;
}

#[when("c.transform ← rotation_y(π/4) * translation(0, -2, 5)")]
fn transform(world: &mut CameraWorld) {
    let mat = Matrix::rotate_y(PI / 4.0) * Matrix::translate(0.0,-2.0,5.0);
    world.camera.transform = mat;
}

#[when("image ← render(c, w)")]
fn render_image(world: &mut CameraWorld) {
    let image = render(&world.camera, &world.world);
    world.image = image;
}

#[then("pixel_at(image, 5, 5) = color(0.38066, 0.47583, 0.2855)")]
fn pixel_at(world: &mut CameraWorld) {
    let pixel = world.image.pixel_at(5, 5);
    let c = Tuples::color(0.38066, 0.47583, 0.2855);
    assert!(pixel.is_equal(&c));
}

#[then(regex = r"^c.(.+) = (.+)")]
fn cam_prop(world: &mut CameraWorld, matches: &[String]) {
    let camera = &world.camera;
    let prop = matches[0].as_str();
    
    match prop {
        "hsize" => {
            let target = &matches[1].parse::<usize>().unwrap();
            assert!(camera.h_size == *target);
        },
        "vsize" => {
            let target = &matches[1].parse::<usize>().unwrap();
            assert!(camera.v_size == *target);
        },
        "field_of_view" => {
            let target = &matches[1].parse::<f64>().unwrap();
            assert!(camera.fov == *target);
        },
        "transform" => {
            let target = Matrix::new(4);
            assert!(camera.transform.is_equal(&target));
        },
        "pixel_size" => {
            let target = &matches[1].parse::<f64>().unwrap();
            assert!(is_equal_f64(camera.pixel_size, *target));
        },
        _ => panic!()
    }
}

#[given(regex = r"(.+) ← (point|vector|ray|intersect|translation|scaling|normal_at|rotation_z|color|point_light)\((.*)\)")]
fn given_item(world: &mut CameraWorld, matches: &[String]) {
    create_item(world, matches);
}

fn create_item(world: &mut CameraWorld, matches: &[String]) {
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
        _ => panic!("{func} not implemented")
    }
}

#[when(regex = r"w ← default_world()")]
#[given(regex = r"w ← default_world()")]
fn given_default_world(world: &mut CameraWorld, _: &[String]) {
    world.world = rtxch_lib::World::default_world();
}

#[derive(Debug, Default, World)]
struct CameraWorld {
    camera: Camera,
    tuple: HashMap<String, Tuples>,
    ray: HashMap<String, Ray>,
    world: rtxch_lib::World,
    image: Canvas,
}



fn main() {
    futures::executor::block_on(CameraWorld::run(
        "tests/features/camera.feature",
    ));
}