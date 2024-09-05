extern crate rtxch_lib;

use rtxch_lib::matrices::Matrix;
use rtxch_lib::utils::{parse_values_usize, parse_values_f64};
use rtxch_lib::Tuples;
use std::collections::HashMap;
use cucumber::{given, when, then, World};

#[given(regex = r"the following 4x4 matrix M: (.+)")]
fn create_4x4(world: &mut MatricesWorld, matches: &[String]) {
    let values: Vec<f64> = matches[0].split(r"[^\.\d-]+")
                .filter(|m| m.len() > 0)
                .map(|m| m.parse::<f64>().unwrap()).collect();
    world.mat2 = rtxch_lib::Mat2::from_values(&values);
}

#[given(regex = r"the following 2x2 matrix M: (.+)")]
fn create_thing(world: &mut MatricesWorld, matches: &[String]) {
    let truncated = matches[0].replace(" ", "");
    let values: Vec<f64> = truncated.split(r"|")
                .filter(|m| m.chars().count() > 0)
                .map(|m| m.parse::<f64>().unwrap())
                .collect();
    world.mat2 = rtxch_lib::Mat2::from_values(&values);
}


#[then(regex = r"M\[(.+),(.+)\] = (.+)")]
fn check_field(world: &mut MatricesWorld, matches: &[String]) {
    let row = matches[0].parse::<usize>().unwrap();
    let col = matches[1].parse::<usize>().unwrap();
    let val = matches[2].parse::<f64>().unwrap();
    assert!(world.mat2.get(row, col) == val, "{}", world.mat2.get(row, col));
}

#[derive(Debug, Default, World)]
struct MatricesWorld {
    mat2: rtxch_lib::Mat2,
}

fn main() {
    futures::executor::block_on(MatricesWorld::run(
        "tests/features/matrices.feature",
    ));
}