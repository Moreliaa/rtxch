use crate::Shape;
use crate::Ray;
use crate::Tuples;
use crate::Matrix;
use crate::Material;
use core::f64;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, Clone)]
pub struct Cone {
    material: Material,
    transform: Matrix,
    transform_inverse: Matrix,
    cast_shadows: bool,
    pub y_min: f64,
    pub y_max: f64,
    pub closed: bool,
}

impl Cone {
    pub fn new() -> Rc<RefCell<Cone>> {
        Cone::new_limited(-f64::INFINITY, f64::INFINITY, false)
    }

    pub fn new_limited(min: f64, max: f64, closed: bool) -> Rc<RefCell<Cone>> {
        Rc::new(
            RefCell::new(
                Cone { 
                    material: Material::material(), 
                    transform: Matrix::new(4), 
                    transform_inverse: Matrix::new(4), 
                    cast_shadows: true,
                    y_min: min,
                    y_max: max,
                    closed
                }
            )
        )
    }

    fn check_cap(r: &Ray, t: f64, y_plane: f64) -> bool {
        let x = r.origin().x + t * r.direction().x;
        let z = r.origin().z + t * r.direction().z;
        (x.powf(2.0) + z.powf(2.0)).sqrt() <= y_plane.abs()
    }

    fn intersect_caps(&self, r: &Ray) -> Vec<f64> {
        if !self.closed || r.direction().y.abs() < crate::utils::EPSILON {
            return vec![];
        }
        // lower cap
        let mut result = vec![];
        let t0 = (self.y_min - r.origin().y) / r.direction().y;
        if Cone::check_cap(r, t0, self.y_min) {
            result.push(t0);
        }
        // upper cap
        let t1 = (self.y_max - r.origin().y) / r.direction().y;
        if Cone::check_cap(r, t1, self.y_max) {
            result.push(t1);
        }

        result
    }
}

impl Shape for Cone {
    fn intersect_local(&self, r: &Ray) -> Vec<f64> {
        let mut result = self.intersect_caps(r);
        let a = r.direction().x.powf(2.0) - r.direction().y.powf(2.0) + r.direction().z.powf(2.0);
        let b =
            2.0 * r.origin().x * r.direction().x -
            2.0 * r.origin().y * r.direction().y +
            2.0 * r.origin().z * r.direction().z;
        let c = r.origin().x.powf(2.0) - r.origin().y.powf(2.0) + r.origin().z.powf(2.0);

        if a.abs() < crate::utils::EPSILON && b.abs() < crate::utils::EPSILON {
            return result;
        }

        if a.abs() < crate::utils::EPSILON {
            let t = -c / (2.0 * b);
            result.push(t);
            result.sort_by(|a, b| a.partial_cmp(&b).unwrap());
            return result;
        }

        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            return result;
        }

        let discriminant_sqrt = discriminant.sqrt();
        let mut t1 = (-b - discriminant_sqrt) / (2.0 * a);
        let mut t2 = (-b + discriminant_sqrt) / (2.0 * a);
        if t1 > t2 {
            let temp = t2;
            t2 = t1;
            t1 = temp;
        }
        
        let y1 = r.origin().y + t1 * r.direction().y;
        if self.y_min < y1 && y1 < self.y_max {
            result.push(t1);
        }

        let y2 = r.origin().y + t2 * r.direction().y;
        if self.y_min < y2 && y2 < self.y_max {
            result.push(t2);
        }
        
        result.sort_by(|a, b| a.partial_cmp(&b).unwrap());
        result
    }

    fn set_transform(&mut self, transform: &Matrix) {
        self.transform = transform.clone();
        self.transform_inverse = Matrix::inverse(transform).unwrap();
    }

    fn set_material(&mut self, material: &Material) {
        self.material = material.clone();
    }

    fn get_material(&self) -> &Material {
        &self.material
    }

    fn get_mut_material(&mut self) -> &mut Material {
        &mut self.material
    }

    fn get_transform(&self) -> &Matrix {
        &self.transform
    }

    fn get_transform_inverse(&self) -> &Matrix {
        &self.transform_inverse
    }

    fn normal_at_local(&self, p_object_space: &Tuples) -> Tuples {
        let distance_y_axis = (p_object_space.x.powf(2.0) + p_object_space.z.powf(2.0)).sqrt();
        if distance_y_axis < self.y_max.abs() && p_object_space.y >= self.y_max - crate::utils::EPSILON {
            return Tuples::vector(0.0, 1.0, 0.0);
        } else if distance_y_axis < self.y_min.abs() && p_object_space.y <= self.y_min + crate::utils::EPSILON {
            return Tuples::vector(0.0, -1.0, 0.0);
        }
        let y = if p_object_space.y > 0.0 { - distance_y_axis } else { distance_y_axis };
        Tuples::vector(p_object_space.x, y, p_object_space.z)
    }

    fn get_type(&self) -> &str {
        "Cone"
    }

    fn set_cast_shadows(&mut self, b: bool) {
        self.cast_shadows = b;
    }

    fn cast_shadows(&self) -> bool {
        self.cast_shadows
    }
}