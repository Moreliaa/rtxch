use crate::Shape;
use crate::Ray;
use crate::Tuples;
use crate::Matrix;
use crate::Material;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, Clone)]
pub struct Cube {
    material: Material,
    transform: Matrix,
    transform_inverse: Matrix,
    cast_shadows: bool,
}

impl Cube {
    pub fn new() -> Rc<RefCell<Cube>> {
        Rc::new(RefCell::new(Cube { material: Material::material(), transform: Matrix::new(4), transform_inverse: Matrix::new(4), cast_shadows: true }))
    }

    fn check_axis(origin: f64, direction: f64) -> (f64, f64) {
        let tmin_num = -1.0 - origin;
        let tmax_num = 1.0 - origin;
        let result = if direction.abs() >= f64::EPSILON {
            (tmin_num / direction, tmax_num /direction)
        } else {
            (tmin_num * f64::INFINITY, tmax_num * f64::INFINITY)
        };
        if result.0 > result.1 {
            (result.1, result.0)
        } else {
            result
        }
    }
}

impl Shape for Cube {
    fn intersect_local(&self, r: &Ray) -> Vec<f64> {
        let (xtmin, xtmax) = Cube::check_axis(r.origin().x, r.direction().x);
        let (ytmin, ytmax) = Cube::check_axis(r.origin().y, r.direction().y);
        let (ztmin, ztmax) = Cube::check_axis(r.origin().z, r.direction().z);
        let tmin = [xtmin, ytmin, ztmin].into_iter().reduce(f64::max).unwrap();
        let tmax = [xtmax, ytmax, ztmax].into_iter().reduce(f64::min).unwrap();
        if tmin > tmax {
            vec![]
        } else {
            vec![tmin, tmax]
        }
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
        let max_coord = [p_object_space.x.abs(), p_object_space.y.abs(), p_object_space.z.abs()].into_iter().reduce(f64::max).unwrap();
        if max_coord == p_object_space.x.abs()  {
            let sign = p_object_space.x.signum();
            Tuples::vector(sign * 1.0, 0.0, 0.0)
        } else if max_coord == p_object_space.y.abs() {
            let sign = p_object_space.y.signum();
            Tuples::vector(0.0, sign * 1.0, 0.0)
        } else {
            let sign = p_object_space.z.signum();
            Tuples::vector(0.0, 0.0, sign * 1.0)
        }
    }

    fn get_type(&self) -> &str {
        "Cube"
    }

    fn set_cast_shadows(&mut self, b: bool) {
        self.cast_shadows = b;
    }

    fn cast_shadows(&self) -> bool {
        self.cast_shadows
    }
}