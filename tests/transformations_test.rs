extern crate rtxch_lib;

use std::collections::HashMap;
use std::f64::consts::PI;
use cucumber::{given, then, World};
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

#[then(regex = r"^(.+) \* (.+) = (.)$")]
fn check_result(world: &mut TrafoWorld, matches: &[String]) {
    let m = world.mat.get(&matches[0]).unwrap();
    let t = world.tuple.get(&matches[1]).unwrap();
    let target = world.tuple.get(&matches[2]).unwrap();

    let result = Matrix::mul_tuple(&m, &t);
    assert!(result.is_equal(target), "Result does not match: {:?}", result);
}

#[then(regex = r"(.+) \* (.+) = (point|vector)\((.+)\)")]
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

fn extract(m: &String) -> Vec<f64> {
    let truncated = m.replace(" ", "");
    truncated.split(r"|")
                .filter(|m| m.chars().count() > 0)
                .map(|m| m.parse::<f64>().unwrap())
                .collect()
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