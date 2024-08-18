extern crate rtxch_lib;

use rtxch_lib::utils::is_equal_f64;
use cucumber::{given, then, World};

#[given(regex = "a ← tuple\\((.+), (.+), (.+), (.+)\\)")]
fn point_tuple(world: &mut TuplesWorld, matches: &[String]) {
    let values: Vec<f64> = matches.into_iter().map(|m| m.parse::<f64>().unwrap()).collect();
    world.tuple = rtxch_lib::Tuples::new(values[0], values[1], values[2], values[3]);
}

#[given(regex = r"a(\d) ← tuple\((.+), (.+), (.+), (.+)\)")]
fn tuple(world: &mut TuplesWorld, matches: &[String]) {
    let values: Vec<f64> = matches[1..5].into_iter().map(|m| m.parse::<f64>().unwrap()).collect();
    let tuple = rtxch_lib::Tuples::new(values[0], values[1], values[2], values[3]);
    match matches[0].as_str() {
        "1" => world.tuple = tuple,
        "2" => world.other = tuple,
        _ => panic!()
    };
}

#[then(regex = r"a1 \+ a2 = tuple\((.+), (.+), (.+), (.+)\)")]
fn add(world: &mut TuplesWorld, matches: &[String]) {
    let values: Vec<f64> = matches.into_iter().map(|m| m.parse::<f64>().unwrap()).collect();
    let wanted = rtxch_lib::Tuples::new(values[0], values[1], values[2], values[3]);
    assert!(wanted.is_equal(world.tuple.add(&world.other)));
}

#[then(regex = r"^a.(.) = (.+)$")]
fn compare(world: &mut TuplesWorld, matches: &[String]) {
    let prop = match matches[0].as_str() {
        "x" => world.tuple.x,
        "y" => world.tuple.y,
        "z" => world.tuple.z,
        "w" => world.tuple.w,
        _ => panic!()
    };
    let value = matches[1].parse::<f64>().unwrap();
    assert!(is_equal_f64(prop, value));
}

#[then(regex = "a is (a|not a) (point|vector)")]
fn is_point(world: &mut TuplesWorld, matches: &[String]) {
    let is_true = matches[0].as_str() == "a";
    let is_target = match matches[1].as_str() {
        "point" => world.tuple.is_point(),
        _ => world.tuple.is_vector(),
    };
    assert!(is_target == is_true);
}

#[given("p ← point(4, -4, 3)")]
fn create_point(world: &mut TuplesWorld) {
    world.tuple = rtxch_lib::Tuples::point(4.0, -4.0, 3.0);
}

#[given("v ← vector(4, -4, 3)")]
fn create_vector(world: &mut TuplesWorld) {
    world.tuple = rtxch_lib::Tuples::vector(4.0, -4.0, 3.0);
}

#[then(regex = "[p|v] = tuple\\((.+), (.+), (.+), (.+)\\)")]
fn check_point(world: &mut TuplesWorld, matches: &[String]) {
    let values: Vec<f64> = matches.into_iter().map(|m| m.parse::<f64>().unwrap()).collect();
    assert!(is_equal_f64(world.tuple.x, values[0]));
    assert!(is_equal_f64(world.tuple.y, values[1]));
    assert!(is_equal_f64(world.tuple.z, values[2]));
    assert!(is_equal_f64(world.tuple.w, values[3]));
}

#[derive(Debug, Default, World)]
struct TuplesWorld {
    tuple: rtxch_lib::Tuples,
    other: rtxch_lib::Tuples,
}

fn main() {
    futures::executor::block_on(TuplesWorld::run(
        "tests/features/tuples.feature",
    ));
}