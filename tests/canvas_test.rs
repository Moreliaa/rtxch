extern crate rtxch_lib;

use rtxch_lib::utils::parse_values_u64;
use cucumber::{given, when, then, World};

#[given(regex = r"(.+) ← canvas\((.+)\)")]
fn create_canvas(world: &mut CanvasWorld, matches: &[String]) {
    println!("{:?}", matches);
    let values = parse_values_u64(&matches[1]);
    world.canvas = rtxch_lib::Canvas::new(values[0], values[1]);
}

#[derive(Debug, Default, World)]
struct CanvasWorld {
    canvas: rtxch_lib::Canvas,
}

fn main() {
    futures::executor::block_on(CanvasWorld::run(
        "tests/features/canvas.feature",
    ));
}