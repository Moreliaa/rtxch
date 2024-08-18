extern crate rtxch_lib;

use rtxch_lib::test_utils::is_equal_f64;
use cucumber::{given, then, World};

#[given(regex = "a <- Tuples\\((.+), (.+), (.+), (.+)\\)")]
fn point_tuple(world: &mut TuplesWorld, matches: &[String]) {
    let values: Vec<f64> = matches.into_iter().map(|m| m.parse::<f64>().unwrap()).collect();
    world.tuples = rtxch_lib::Tuples::new(values[0], values[1], values[2], values[3]);
}

#[then(regex = r"^a.(.) = (.+)$")]
fn compare(world: &mut TuplesWorld, matches: &[String]) {
    let prop = match matches[0].as_str() {
        "x" => world.tuples.x,
        "y" => world.tuples.y,
        "z" => world.tuples.z,
        "w" => world.tuples.w,
        _ => panic!()
    };
    let value = matches[1].parse::<f64>().unwrap();
    assert!(is_equal_f64(prop, value));
}

#[then(regex = "a is (a|not a) (point|vector)")]
fn is_point(world: &mut TuplesWorld, matches: &[String]) {
    let is_true = matches[0].as_str() == "a";
    let is_target = match matches[1].as_str() {
        "point" => world.tuples.is_point(),
        _ => world.tuples.is_vector(),
    };
    assert!(is_target == is_true);
}

#[given("p <- point(4.0, -4.0, 3.0)")]
fn create_point(world: &mut TuplesWorld) {
    world.tuples = rtxch_lib::Tuples::point(4.0, -4.0, 3.0);
}

#[given("v <- vector(4.0, -4.0, 3.0)")]
fn create_vector(world: &mut TuplesWorld) {
    world.tuples = rtxch_lib::Tuples::vector(4.0, -4.0, 3.0);
}

#[then(regex = "[p|v] = tuple\\((.+), (.+), (.+), (.+)\\)")]
fn check_point(world: &mut TuplesWorld, matches: &[String]) {
    let values: Vec<f64> = matches.into_iter().map(|m| m.parse::<f64>().unwrap()).collect();
    assert!(is_equal_f64(world.tuples.x, values[0]));
    assert!(is_equal_f64(world.tuples.y, values[1]));
    assert!(is_equal_f64(world.tuples.z, values[2]));
    assert!(is_equal_f64(world.tuples.w, values[3]));
}

#[derive(Debug, Default, World)]
struct TuplesWorld {
    tuples: rtxch_lib::Tuples
}

fn main() {
    futures::executor::block_on(TuplesWorld::run(
        "tests/features/tuples.feature",
    ));
}