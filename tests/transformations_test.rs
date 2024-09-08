extern crate rtxch_lib;

use std::collections::HashMap;
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
            let param = matches[2].parse::<f64>().unwrap();
            world.mat.insert(target, Matrix::rotate_x(param));
        },
        "rotation_y" => {
            let param = matches[2].parse::<f64>().unwrap();
            world.mat.insert(target, Matrix::rotate_y(param));
        },
        "rotation_z" => {
            let param = matches[2].parse::<f64>().unwrap();
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