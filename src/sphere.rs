use crate::Shape;
use crate::Ray;
use crate::Tuples;
use crate::Matrix;
use crate::Material;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, Clone)]
pub struct Sphere {
    material: Material,
    transform: Matrix,
    transform_inverse: Matrix,
    cast_shadows: bool,
}

impl Sphere {
    pub fn new() -> Rc<RefCell<Sphere>> {
        Rc::new(RefCell::new(Sphere { material: Material::material(), transform: Matrix::new(4), transform_inverse: Matrix::new(4), cast_shadows: true }))
    }
    pub fn glass_sphere() -> Rc<RefCell<Sphere>> {
        let mut material = Material::material();
        material.transparency = 1.0;
        material.refractive_index = 1.5;

        Rc::new(RefCell::new(Sphere { material, transform: Matrix::new(4), transform_inverse: Matrix::new(4), cast_shadows: true }))
    }
}

impl Shape for Sphere {
    fn intersect_local(&self, r: &Ray) -> Vec<f64> {
        let sphere_origin = Tuples::point(0.0,0.0,0.0);
        let sphere_to_ray = r.origin().clone().subtract(&sphere_origin);
        // a = 1.0 only if direction is normalized
        let a = Tuples::dot(&r.direction(), &r.direction());
        let b = 2.0 * Tuples::dot(&r.direction(), &sphere_to_ray);
        let c = Tuples::dot(&sphere_to_ray, &sphere_to_ray) - 1.0;
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
        let origin = Tuples::point(0.0,0.0, 0.0);
        Tuples::vector(p_object_space.x - origin.x, p_object_space.y - origin.y, p_object_space.z - origin.z)
    }

    fn get_type(&self) -> &str {
        "Sphere"
    }

    fn set_cast_shadows(&mut self, b: bool) {
        self.cast_shadows = b;
    }

    fn cast_shadows(&self) -> bool {
        self.cast_shadows
    }
}