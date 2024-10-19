use crate::Shape;
use crate::Ray;
use crate::Tuples;
use crate::Matrix;
use crate::Material;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, Clone)]
pub struct Cylinder {
    material: Material,
    transform: Matrix,
    transform_inverse: Matrix,
    cast_shadows: bool,
}

impl Cylinder {
    pub fn new() -> Rc<RefCell<Cylinder>> {
        Rc::new(RefCell::new(Cylinder { material: Material::material(), transform: Matrix::new(4), transform_inverse: Matrix::new(4), cast_shadows: true }))
    }
}

impl Shape for Cylinder {
    fn intersect_local(&self, r: &Ray) -> Vec<f64> {
        let a = r.direction().x.powf(2.0) + r.direction().z.powf(2.0);
        if a < crate::utils::EPSILON {
            return vec![];
        }

        let b =
            2.0 * r.origin().x * r.direction().x +
            2.0 * r.origin().z * r.direction().z;
        let c = r.origin().x.powf(2.0) + r.origin().z.powf(2.0) - 1.0;

        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            return vec![];
        }

        let discriminant_sqrt = discriminant.sqrt();
        let t1 = (-b - discriminant_sqrt) / (2.0 * a);
        let t2 = (-b + discriminant_sqrt) / (2.0 * a);
        vec![t1, t2]
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
        Tuples::vector(p_object_space.x, 0.0, p_object_space.z)
    }

    fn get_type(&self) -> &str {
        "Cylinder"
    }

    fn set_cast_shadows(&mut self, b: bool) {
        self.cast_shadows = b;
    }

    fn cast_shadows(&self) -> bool {
        self.cast_shadows
    }
}