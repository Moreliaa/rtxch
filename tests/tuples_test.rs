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

#[given(regex = r"(.+) ← (point|vector)\((.+), (.+), (.+)\)")]
fn create_vector_or_point(world: &mut TuplesWorld, matches: &[String]) {
    let values: Vec<f64> = matches[2..].into_iter().map(|m| m.parse::<f64>().unwrap()).collect();
    let tuple = match matches[1].as_str() {
        "point" => rtxch_lib::Tuples::point(values[0], values[1], values[2]),
        "vector" => rtxch_lib::Tuples::vector(values[0], values[1], values[2]),
        _ => panic!(),
    };
    match matches[0].as_str() {
        "a" | "a1" => world.tuple = tuple,
        "a2" => world.other = tuple,
        "p" | "p1" => world.point1 = tuple,
        "p2" => world.point2 = tuple,
        "v" | "v1" => world.vector1 = tuple,  
        "v2" => world.vector2 = tuple,
        _ => panic!(),        
    };
}

// Operation
#[then(regex = r"^(.+) (.+) (.+) = (vector|point)\((.+), (.+), (.+)\)$")]
fn subtract_p_v(world: &mut TuplesWorld, matches: &[String]) {
    let mut param1 = match matches[0].as_str() {
        "a" | "a1" => world.tuple.clone(),
        "a2" => world.other.clone(),
        "p" | "p1" => world.point1.clone(),
        "p2" => world.point2.clone(),
        "v" | "v1" => world.vector1.clone(),
        "v2" => world.vector2.clone(),
        _ => panic!(),
    };

    let op = matches[1].as_str();

    let param2 = match matches[2].as_str() {
        "a" | "a1" => world.tuple.clone(),
        "a2" => world.other.clone(),
        "p" | "p1" => world.point1.clone(),
        "p2" => world.point2.clone(),
        "v" | "v1" => world.vector1.clone(),
        "v2" => world.vector2.clone(),
        _ => panic!(),
    };

    let values: Vec<f64> = matches[4..].into_iter().map(|m| m.parse::<f64>().unwrap()).collect();
    let wanted = match matches[3].as_str() {
        "point" => rtxch_lib::Tuples::point(values[0], values[1], values[2]),
        "vector" => rtxch_lib::Tuples::vector(values[0], values[1], values[2]),
        _ => panic!(),
    };
    let result = match op {
        "+" => param1.add(&param2),
        "-" => param1.subtract(&param2),
        _ => panic!(),
    };
    assert!(wanted.is_equal(result));
}

#[then(regex = "(p|v) = tuple\\((.+), (.+), (.+), (.+)\\)")]
fn check_point(world: &mut TuplesWorld, matches: &[String]) {
    let target = match matches[0].as_str() {
        "p" => &world.point1,
        "v" => &world.vector1,
        _ => panic!(),
    };
    let values: Vec<f64> = matches[1..].into_iter().map(|m| m.parse::<f64>().unwrap()).collect();
    assert!(is_equal_f64(target.x, values[0]));
    assert!(is_equal_f64(target.y, values[1]));
    assert!(is_equal_f64(target.z, values[2]));
    assert!(is_equal_f64(target.w, values[3]));
}

#[derive(Debug, Default, World)]
struct TuplesWorld {
    tuple: rtxch_lib::Tuples,
    other: rtxch_lib::Tuples,
    point1: rtxch_lib::Tuples,
    point2: rtxch_lib::Tuples,
    vector1: rtxch_lib::Tuples,
    vector2: rtxch_lib::Tuples,
}

fn main() {
    futures::executor::block_on(TuplesWorld::run(
        "tests/features/tuples.feature",
    ));
}