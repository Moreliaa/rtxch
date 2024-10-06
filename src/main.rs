extern crate rtxch_lib;

use std::{f64::consts::PI, fs};
use intersections::Shape;
use lights::point_light;
use rtxch_lib::*;
use utils::deg_to_rad;

fn main() {
    let mut camera = Camera::new(500, 500, deg_to_rad(60.0));
    let from = Tuples::point(0.0,1.5, -5.0);
    let to = Tuples::point(0.0,1.0,0.0);
    let up = Tuples::vector(0.0,1.0,0.0);
    camera.transform = Matrix::view_transform(&from, &to, &up);

    let mut world = World::new();
    let floor = Sphere::new();
    Sphere::set_transform(&floor, &Matrix::scale(10.0,0.01,10.0));
    let mut floor_material = Material::material();
    floor_material.color = Tuples::color(1.0,0.9,0.9);
    floor_material.specular = 0.0;
    floor_material.ambient = 0.3;
    Sphere::set_material(&floor, &floor_material);
    world.add_object(floor);

    let left_wall = Sphere::new();
    Sphere::set_transform(&left_wall, 
        &Matrix::transform_from_trs(
            &Matrix::translate(0.0,0.0,5.0),
            &(Matrix::rotate_y(-PI / 4.0) * Matrix::rotate_x(PI / 2.0)),
    &Matrix::scale(10.0,0.01,10.0)
    ));
    Sphere::set_material(&left_wall, &floor_material);
    world.add_object(left_wall);

    let right_wall = Sphere::new();
    Sphere::set_transform(&right_wall, 
        &Matrix::transform_from_trs(
            &Matrix::translate(0.0,0.0,5.0),
            &(Matrix::rotate_y(PI / 4.0) * Matrix::rotate_x(PI / 2.0)),
    &Matrix::scale(10.0,0.01,10.0)
    ));
    Sphere::set_material(&right_wall, &floor_material);
    world.add_object(right_wall);

    let middle = Sphere::new();
    Sphere::set_transform(&middle, &Matrix::translate(-0.5,1.0,0.5));
    let mut middle_material = Material::material();
    middle_material.color = Tuples::color(0.1,1.0,0.5);
    middle_material.diffuse = 0.7;
    middle_material.specular = 0.3;
    Sphere::set_material(&middle, &middle_material);
    world.add_object(middle);

    let right = Sphere::new();
    Sphere::set_transform(&right, 
        &(Matrix::translate(1.5,0.5,-0.5) * Matrix::scale(0.5,0.5,0.5)));
    let mut right_material = Material::material();
    right_material.color = Tuples::color(0.5,1.0,0.1);
    right_material.diffuse = 0.7;
    right_material.specular = 0.3;
    Sphere::set_material(&right, &right_material);
    world.add_object(right);

    // left tood
    let left = Sphere::new();
    Sphere::set_transform(&left, 
        &(Matrix::translate(-1.5,0.33,-0.75) * Matrix::scale(0.33,0.33,0.33)));
    let mut left_material = Material::material();
    left_material.color = Tuples::color(1.0,0.8,0.1);
    left_material.diffuse = 0.7;
    left_material.specular = 0.3;
    Sphere::set_material(&left, &left_material);
    world.add_object(left);

    let light = point_light(
        &Tuples::point(-10.0,10.0,-10.0),
         &Tuples::color(0.5,0.5,0.5)
    );
    world.add_point_light(light);

    let light2 = point_light(
        &Tuples::point(5.0,10.0,-10.0), 
        &Tuples::color(0.5,0.5,0.5)
    );
    world.add_point_light(light2);

    let canvas = render::render(&camera, &world);
    
    println!("Writing ppm...");
    let ppm = canvas.canvas_to_ppm();
    fs::write("./output.ppm", ppm).expect("Failed to write file.");
    
}