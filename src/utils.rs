use std::f64::consts::PI;
use crate::Tuples;
use perlin2d::PerlinNoise2D;

pub const EPSILON: f64 = 0.00001;

pub fn is_equal_f64(a: f64, b: f64) -> bool {
    (a - b).abs() < EPSILON
}

pub fn parse_values_f64(input: &String) -> Vec<f64> {
    input.split(", ").into_iter().map(|m| m.parse::<f64>().unwrap()).collect()
}

pub fn parse_values_u64(input: &String) -> Vec<u64> {
    input.split(", ").map(|m| m.parse::<u64>().unwrap()).collect()
}

pub fn parse_values_usize(input: &String) -> Vec<usize> {
    input.split(", ").map(|m| m.parse::<usize>().unwrap()).collect()
}

pub fn deg_to_rad(n: f64) -> f64 {
    n * PI / 180.0
}

pub fn color_to_256(input: f64) -> u8 {
    let factor = if input > 1.0 { 1.0 } else if input < 0.0 { 0.0 } else { input };
    let output = (factor * 255.0).round();
    output as u8
}

pub fn perlin_noise(point: &Tuples) -> Tuples {
    let octaves = 6;
    let amplitude = 1.0;
    let frequency = 4.0;
    let persistence = 1.0;
    let lacunarity = 2.0;
    let scale = (1.0, 1.0);
    let bias = 0.5;
    let seed = 2352356;

    let noise = PerlinNoise2D::new(octaves, amplitude, frequency, persistence, lacunarity, scale, bias, seed);
    let f = noise.get_noise(point.x, point.z);
    Tuples::point(point.x + 0.2 * f, point.y, point.z + 0.2 * f)
}