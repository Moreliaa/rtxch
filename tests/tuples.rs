extern crate rtxch_lib;

use rtxch_lib::test_utils::is_equal_f64;
use cucumber::{given, then, World};

#[given("a <- Tuples(4.3, -4.2, 3.1, 1.0)")]
fn point_tuple(world: &mut TuplesWorld) {
    world.tuples = rtxch_lib::Tuples::new(4.3, -4.2, 3.1, 1.0)
}

#[then("a.x = 4.3")]
fn compare_x(world: &mut TuplesWorld) {
    assert!(is_equal_f64(world.tuples.x, 4.3));
}

#[then("a.y = -4.2")]
fn compare_y(world: &mut TuplesWorld) {
    assert!(is_equal_f64(world.tuples.y, -4.2));
}

#[then("a.z = 3.1")]
fn compare_z(world: &mut TuplesWorld) {
    assert!(is_equal_f64(world.tuples.z, 3.1));
}

#[then("a.w = 1.0")]
fn compare_w_point(world: &mut TuplesWorld) {
    assert!(is_equal_f64(world.tuples.w, 1.0));
}

#[then("a.w = 0.0")]
fn compare_w_vector(world: &mut TuplesWorld) {
    assert!(is_equal_f64(world.tuples.w, 0.0));
}

#[given("a <- Tuples(4.3, -4.2, 3.1, 0.0)")]
fn vector_tuple(world: &mut TuplesWorld) {
    world.tuples = rtxch_lib::Tuples::new(4.3, -4.2, 3.1, 0.0)
}

#[derive(Debug, Default, World)]
pub struct TuplesWorld {
    tuples: rtxch_lib::Tuples
}

fn main() {
    futures::executor::block_on(TuplesWorld::run(
        "tests/features/tuples.feature",
    ));
}