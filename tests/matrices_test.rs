extern crate rtxch_lib;

use std::collections::HashMap;
use cucumber::{given, when, then, World};

#[given(regex = r"the following .+ matrix (.+): (.+)")]
fn create_matrix(world: &mut MatricesWorld, matches: &[String]) {
    let truncated = matches[1].replace(" ", "");
    let values: Vec<f64> = truncated.split(r"|")
                .filter(|m| m.chars().count() > 0)
                .map(|m| m.parse::<f64>().unwrap())
                .collect();
    world.mat.insert(matches[0].clone(), rtxch_lib::Matrix::from_values(&values));
}


#[then(regex = r"(.+)\[(.+),(.+)\] = (.+)")]
fn check_field(world: &mut MatricesWorld, matches: &[String]) {
    let key = matches[0].clone();
    let row = matches[1].parse::<usize>().unwrap();
    let col = matches[2].parse::<usize>().unwrap();
    let val = matches[3].parse::<f64>().unwrap();
    let m = world.mat.get(&key).unwrap();
    assert!(m.get(row, col) == val, "{}", m.get(row, col));
}

#[derive(Debug, Default, World)]
struct MatricesWorld {
    mat: HashMap<String, rtxch_lib::Matrix>,
}

fn main() {
    futures::executor::block_on(MatricesWorld::run(
        "tests/features/matrices.feature",
    ));
}