extern crate rtxch_lib;

use std::fs;

fn main() {
    let mut canvas = rtxch_lib::Canvas::new(900, 550);

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
    for i in 0..100 {
        proj.tick(&env); 
        println!("Step {}: {:?}", {i}, {proj.position});
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
            println!("factor {factor} {y}");
            canvas.write_pixel(x, y as usize, &proj_color);
        }
    }
    
    
    let ppm = canvas.canvas_to_ppm();
    fs::write("./output.ppm", ppm).expect("Failed to write file.");
    
}