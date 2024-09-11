use crate::intersections::{Intersectable, Intersections};
use crate::Ray;
use crate::Tuples;

#[derive(Debug)]
pub struct Sphere {

}

impl Sphere {
    pub fn new() -> Sphere {
        Sphere {}
    }
}

impl Intersectable for Sphere {
    fn intersect(&self, r: &Ray) -> Intersections {
        

        let sphere_origin = Tuples::point(0.0,0.0,0.0);
        let sphere_to_ray = r.origin().clone().subtract(&sphere_origin);
        let a = Tuples::dot(&r.direction(), &r.direction());
        let b = 2.0 * Tuples::dot(&r.direction(), &sphere_to_ray);
        let c = Tuples::dot(&sphere_to_ray, &sphere_to_ray) - 1.0;
        let discriminant = b * b - 4.0 * a * c;
        if discriminant < 0.0 {
            return Intersections::new(vec![]);
        }

        let t1 = (-b - discriminant.sqrt()) / (2.0 * a);
        let t2 = (-b + discriminant.sqrt()) / (2.0 * a);
        Intersections::new(vec![t1, t2])
        
    }
}