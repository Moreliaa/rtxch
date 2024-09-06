extern crate rtxch_lib;

use rtxch_lib::matrices::Matrix;
use std::collections::HashMap;
use cucumber::{given, when, then, World};

#[given(regex = r"the following 4x4 matrix (.+): (.+)")]
fn create_4x4(world: &mut MatricesWorld, matches: &[String]) {
    let truncated = matches[1].replace(" ", "");
    let values: Vec<f64> = truncated.split(r"|")
                .filter(|m| m.chars().count() > 0)
                .map(|m| m.parse::<f64>().unwrap())
                .collect();
    world.mat4.insert(matches[0].clone(), rtxch_lib::Mat4::from_values(&values));
}

#[given(regex = r"the following 3x3 matrix (.+): (.+)")]
fn create_3x3(world: &mut MatricesWorld, matches: &[String]) {
    let truncated = matches[1].replace(" ", "");
    let values: Vec<f64> = truncated.split(r"|")
                .filter(|m| m.chars().count() > 0)
                .map(|m| m.parse::<f64>().unwrap())
                .collect();
    world.mat3.insert(matches[0].clone(), rtxch_lib::Mat3::from_values(&values));
}

#[given(regex = r"the following 2x2 matrix (.+): (.+)")]
fn create_2x2(world: &mut MatricesWorld, matches: &[String]) {
    let truncated = matches[1].replace(" ", "");
    let values: Vec<f64> = truncated.split(r"|")
                .filter(|m| m.chars().count() > 0)
                .map(|m| m.parse::<f64>().unwrap())
                .collect();
    world.mat2.insert(matches[0].clone(), rtxch_lib::Mat2::from_values(&values));
}


#[then(regex = r"(.+)\[(.+),(.+)\] = (.+)")]
fn check_field(world: &mut MatricesWorld, matches: &[String]) {
    let key = matches[0].clone();
    let row = matches[1].parse::<usize>().unwrap();
    let col = matches[2].parse::<usize>().unwrap();
    let val = matches[3].parse::<f64>().unwrap();
    let m2 = world.mat2.get(&key);
    if let Some(m) = m2 {
        assert!(m.get(row, col) == val, "{}", m.get(row, col));
    }
    let m3 = world.mat3.get(&key);
    if let Some(m) = m3 {
        assert!(m.get(row, col) == val, "{}", m.get(row, col));
    }
    let m4 = world.mat4.get(&key);
    if let Some(m) = m4 {
        assert!(m.get(row, col) == val, "{}", m.get(row, col));
    }
}

#[derive(Debug, Default, World)]
struct MatricesWorld {
    mat2: HashMap<String, rtxch_lib::Mat2>,
    mat3: HashMap<String, rtxch_lib::Mat3>,
    mat4: HashMap<String, rtxch_lib::Mat4>,
}

fn main() {
    futures::executor::block_on(MatricesWorld::run(
        "tests/features/matrices.feature",
    ));
}