extern crate rtxch_lib;

use std::collections::HashMap;
use std::f64::consts::PI;
use cucumber::{given, when, then, World};
use rtxch_lib::utils::parse_values_f64;
use rtxch_lib::Matrix;
use rtxch_lib::Tuples;

#[given(regex = r"(.+) ← (.+)\((.+)\)")]
fn create_matrix(world: &mut TrafoWorld, matches: &[String]) {
    let target = matches[0].to_string();
    let func = matches[1].to_string();
    
    match func.as_str() {
        "translation" => {
            let params = parse_values_f64(&matches[2]);
            let (x,y,z) = (params[0],params[1],params[2]);
            world.mat.insert(target, Matrix::translate(x, y, z));
        },
        "rotation_x" => {
            let mut param = matches[2].chars().nth(4).unwrap().to_string().parse::<f64>().unwrap();
            param = PI / param;
            world.mat.insert(target, Matrix::rotate_x(param));
        },
        "rotation_y" => {
            let mut param = matches[2].chars().nth(4).unwrap().to_string().parse::<f64>().unwrap();
            param = PI / param;
            world.mat.insert(target, Matrix::rotate_y(param));
        },
        "rotation_z" => {
            let mut param = matches[2].chars().nth(4).unwrap().to_string().parse::<f64>().unwrap();
            param = PI / param;
            world.mat.insert(target, Matrix::rotate_z(param));
        },
        "point" => {
            let params = parse_values_f64(&matches[2]);
            let (x,y,z) = (params[0],params[1],params[2]);
            world.tuple.insert(target, Tuples::point(x, y, z));
        },
        "vector" => {
            let params = parse_values_f64(&matches[2]);
            let (x,y,z) = (params[0],params[1],params[2]);
            world.tuple.insert(target, Tuples::vector(x, y, z));
        },
        "scaling" => {
            let params = parse_values_f64(&matches[2]);
            let (x,y,z) = (params[0],params[1],params[2]);
            world.mat.insert(target, Matrix::scale(x, y, z));
        },
        "shearing" => {
            let params = parse_values_f64(&matches[2]);
            let (x_y,x_z,y_x, y_z,z_x,z_y) =
                (params[0],params[1],params[2],params[3],params[4],params[5]);
            world.mat.insert(target, Matrix::shear(x_y,x_z,y_x, y_z,z_x,z_y));
        },
        "inverse" => {
            let key = matches[2].clone();
            let m = world.mat.get(&key).unwrap();
            world.mat.insert(target, Matrix::inverse(m).unwrap());
        },
        _ => panic!("'{func}' not implemented"),
    };
}

#[when(regex = r"^(.+) ← (.+) \* ([a-z]|[a-z][0-9])$")]
fn mul_tuple(world: &mut TrafoWorld, matches: &[String]) {
    let m = world.mat.get(&matches[1]).unwrap();
    let t = world.tuple.get(&matches[2]).unwrap();
    let target = matches[0].clone();

    let result = m * t;
    world.tuple.insert(target, result);
}

#[when(regex = r"^(.+) ← (.+) \* (.+) \* (.+)$")]
fn mul_mat_chain(world: &mut TrafoWorld, matches: &[String]) {
    let key_out = matches[0].clone();
    let c = world.mat.get(&matches[1]).unwrap();
    let b = world.mat.get(&matches[2]).unwrap();
    let a = world.mat.get(&matches[3]).unwrap();
    let out = c * b * a;
    world.mat.insert(key_out, out);
}

#[when(regex = r"(.+) ← view_transform\((from), (to), (up)\)$")]
fn view_transform(world: &mut TrafoWorld, matches: &[String]) {
    let key_out = matches[0].clone();
    let from = world.tuple.get(&matches[1]).unwrap();
    let to = world.tuple.get(&matches[2]).unwrap();
    let up = world.tuple.get(&matches[3]).unwrap();
    let out = Matrix::view_transform(from, to, up);
    world.mat.insert(key_out, out);
}

#[then(regex = r"^([a-z]|[a-z][0-9]) = point\((.+)\)$")]
fn check_point(world: &mut TrafoWorld, matches: &[String]) {
    let t = world.tuple.get(&matches[0]).unwrap();
    let values = parse_values_f64(&matches[1]);
    let wanted = Tuples::point(values[0],values[1],values[2]);
    assert!(t.is_equal(&wanted), "Result does not match: {:?}", t);
}

#[then(regex = r"^(.+) \* (.+) = (.)$")]
fn check_result(world: &mut TrafoWorld, matches: &[String]) {
    let m = world.mat.get(&matches[0]).unwrap();
    let t = world.tuple.get(&matches[1]).unwrap();
    let target = world.tuple.get(&matches[2]).unwrap();

    let result = Matrix::mul_tuple(&m, &t);
    assert!(result.is_equal(target), "Result does not match: {:?}", result);
}

#[then(regex = r"^([a-zA-Z]+) \* ([a-z]|[a-z][0-9]) = (point|vector)\((.+)\)$")]
fn check_result_fn(world: &mut TrafoWorld, matches: &[String]) {
    let m = world.mat.get(&matches[0]).unwrap();
    let t = world.tuple.get(&matches[1]).unwrap();
    let values = parse_values_f64(&matches[3]);
    let target = match matches[2].as_str() {
        "point" => { Tuples::point(values[0], values[1], values[2]) },
        "vector" => { Tuples::vector(values[0], values[1], values[2]) },
        _ => panic!(),
    };

    let result = Matrix::mul_tuple(&m, &t);
    assert!(result.is_equal(&target), "Result does not match: {:?}", result);
}

#[then(regex = r"^C \* B \* A \* p = point\((.+)\)$")]
fn check_result_chain(world: &mut TrafoWorld, matches: &[String]) {
    let c = world.mat.get(&String::from("C")).unwrap();
    let b = world.mat.get(&String::from("B")).unwrap();
    let a = world.mat.get(&String::from("A")).unwrap();
    let p = world.tuple.get(&String::from("p")).unwrap();
    let result = c * b * a * p;
    let values = parse_values_f64(&matches[0]);
    let target = Tuples::point(values[0], values[1], values[2]);

    assert!(result.is_equal(&target), "Result does not match: {:?}", result);
}

#[then(regex = r"^t = (.+)$")]
fn check_view_transform(world: &mut TrafoWorld, matches: &[String]) {
    let t = world.mat.get(&"t".to_string()).unwrap();
    let other = match matches[0].as_str() {
        "identity_matrix" => {
            Matrix::new(4)
        },
        "scaling(-1, 1, -1)" => {
            Matrix::scale(-1.0,1.0,-1.0)
        },
        "translation(0, 0, -8)" => {
            Matrix::translate(0.0,0.0,-8.0)
        },
        _ => panic!(),
    };
    assert!(t.is_equal(&other));
}

#[then(regex = r"t is the following 4x4 matrix:(.+)")]
fn check_4x4(world: &mut TrafoWorld, matches: &[String]) {
    let t = world.mat.get(&"t".to_string()).unwrap();
    let values: Vec<f64> = matches[0].split("|").map(|a| a.parse::<f64>().unwrap()).collect();
    let other_mat = Matrix::from_values(&values);
    
    assert!(t.is_equal(&other_mat));
}

#[derive(Debug, Default, World)]
struct TrafoWorld {
    mat: HashMap<String, rtxch_lib::Matrix>,
    tuple: HashMap<String, rtxch_lib::Tuples>
}

fn main() {
    futures::executor::block_on(TrafoWorld::run(
        "tests/features/transformations.feature",
    ));
}