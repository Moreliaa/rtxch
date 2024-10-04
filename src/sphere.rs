use crate::intersections::{Shape, IntersectionList};
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
}

impl Sphere {
    pub fn new() -> Rc<RefCell<Sphere>> {
        Rc::new(RefCell::new(Sphere { material: Material::material(), transform: Matrix::new(4), transform_inverse: Matrix::new(4) }))
    }

    pub fn is_equal(&self, other: &Rc<RefCell<Self>>) -> bool {
        self.material.is_equal(&other.borrow().material) &&
        self.transform.is_equal(&other.borrow().transform)
    }
}

impl Shape for Sphere {
    fn intersect(s: &Rc<RefCell<Sphere>>, r: &Ray) -> IntersectionList<Sphere> {
        let r = Ray::transform(r, s.borrow().get_transform_inverse());
        let sphere_origin = Tuples::point(0.0,0.0,0.0);
        let sphere_to_ray = r.origin().clone().subtract(&sphere_origin);
        // a = 1.0 only if direction is normalized
        let a = Tuples::dot(&r.direction(), &r.direction());
        let b = 2.0 * Tuples::dot(&r.direction(), &sphere_to_ray);
        let c = Tuples::dot(&sphere_to_ray, &sphere_to_ray) - 1.0;
        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            return IntersectionList::new(vec![], &s);
        }

        let discriminant_sqrt = discriminant.sqrt();
        let t1 = (-b - discriminant_sqrt) / (2.0 * a);
        let t2 = (-b + discriminant_sqrt) / (2.0 * a);
        IntersectionList::new(vec![t1, t2], &s)
    }

    fn set_transform(s: &Rc<RefCell<Self>>, transform: &Matrix) {
        s.borrow_mut().transform = transform.clone();
        s.borrow_mut().transform_inverse = Matrix::inverse(transform).unwrap();
    }

    fn set_material(s: &Rc<RefCell<Self>>, material: &Material) {
        s.borrow_mut().material = material.clone();
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

    fn normal_at(s: &Rc<RefCell<Self>>, p: &Tuples) -> Tuples {
        let origin = Tuples::point(0.0,0.0, 0.0);
        let inverse_transform = Matrix::inverse(s.borrow().get_transform()).unwrap();
        let p_object_space = &inverse_transform * p;
        let n = Tuples::vector(p_object_space.x - origin.x, p_object_space.y - origin.y, p_object_space.z - origin.z);
        let mut n_world = Matrix::transpose(&inverse_transform) * n;
        n_world.w = 0.0; // remove influence from translation
        n_world.normalize()

    }
}