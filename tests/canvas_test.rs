extern crate rtxch_lib;

use rtxch_lib::utils::{parse_values_usize, parse_values_f64};
use rtxch_lib::Tuples;
use std::collections::HashMap;
use cucumber::{given, when, then, World};

#[given(regex = r"(.+) ← canvas\((.+)\)")]
fn create_canvas(world: &mut CanvasWorld, matches: &[String]) {
    let values = parse_values_usize(&matches[1]);
    world.canvas = rtxch_lib::Canvas::new(values[0], values[1]);
}

#[given(regex = r"(.+) ← color\((.+)\)")]
fn create_color(world: &mut CanvasWorld, matches: &[String]) {
    let name = matches[0].clone();
    let values = parse_values_f64(&matches[1]);
    let color = Tuples::color(values[0], values[1], values[2]);
    world.colors.insert(name, color);
}

#[when(regex = r"write_pixel\(c, (.+), (.+), (.+)\)")]
fn write_pixel(world: &mut CanvasWorld, matches: &[String]) {
    let x = matches[0].parse::<usize>().unwrap();
    let y = matches[1].parse::<usize>().unwrap();
    let color = world.colors.get(matches[2].as_str()).unwrap();
    world.canvas.write_pixel(x, y, &color);
}

#[when(regex = r"ppm ← canvas_to_ppm\(c\)")]
fn write_ppm(world: &mut CanvasWorld, _: &[String]) {
    world.ppm = world.canvas.canvas_to_ppm();
}

#[when(regex = r"every pixel of c is set to color\(1, 0.8, 0.6\)")]
fn write_all_pixels(world: &mut CanvasWorld, _: &[String]) {
    let color = Tuples::color(1.0,0.8,0.6);
    world.canvas.clear(&color);
}

#[then(regex = r"lines (.+)-(.+) of ppm are '(.+)'")]
fn check_ppm_lines(world: &mut CanvasWorld, matches: &[String]) {
    let start = matches[0].parse::<usize>().unwrap() - 1;
    let end = matches[1].parse::<usize>().unwrap() - 1;
    let ppm_split: Vec<&str> = world.ppm.split("\n").collect();
    let wanted: Vec<&str> = matches[2].split("\\n").collect();
    assert!(ppm_split.len() > end);
    for i in start..end {
        assert!(ppm_split[i] == wanted[i - start], "'{}' != '{}'", ppm_split[i], wanted[i - start]);
    }
}

#[then(regex = r"ppm ends with a newline character")]
fn check_ppm_ends_in_newline(world: &mut CanvasWorld, _: &[String]) {
    assert!(world.ppm.ends_with("\n"));
}

#[then(regex = r".\.(width|height) = (.+)")]
fn check_width_height(world: &mut CanvasWorld, matches: &[String]) {
    let prop = &matches[0];
    let values = parse_values_usize(&matches[1]);
    let a = match prop.as_str() {
        "width" => world.canvas.width,
        "height" => world.canvas.height,
        _ => panic!(),
    };
    assert!(a == values[0]);
}

#[then(regex = r"every pixel of c is color\((.+)\)")]
fn check_pixels(world: &mut CanvasWorld, matches: &[String]) {
    let values = parse_values_f64(&matches[0]);
    let color = Tuples::color(values[0], values[1], values[2]);
    for pixel in world.canvas.get_pixels() {
        assert!(pixel.is_equal(&color));
    }
}

#[then(regex = r"pixel_at\(c, (.+), (.+)\) = (.+)")]
fn pixel_at(world: &mut CanvasWorld, matches: &[String]) {
    let x = matches[0].parse::<usize>().unwrap();
    let y = matches[1].parse::<usize>().unwrap();
    let color = world.colors.get(matches[2].as_str()).unwrap();
    assert!(color.is_equal(world.canvas.pixel_at(x, y)), "{:?} not equal to {:?}", color, world.canvas.pixel_at(x, y));
}

#[derive(Debug, Default, World)]
struct CanvasWorld {
    canvas: rtxch_lib::Canvas,
    colors: HashMap<String, Tuples>,
    ppm: String,
}

fn main() {
    futures::executor::block_on(CanvasWorld::run(
        "tests/features/canvas.feature",
    ));
}