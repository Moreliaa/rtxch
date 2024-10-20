extern crate rtxch_lib;

use std::{f64::consts::PI, fs};
use rtxch_lib::Shape;
use lights::point_light;
use rtxch_lib::*;
use utils::deg_to_rad;

fn main() {
    let mut camera = Camera::new(500, 500, deg_to_rad(60.0));
    let from = Tuples::point(0.0,0.1, -5.0);
    let to = Tuples::point(0.0,3.0,0.0);
    let up = Tuples::vector(0.0,1.0,0.0);
    camera.transform = Matrix::view_transform(&from, &to, &up);

    let mut world = World::new();

    let floor = Plane::new();
    floor.borrow_mut().set_transform(&Matrix::new(4));
    let mut floor_material = Material::material();
    let sub_pattern1 = CheckersPattern::new(Tuples::color(0.8,0.0,0.0), Tuples::color(0.8,0.8,0.0));
    sub_pattern1.borrow_mut().set_transform(
        Matrix::scale(0.5,0.5,0.5)
    );
    let sub_pattern2 = CheckersPattern::new(Tuples::color(0.0,0.0,1.0), Tuples::color(0.8,0.4,0.3));
    sub_pattern2.borrow_mut().set_transform(
        Matrix::scale(0.25,0.25,0.25)
    );
    floor_material.pattern = NestedCheckersPattern::new(
        sub_pattern1,
        sub_pattern2
    );
    floor_material.pattern.borrow_mut().set_transform(
        Matrix::scale(0.5,0.5,0.5)
    );
    floor_material.specular = 0.0;
    floor_material.ambient = 0.3;
    floor.borrow_mut().set_material(&floor_material);
    world.add_object(floor);

    /*let behind_camera_wall_left = Plane::new();
    behind_camera_wall_left.borrow_mut().set_transform(
        &Matrix::transform_from_trs(
            &Matrix::translate(0.0,0.0,-20.0),
            &(Matrix::rotate_y(-PI / 4.0) * Matrix::rotate_x(PI / 2.0)),
    &Matrix::scale(1.0,1.0,1.0)
    ));
    behind_camera_wall_left.borrow_mut().set_material(&floor_material);
    world.add_object(behind_camera_wall_left);

    let behind_camera_wall_right = Plane::new();
    behind_camera_wall_right.borrow_mut().set_transform( 
        &Matrix::transform_from_trs(
            &Matrix::translate(0.0,0.0,-20.0),
            &(Matrix::rotate_y(PI / 4.0) * Matrix::rotate_x(PI / 2.0)),
            &Matrix::scale(1.0,1.0,1.0)
    ));
    behind_camera_wall_right.borrow_mut().set_material(&floor_material);
    world.add_object(behind_camera_wall_right);*/


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

    let middle = Cube::new();
    middle.borrow_mut().set_transform(&Matrix::translate(-0.5,1.0,0.5));
    let mut middle_material = Material::material();
    let sub_pattern1 = StripePattern::new(Tuples::color(0.1,1.0,0.5), Tuples::color(1.0,0.5,0.5));
    let sub_pattern2 = StripePattern::new(Tuples::color(0.1,1.0,0.5), Tuples::color(1.0,0.5,0.5));
    sub_pattern2.borrow_mut().set_transform(
        Matrix::transform_from_trs(
            &Matrix::translate(0.0,0.0,0.0),
             &Matrix::rotate_y(PI / 3.0),
            &Matrix::scale(1.0,1.0,1.0)
    ));

    middle_material.pattern = PerturbedPattern::new(BlendedPattern::new(sub_pattern1, sub_pattern2));
    middle_material.pattern.borrow_mut().set_transform(
        Matrix::transform_from_trs(
            &Matrix::translate(-0.5,1.0,0.5),
            &(Matrix::rotate_y(PI / 4.0) * Matrix::rotate_x(PI / 2.0)),
            &Matrix::scale(0.2,0.2,0.2)
    ));
    middle_material.diffuse = 0.7;
    middle_material.specular = 0.3;
    middle_material.reflective = 0.2;
    middle.borrow_mut().set_material(&middle_material);
    //world.add_object(middle);

    let right = Cube::new();
    right.borrow_mut().set_transform(
        &(Matrix::translate(1.5,0.5,-0.5) * Matrix::rotate_x(deg_to_rad(20.0)) * Matrix::rotate_y(deg_to_rad(30.0)) * Matrix::rotate_z(deg_to_rad(50.0)) * Matrix::scale(1.2,1.2,1.2)));
    let mut right_material = Material::material();
    right_material.pattern = SingleColorPattern::new(Tuples::color(0.1, 0.1, 0.1));
    right_material.diffuse = 0.7;
    right_material.reflective = 0.7;
    right_material.specular = 0.3;
    right.borrow_mut().set_material(&right_material);
    //world.add_object(right);

    let left = Cone::new_limited(4.0,7.0, true);
    left.borrow_mut().set_transform( 
        &(Matrix::translate(-0.0,0.33,-0.75) * Matrix::scale(0.33,0.33,0.33)));
    let mut left_material = Material::material();
    left_material.pattern = RingPattern::new(Tuples::color(1.0,1.0,1.0), Tuples::color(0.2,0.8,0.1));
    left_material.pattern.borrow_mut().set_transform(Matrix::translate(-1.5,0.33,-0.75) * Matrix::scale(0.33,0.33,0.33));
    left_material.diffuse = 0.7;
    left_material.specular = 0.3;
    left.borrow_mut().set_material(&left_material);
    world.add_object(left);

    let light = point_light(
        &Tuples::point(-10.0,10.0,-10.0),
         &Tuples::color(1.0,1.0,1.0)
    );
    world.add_point_light(light);
 
    /*let light2 = point_light(
        &Tuples::point(5.0,10.0,-10.0), 
        &Tuples::color(0.5,0.5,0.5)
    );
    world.add_point_light(light2);*/

    let canvas = render::render(&camera, &world);
    
    println!("Writing ppm...");
    let ppm = canvas.canvas_to_ppm();
    fs::write("./output.ppm", ppm).expect("Failed to write file.");
    
}