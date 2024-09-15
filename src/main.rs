extern crate rtxch_lib;

use std::{f64::consts::PI, fs};
use rtxch_lib::{intersections::Shape, Matrix, Sphere, Tuples, Ray, IntersectionList};

fn main() {
    let mut canvas = rtxch_lib::Canvas::new(900, 550);
    let megasphere = Sphere::new();
    let scale_sphere = Matrix::scale(250.0, 250.0, 250.0);
    let translation_canvas = Matrix::translate((canvas.width / 2) as f64, (canvas.height / 2) as f64, 0.0);
    let transform = translation_canvas * scale_sphere;
    Sphere::set_transform(&megasphere, &transform);

    let color = Tuples::color(1.0, 1.0, 0.5);
    let ray_direction = Tuples::vector(0.0, 0.0, 1.0);
    for x in 0..canvas.width {
        for y in 0..canvas.height {
            let ray = Ray::new(Tuples::point(x as f64, y as f64, -1.0), ray_direction.clone());
            let is = Sphere::intersect(&megasphere, &ray);
            let hit = IntersectionList::hit(&is);
            match hit {
                Some(_) => {
                    canvas.write_pixel(x, y, &color)
                },
                None => {}
            }
        }
    }

    // === Clock example
    /*let rot_angle = PI / 6.0;
    let center = Tuples::point(0.0,0.0,0.0);
    let translation_local = Matrix::translate(0.0, 250.0, 0.0);
    let translation_canvas = Matrix::translate((canvas.width / 2) as f64, (canvas.height / 2) as f64, 0.0);
    let color = Tuples::color(1.0, 1.0, 0.0);
    for i in 0..12 {
        let rotation = Matrix::rotate_z(rot_angle * i as f64);
        let transform =  &translation_canvas * &rotation * &translation_local;
        let pixel = transform * center;
        println!("{:?}", pixel);
        canvas.write_pixel(pixel.x as usize, pixel.y as usize, &color)
    }*/

    // === Clock example end

    // === random stuff

    /*let mut m1 = Matrix::new(4);
    m1 = Matrix::inverse(&m1).unwrap();

    println!("Inverse of identity matrix {:?}", m1);

    let m2 = Matrix::from_values(&vec![25.0,13.0,23.0,765.0,12.0,24.0,63.0,24.0,124.0,46.0,24.0,35.0,12.0,23.0,45.0,123.0]);
    let m2_inv = Matrix::inverse(&m2).unwrap();
    let m2_out = Matrix::mul(&m2, &m2_inv);
    println!("m2 multiplied with its inverse {:?}", m2_out);

    let m3 = Matrix::from_values(&vec![25.0,13.0,23.0,765.0,12.0,24.0,63.0,24.0,124.0,46.0,24.0,35.0,12.0,23.0,45.0,123.0]);
    let m3_inv = Matrix::inverse(&m2).unwrap();
    let m3_invtrans = Matrix::transpose(&m3_inv);
    let m3_trans = Matrix::transpose(&m3);
    let m3_transinv = Matrix::inverse(&m3_trans).unwrap();

    println!("m3 inv -> trans {:?}", m3_invtrans);
    println!("m3 trans -> inv {:?}", m3_transinv);

    let mut m4 = Matrix::new(4);
    let tuple = rtxch_lib::Tuples::vector(0.1, 0.2, 0.3);
    let mut res_tuple = Matrix::mul_tuple(&m4, &tuple);
    println!("tuple * ident {:?}", res_tuple);
    m4.set(0, 2, 5.0);
    m4.set(1, 2, 5.0);
    m4.set(2, 2, 5.0);
    res_tuple = Matrix::mul_tuple(&m4, &tuple);
    println!("tuple * changed {:?}", res_tuple);
    

    

    let mut proj_velocity = rtxch_lib::Tuples::vector(1.0, 1.8, 0.0);
    proj_velocity.normalize().scale(11.25);

    let mut proj = rtxch_lib::projectile::Projectile::
        from_values(0.0,1.0,0.0, proj_velocity.x, proj_velocity.y, proj_velocity.z);
        println!("{:?}", proj.velocity);
    let env = rtxch_lib::environment::Environment::
        from_values(0.0, -0.1, 0.0, -0.01, 0.0, 0.0);


    let proj_color = rtxch_lib::Tuples::color(1.0,0.0,0.0);
    let (mut x_min, mut x_max, mut y_min, mut y_max) =
        (proj.position.x,proj.position.x,proj.position.y,proj.position.y);
    let mut ticks: Vec<(f64, f64)> = Vec::new();
    for _ in 0..100 {
        proj.tick(&env);
        if proj.position.y < 0.0 {
            break;
        }
        ticks.push((proj.position.x, proj.position.y));
        x_min = f64::min(x_min, proj.position.x);
        x_max = f64::max(x_max, proj.position.x);
        y_min = f64::min(y_min, proj.position.y);
        y_max = f64::max(y_max, proj.position.y);
        
    }
    let mut pixels: Vec<(usize, usize)> = Vec::new();
    for (x_pos, y_pos) in ticks.into_iter() {
        let x = ((x_pos - x_min) / x_max) * (canvas.width - 1) as f64;
        let y = (canvas.height - 1) as f64 - ((y_pos - y_min) / y_max) * (canvas.height - 1) as f64;
        pixels.push((x as usize, y as usize));
    }

    for i in 0..pixels.len() - 1 {
        let pixel_1 = pixels[i];
        let pixel_2 = pixels[i + 1];
        for x in pixel_1.0..pixel_2.0 {
            let x_pos_current = (x - pixel_1.0) as f64;
            let x_size = (pixel_2.0 - pixel_1.0) as f64;
            let factor = x_pos_current / x_size;

            let y_size = pixel_2.1 as f64 - pixel_1.1 as f64;
            let y = pixel_1.1 as f64 + factor * y_size;
            canvas.write_pixel(x, y as usize, &proj_color);
        }
    }*/
    
    
    let ppm = canvas.canvas_to_ppm();
    fs::write("./output.ppm", ppm).expect("Failed to write file.");
    
}