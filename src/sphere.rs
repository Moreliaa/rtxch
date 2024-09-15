use crate::intersections::{Shape, IntersectionList};
use crate::Ray;
use crate::Tuples;
use crate::Matrix;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, Clone)]
pub struct Sphere {
    transform: Matrix,
    transform_inverse: Matrix,
}

impl Sphere {
    pub fn new() -> Sphere {
        Sphere { transform: Matrix::new(4), transform_inverse: Matrix::new(4) }
    }
}

impl Shape for Sphere {
    fn intersect(this: &Rc<RefCell<Sphere>>, r: &Ray) -> IntersectionList<Sphere> {
        let r = Ray::transform(r, this.borrow().get_transform_inverse());
        let sphere_origin = Tuples::point(0.0,0.0,0.0);
        let sphere_to_ray = r.origin().clone().subtract(&sphere_origin);
        // a = 1.0 only if direction is normalized
        let a = Tuples::dot(&r.direction(), &r.direction());
        let b = 2.0 * Tuples::dot(&r.direction(), &sphere_to_ray);
        let c = Tuples::dot(&sphere_to_ray, &sphere_to_ray) - 1.0;
        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            return IntersectionList::new(vec![], &this);
        }

        let discriminant_sqrt = discriminant.sqrt();
        let t1 = (-b - discriminant_sqrt) / (2.0 * a);
        let t2 = (-b + discriminant_sqrt) / (2.0 * a);
        IntersectionList::new(vec![t1, t2], &this)
    }

    fn set_transform(this: &Rc<RefCell<Self>>, transform: &Matrix) {
        this.borrow_mut().transform = transform.clone();
        this.borrow_mut().transform_inverse = Matrix::inverse(transform).unwrap();
    }

    fn get_transform(&self) -> &Matrix {
        &self.transform
    }

    fn get_transform_inverse(&self) -> &Matrix {
        &self.transform_inverse
    }
}