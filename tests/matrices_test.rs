extern crate rtxch_lib;

use rtxch_lib::utils::{parse_values_usize, parse_values_f64};
use rtxch_lib::Tuples;
use std::collections::HashMap;
use cucumber::{given, when, then, World};

// #[given(regex = r"")]
// fn create_canvas(world: &mut MatricesWorld, matches: &[String]) {

// }

// #[when(regex = r"")]
// fn write_pixel(world: &mut MatricesWorld, matches: &[String]) {

// }

// #[then(regex = r"")]
// fn check_ppm_lines(world: &mut MatricesWorld, matches: &[String]) {

// }

#[derive(Debug, Default, World)]
struct MatricesWorld {

}

fn main() {
    futures::executor::block_on(MatricesWorld::run(
        "tests/features/matrices.feature",
    ));
}