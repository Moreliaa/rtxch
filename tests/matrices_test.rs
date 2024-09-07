extern crate rtxch_lib;

use std::collections::HashMap;
use cucumber::{given, then, World};
use rtxch_lib::utils::parse_values_f64;

#[given(regex = r"the following .+ matrix (.+): (.+)")]
fn create_matrix(world: &mut MatricesWorld, matches: &[String]) {
    let values = extract(&matches[1]);
    world.mat.insert(matches[0].clone(), rtxch_lib::Matrix::from_values(&values));
}

#[given(regex = r"A ← transpose\(identity_matrix\)")]
fn transpose_ident(world: &mut MatricesWorld, _: &[String]) {
    let ident = rtxch_lib::Matrix::new(4);
    world.mat.insert("A".to_string(), rtxch_lib::Matrix::transpose(&ident));
}

#[given(regex = r"B ← inverse\(A\)")]
fn inv(world: &mut MatricesWorld, _: &[String]) {
    let a = world.mat.get(&"A".to_string()).unwrap();
    let b = rtxch_lib::Matrix::inverse(&a).unwrap();
    world.mat.insert("B".to_string(), b);
}

#[given(regex = "(.+) ← tuple\\((.+)\\)")]
fn point_tuple(world: &mut MatricesWorld, matches: &[String]) {
    let values: Vec<f64> = parse_values_f64(&matches[1]);
    let key = matches[0].clone();
    let tuple = rtxch_lib::Tuples::new(values[0], values[1], values[2], values[3]);
    world.tuple.insert(key, tuple);
}

#[given(regex = r"C ← A \* B")]
fn given_mul_mat(world: &mut MatricesWorld, _: &[String]) {
    let a = world.mat.get(&"A".to_string()).unwrap();
    let b = world.mat.get(&"B".to_string()).unwrap();
    let c = rtxch_lib::Matrix::mul(&a, &b);
    world.mat.insert("C".to_string(), c);
}

#[given(regex = r"(.+) ← submatrix\((.+)\)")]
fn submatrix(world: &mut MatricesWorld, matches: &[String]) {
    let key = matches[0].clone();
    let params: Vec<&str> = matches[1].split(", ").collect();
    let param_key = params[0].to_string();
    let param_row = params[1].parse::<usize>().unwrap();
    let param_col = params[2].parse::<usize>().unwrap();
    let a = world.mat.get(&param_key).unwrap();
    let b = rtxch_lib::Matrix::submatrix(a, param_row, param_col);
    world.mat.insert(key, b);
}

#[then(regex = r"(.+)\[(.+),(.+)\] = (.+)")]
fn check_field(world: &mut MatricesWorld, matches: &[String]) {
    let key = matches[0].clone();
    let row = matches[1].parse::<usize>().unwrap();
    let col = matches[2].parse::<usize>().unwrap();
    let val = matches[3].parse::<f64>().unwrap();
    let m = world.mat.get(&key).unwrap();
    assert!(rtxch_lib::utils::is_equal_f64(m.get(row, col), val), "{}", m.get(row, col));
}

#[then(regex = r"^(.) (=|!=) (.)$")]
fn check_eq(world: &mut MatricesWorld, matches: &[String]) {
    let k1 = matches[0].clone();
    let op = matches[1].as_str();
    let k2 = matches[2].clone();
    let m1 = world.mat.get(&k1).unwrap();
    let m2 = world.mat.get(&k2).unwrap();
    match op {
        "=" => assert!(m1.is_equal(m2)),
        "!=" => assert!(!m1.is_equal(m2)),
        _ => panic!(),
    }
}

#[then(regex = r"A = identity_matrix")]
fn check_ident(world: &mut MatricesWorld, _: &[String]) {
    assert!(world.mat.get(&"A".to_string()).unwrap().is_equal(&rtxch_lib::Matrix::new(4)));
}

#[then(regex = r"A \* (B|identity_matrix) is the following 4x4 matrix: (.+)")]
fn check_mul(world: &mut MatricesWorld, matches: &[String]) {
    let values = extract(&matches[1]);
    let wanted = rtxch_lib::Matrix::from_values(&values);
    let m1 = world.mat.get("A").unwrap();
    let m2 = match matches[0].as_str() {
        "B" => world.mat.get("B").unwrap(),
        "identity_matrix" => &rtxch_lib::Matrix::new(m1.dim()),
        _ => panic!(),
    };
    
    let out = rtxch_lib::Matrix::mul(m1, m2);
    assert!(out.is_equal(&wanted));
}

#[then(regex = r"^(.) is the following 4x4 matrix: (.+)")]
fn check_mat(world: &mut MatricesWorld, matches: &[String]) {
    let key = matches[0].clone();
    let out = world.mat.get(&key).unwrap();
    let values = extract(&matches[1]);
    let wanted = rtxch_lib::Matrix::from_values(&values);
    assert!(out.is_equal(&wanted), "{:?}", out);    
}

#[then(regex = r"A \* (B|identity_matrix) = A")]
fn check_mul_a(world: &mut MatricesWorld, matches: &[String]) {
    let m1 = world.mat.get("A").unwrap();
    let m2 = match matches[0].as_str() {
        "B" => world.mat.get("B").unwrap(),
        "identity_matrix" => &rtxch_lib::Matrix::new(m1.dim()),
        _ => panic!(),
    };
    
    let out = rtxch_lib::Matrix::mul(m1, m2);
    assert!(out.is_equal(&m1));
}

#[then(regex = r"identity_matrix \* a = a")]
fn check_mul_ident_a(world: &mut MatricesWorld, _: &[String]) {
    let a = world.tuple.get("a").unwrap();
    let m = rtxch_lib::Matrix::new(4);
    
    let out = rtxch_lib::Matrix::mul_tuple(&m, a);
    assert!(out.is_equal(&a));
}

#[then(regex = r"transpose\(A\) is the following 4x4 matrix: (.+)")]
fn transpose(world: &mut MatricesWorld, matches: &[String]) {
    let values = extract(&matches[0]);
    let wanted = rtxch_lib::Matrix::from_values(&values);
    let m = world.mat.get("A").unwrap();
    let out = rtxch_lib::Matrix::transpose(&m);

    assert!(out.is_equal(&wanted));
}

#[then(regex = r"A \* b = tuple\((.+)\)")]
fn check_mul_tuple(world: &mut MatricesWorld, matches: &[String]) {
    let values: Vec<f64> = parse_values_f64(&matches[0]);
    let wanted = rtxch_lib::Tuples::new(values[0], values[1], values[2], values[3]);
    let m = world.mat.get("A").unwrap();
    let t = world.tuple.get("b").unwrap();
    
    let out = rtxch_lib::Matrix::mul_tuple(m, t);
    assert!(out.is_equal(&wanted));
}

#[then(regex = r"(.+)\((.+)\) = (.+)")]
fn check_function(world: &mut MatricesWorld, matches: &[String]) {
    let fn_name = matches[0].as_str();
    let params: Vec<&str> = matches[1].split(", ").collect();

    match fn_name {
        "determinant" => {
            let wanted = matches[2].parse::<f64>().unwrap();
            let m = world.mat.get(params[0]).unwrap();
            let det = rtxch_lib::Matrix::det(m);
            assert!(det == wanted);
        },
        "minor" => {
            let wanted = matches[2].parse::<f64>().unwrap();
            let m = world.mat.get(params[0]).unwrap();
            let row = params[1].parse::<usize>().unwrap();
            let col = params[2].parse::<usize>().unwrap();
            let minor = rtxch_lib::Matrix::minor(&m, row, col);
            assert!(minor == wanted);
        },
        "cofactor" => {
            let wanted = matches[2].parse::<f64>().unwrap();
            let m = world.mat.get(params[0]).unwrap();
            let row = params[1].parse::<usize>().unwrap();
            let col = params[2].parse::<usize>().unwrap();
            let cofactor = rtxch_lib::Matrix::cofactor(&m, row, col);
            assert!(cofactor == wanted);
        },
        "C * inverse" => {
            let c = world.mat.get(&"C".to_string()).unwrap();
            let b = world.mat.get(&"B".to_string()).unwrap();
            let b_inv = rtxch_lib::Matrix::inverse(&b).unwrap();
            let out = rtxch_lib::Matrix::mul(&c, &b_inv);
            let a = world.mat.get(&"A".to_string()).unwrap();
            assert!(out.is_equal(&a));
        },
        _ => panic!("fn {fn_name} not defined"),
    }
}

#[then(regex = r"submatrix\((.+)\) is the following .+ matrix: (.+)")]
fn check_submatrix(world: &mut MatricesWorld, matches: &[String]) {
    let params: Vec<&str> = matches[0].split(", ").collect();
    let values = extract(&matches[1]);
    let wanted = rtxch_lib::Matrix::from_values(&values);
    let m = world.mat.get(params[0]).unwrap();
    let row = params[1].parse::<usize>().unwrap();
    let col = params[2].parse::<usize>().unwrap();
    assert!(rtxch_lib::Matrix::submatrix(m, row, col).is_equal(&wanted));
}

#[then(regex = r"A (is|is not) invertible")]
fn check_invertible(world: &mut MatricesWorld, matches: &[String]) {
    let m = world.mat.get(&"A".to_string()).unwrap();
    let result = rtxch_lib::Matrix::is_invertible(&m);
    match matches[0].as_str() {
        "is" => {
            assert!(result);
        },
        "is not" => {
            assert!(!result);
        },
        _ => panic!(),
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
struct MatricesWorld {
    mat: HashMap<String, rtxch_lib::Matrix>,
    tuple: HashMap<String, rtxch_lib::Tuples>
}

fn main() {
    futures::executor::block_on(MatricesWorld::run(
        "tests/features/matrices.feature",
    ));
}