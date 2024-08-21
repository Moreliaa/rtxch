extern crate rtxch_lib;

use rtxch_lib::utils::is_equal_f64;
use cucumber::{given, when, then, World};

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

#[then(regex = r"^..(.|red|green|blue) = (.+)$")]
fn compare(world: &mut TuplesWorld, matches: &[String]) {
    let prop = match matches[0].as_str() {
        "x" => world.tuple.x,
        "y" => world.tuple.y,
        "z" => world.tuple.z,
        "w" => world.tuple.w,
        "red" => world.color1.x,
        "green" => world.color1.y,
        "blue" => world.color1.z,
        _ => panic!()
    };
    let value = matches[1].parse::<f64>().unwrap();
    assert!(is_equal_f64(prop, value), "{} {}", prop, value);
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

#[given(regex = r"(.+) ← (point|vector|color)\((.+), (.+), (.+)\)")]
fn create_vector_or_point(world: &mut TuplesWorld, matches: &[String]) {
    let values: Vec<f64> = matches[2..].into_iter().map(|m| m.parse::<f64>().unwrap()).collect();
    let tuple = match matches[1].as_str() {
        "point" => rtxch_lib::Tuples::point(values[0], values[1], values[2]),
        "vector" => rtxch_lib::Tuples::vector(values[0], values[1], values[2]),
        "color" => rtxch_lib::Tuples::color(values[0], values[1], values[2]),
        _ => panic!(),
    };
    match matches[0].as_str() {
        "zero" => world.zero = tuple,
        "a" | "a1" => world.tuple = tuple,
        "a2" => world.other = tuple,
        "p" | "p1" => world.point1 = tuple,
        "p2" => world.point2 = tuple,
        "v" | "v1" => world.vector1 = tuple,  
        "v2" => world.vector2 = tuple,
        "c" | "c1" => world.color1 = tuple,
        "c2" => world.color2 = tuple,
        _ => panic!(),        
    };
}

#[then(regex = r"magnitude\((.+)\) = (.+)")]
fn mag(world: &mut TuplesWorld, matches: &[String]) {
    let mut param1 = match matches[0].as_str() {
        "norm" => world.norm.clone(),
        "zero" => world.zero.clone(),
        "a" | "a1" => world.tuple.clone(),
        "a2" => world.other.clone(),
        "p" | "p1" => world.point1.clone(),
        "p2" => world.point2.clone(),
        "v" | "v1" => world.vector1.clone(),
        "v2" => world.vector2.clone(),
        _ => panic!(),
    };
    let val = matches[1].as_str();
    let wanted = if val.chars().nth(0) == Some('√') {
        // start at index 3 due to multibyte character √
        let number = val.to_string()[3..].parse::<f64>().unwrap();
        number.sqrt()
    } else {
        val.to_string().parse::<f64>().unwrap()
    };
    let result = param1.magnitude();
    assert!(is_equal_f64(result, wanted));
}

#[then(regex = r"normalize\((.+)\) = vector\((.+)\)")]
fn norm(world: &mut TuplesWorld, matches: &[String]) {
    let mut param1 = match matches[0].as_str() {
        "zero" => world.zero.clone(),
        "a" | "a1" => world.tuple.clone(),
        "a2" => world.other.clone(),
        "p" | "p1" => world.point1.clone(),
        "p2" => world.point2.clone(),
        "v" | "v1" => world.vector1.clone(),
        "v2" => world.vector2.clone(),
        _ => panic!(),
    };
    let values: Vec<f64> = matches[1].split(", ").into_iter().map(|m| m.parse::<f64>().unwrap()).collect();
    let wanted = rtxch_lib::Tuples::vector(values[0], values[1], values[2]);
    let result = param1.normalize();
    assert!(result.is_equal(&wanted));
}

#[when("norm ← normalize(v)")]
fn normwhen(world: &mut TuplesWorld) {
    world.norm = world.vector1.normalize().clone();
}

#[then("dot(v1, v2) = 20")]
fn dot(world: &mut TuplesWorld) {
    let result = rtxch_lib::Tuples::dot(&world.vector1, &world.vector2);
    assert!(is_equal_f64(result, 20.0));
}

#[then(regex = r"cross\((.+), (.+)\) = vector\((.+)\)")]
fn cross(world: &mut TuplesWorld, matches: &[String]) {
    let param1 = match matches[0].as_str() {
        "v" | "v1" => world.vector1.clone(),
        "v2" => world.vector2.clone(),
        _ => panic!(),
    };

    let param2 = match matches[1].as_str() {
        "v" | "v1" => world.vector1.clone(),
        "v2" => world.vector2.clone(),
        _ => panic!(),
    };

    let values: Vec<f64> = matches[2].split(", ").into_iter().map(|m| m.parse::<f64>().unwrap()).collect();
    let wanted = rtxch_lib::Tuples::vector(values[0], values[1], values[2]);
    let result = rtxch_lib::Tuples::cross(&param1, &param2);
    assert!(result.is_equal(&wanted));
}

// Operation
#[then(regex = r"^(.+) (.+) (.+) = (vector|point|tuple|color)\((.+)\)$")]
fn subtract_p_v(world: &mut TuplesWorld, matches: &[String]) {
    let mut param1 = match matches[0].as_str() {
        "zero" => world.zero.clone(),
        "a" | "a1" => world.tuple.clone(),
        "a2" => world.other.clone(),
        "p" | "p1" => world.point1.clone(),
        "p2" => world.point2.clone(),
        "v" | "v1" => world.vector1.clone(),
        "v2" => world.vector2.clone(),
        "c" | "c1" => world.color1.clone(),
        "c2" => world.color2.clone(),
        _ => panic!(),
    };

    let op = matches[1].as_str();

    let param2 = match matches[2].as_str() {
        "zero" => Some(world.zero.clone()),
        "a" | "a1" => Some(world.tuple.clone()),
        "a2" => Some(world.other.clone()),
        "p" | "p1" => Some(world.point1.clone()),
        "p2" => Some(world.point2.clone()),
        "v" | "v1" => Some(world.vector1.clone()),
        "v2" => Some(world.vector2.clone()),
        "c" | "c1" => Some(world.color1.clone()),
        "c2" => Some(world.color2.clone()),
        _ => None,
    };

    let values: Vec<f64> = matches[4].split(", ").into_iter().map(|m| m.parse::<f64>().unwrap()).collect();
    let wanted = match matches[3].as_str() {
        "point" => rtxch_lib::Tuples::point(values[0], values[1], values[2]),
        "vector" => rtxch_lib::Tuples::vector(values[0], values[1], values[2]),
        "tuple" => rtxch_lib::Tuples::new(values[0], values[1], values[2], values[3]),
        "color" => rtxch_lib::Tuples::color(values[0], values[1], values[2]),
        _ => panic!(),
    };
    let result = match op {
        "+" => param1.add(&param2.unwrap()),
        "-" => param1.subtract(&param2.unwrap()),
        "*" => {
            if let Some(other_value) = param2 {
                param1.multiply(&other_value)
            } else {
                param1.scale(matches[2].as_str().parse::<f64>().unwrap())
            }
            
        },
        "/" => param1.scale(1.0 / matches[2].as_str().parse::<f64>().unwrap()),
        _ => panic!(),
    };
    assert!(wanted.is_equal(result));
}

#[then(regex = r"^(-?\w+\d?) = tuple\((.+), (.+), (.+), (.+)\)")]
fn check_point(world: &mut TuplesWorld, matches: &[String]) {
    let target = match matches[0].as_str() {
        "-a" => {
            let mut val = world.tuple.clone();
            val.negate().clone()
        },
        "p" => world.point1.clone(),
        "v" => world.vector1.clone(),
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
    zero: rtxch_lib::Tuples,
    norm: rtxch_lib::Tuples,
    color1: rtxch_lib::Tuples,
    color2: rtxch_lib::Tuples,
}

fn main() {
    futures::executor::block_on(TuplesWorld::run(
        "tests/features/tuples.feature",
    ));
}