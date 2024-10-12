extern crate rtxch_lib;

use std::{f64::consts::PI, fs};
use rtxch_lib::Shape;
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
    let floor = Plane::new();
    floor.borrow_mut().set_transform(&Matrix::new(4));
    let mut floor_material = Material::material();
    floor_material.pattern = SingleColorPattern::new(Tuples::color(1.0,0.9,0.9));
    floor_material.specular = 0.0;
    floor_material.ambient = 0.3;
    floor.borrow_mut().set_material(&floor_material);
    world.add_object(floor);

    let left_wall = Plane::new();
    left_wall.borrow_mut().set_transform(
        &Matrix::transform_from_trs(
            &Matrix::translate(0.0,0.0,5.0),
            &(Matrix::rotate_y(-PI / 4.0) * Matrix::rotate_x(PI / 2.0)),
    &Matrix::scale(1.0,1.0,1.0)
    ));
    left_wall.borrow_mut().set_material(&floor_material);
    world.add_object(left_wall);

    let right_wall = Plane::new();
    right_wall.borrow_mut().set_transform( 
        &Matrix::transform_from_trs(
            &Matrix::translate(0.0,0.0,5.0),
            &(Matrix::rotate_y(PI / 4.0) * Matrix::rotate_x(PI / 2.0)),
            &Matrix::scale(1.0,1.0,1.0)
    ));
    right_wall.borrow_mut().set_material(&floor_material);
    world.add_object(right_wall);

    let middle = Sphere::new();
    middle.borrow_mut().set_transform(&Matrix::translate(-0.5,1.0,0.5));
    let mut middle_material = Material::material();
    middle_material.pattern = StripePattern::new(Tuples::color(0.1,1.0,0.5), Tuples::color(1.0,0.5,0.5));
    middle_material.diffuse = 0.7;
    middle_material.specular = 0.3;
    middle.borrow_mut().set_material(&middle_material);
    world.add_object(middle);

    let right = Sphere::new();
    right.borrow_mut().set_transform(
        &(Matrix::translate(1.5,0.5,-0.5) * Matrix::scale(0.5,0.5,0.5)));
    let mut right_material = Material::material();
    right_material.pattern = SingleColorPattern::new(Tuples::color(0.5,1.0,0.1));
    right_material.diffuse = 0.7;
    right_material.specular = 0.3;
    right.borrow_mut().set_material(&right_material);
    world.add_object(right);

    let left = Sphere::new();
    left.borrow_mut().set_transform( 
        &(Matrix::translate(-1.5,0.33,-0.75) * Matrix::scale(0.33,0.33,0.33)));
    let mut left_material = Material::material();
    left_material.pattern = SingleColorPattern::new(Tuples::color(1.0,0.8,0.1));
    left_material.diffuse = 0.7;
    left_material.specular = 0.3;
    left.borrow_mut().set_material(&left_material);
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