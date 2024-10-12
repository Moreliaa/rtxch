use crate::Shape;
use crate::Ray;
use crate::Tuples;
use crate::Matrix;
use crate::Material;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, Clone)]
pub struct Plane {
    material: Material,
    transform: Matrix,
    transform_inverse: Matrix,
}

impl Plane {
    pub fn new() -> Rc<RefCell<Plane>> {
        Rc::new(RefCell::new(Plane { material: Material::material(), transform: Matrix::new(4), transform_inverse: Matrix::new(4) }))
    }
}

impl Shape for Plane {
    fn intersect_local(&self, r: &Ray) -> Vec<f64> {
        vec![]
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
        Tuples::vector(1.0,0.0, 0.0)
    }

    fn get_type(&self) -> &str {
        "Plane"
    }
}