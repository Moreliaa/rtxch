use crate::{Matrix, Ray, Tuples};

#[derive(Debug, Default)]
pub struct Camera {
    pub h_size: usize,
    pub v_size: usize,
    pub fov: f64,
    pub transform: Matrix,
    pub pixel_size: f64,
    pub half_width: f64,
    pub half_height: f64,
}

impl Camera {
    pub fn new(h_size: usize, v_size: usize, fov: f64) -> Camera {
        let half_view = (fov / 2.0).tan();
        let aspect_ratio = h_size as f64 / v_size as f64;
        let half_width = if aspect_ratio >= 1.0 { half_view } else { half_view * aspect_ratio };
        let half_height = if aspect_ratio >= 1.0 { half_view / aspect_ratio } else { half_view };
        let pixel_size = half_width * 2.0 / h_size as f64;
        Camera {
            h_size, v_size, fov, transform: Matrix::new(4), pixel_size, half_width, half_height
        }
    }

    pub fn ray_for_pixel(c: &Camera, x: usize, y: usize) -> Ray {
        let x_offset = (x as f64 + 0.5) * c.pixel_size;
        let y_offset = (y as f64 + 0.5) * c.pixel_size;
        
        let world_x = c.half_width  - x_offset;
        let world_y = c.half_height - y_offset;

        let mut pixel = Matrix::inverse(&c.transform).unwrap() * Tuples::point(world_x, world_y, -1.0);
        let origin = Matrix::inverse(&c.transform).unwrap() * Tuples::point(0.0,0.0,0.0);
        let direction = pixel.subtract(&origin).normalize();
        Ray::new(origin, direction)
    }
}