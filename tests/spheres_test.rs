extern crate rtxch_lib;

use std::collections::HashMap;
use cucumber::{given, when, then, World};
use rtxch_lib::intersections::Shape;
use rtxch_lib::utils::{parse_values_f64, is_equal_f64};
use rtxch_lib::*;
use std::rc::Rc;
use std::cell::RefCell;


#[given(regex = r"(.+) ← (point|vector|ray|sphere|intersect|translation|scaling|normal_at|rotation_z|material)\((.*)\)")]
fn given_item(world: &mut RaysWorld, matches: &[String]) {
    create_item(world, matches);
}

fn create_item(world: &mut RaysWorld, matches: &[String]) {
    let t = matches[0].clone();
    let func = matches[1].as_str();
    match func {
        "point" => {
            let v = parse_values_f64(&matches[2]);
            let p = Tuples::point(v[0], v[1], v[2]);
            world.tuple.insert(t, p);
        },
        "vector" => {
            let v = parse_values_f64(&matches[2]);
            let p = Tuples::vector(v[0], v[1], v[2]);
            world.tuple.insert(t, p);
        },
        "ray" => {
            let v: Vec<&str> = matches[2].split(", ").collect();
            let o = world.tuple.get(&v[0].to_string()).unwrap();
            let d = world.tuple.get(&v[1].to_string()).unwrap();
            let r = Ray::new(o.clone(), d.clone());
            world.ray.insert(t, r);
        },
        "sphere" => {
            world.sphere.insert(t, Sphere::new());
        },
        "intersect" => {
            let v: Vec<&str> = matches[2].split(", ").collect();
            let s = world.sphere.get(&v[0].to_string()).unwrap();
            let r = world.ray.get(&v[1].to_string()).unwrap();
            world.inter.insert(t, Sphere::intersect(s, r));
        },
        "translation" => {
            let v = parse_values_f64(&matches[2]);
            let m = Matrix::translate(v[0], v[1], v[2]);
            world.matrix.insert(t, m);
        },
        "scaling" =>  {
            let v = parse_values_f64(&matches[2]);
            let m = Matrix::scale(v[0], v[1], v[2]);
            world.matrix.insert(t, m);
        },
        "rotation_z" => {
            let v = parse_values_f64(&matches[2]);
            let m = Matrix::rotate_z(v[0]);
            world.matrix.insert(t, m);
        },
        "normal_at" => {
            let v: Vec<&str> = matches[2].split(", ").collect();
            let s = world.sphere.get(&v[0].to_string()).unwrap();
            let p = world.tuple.get(&v[1].to_string()).unwrap();
            world.tuple.insert(t, Sphere::normal_at(s, p));
        },
        "material" => {
            world.material.insert(t, Material::material());
        },
        _ => panic!("{func} not implemented")
    }
}

#[when(regex = r"(.+) ← (point|vector|ray|sphere|intersect|translation|scaling|normal_at)\((.*)\)")]
fn when_item(world: &mut RaysWorld, matches: &[String]) {
    create_item(world, matches);
}

#[given(regex = r"m.ambient ← 1")]
fn set_ambient(world: &mut RaysWorld, _: &[String]) {
    let mat = world.material.get_mut(&"m".to_string()).unwrap();
    mat.ambient = 1.0;
}

#[given(regex = r"s.material ← m")]
fn set_mat(world: &mut RaysWorld, _: &[String]) {
    let s = world.sphere.get(&"s".to_string()).unwrap();
    let mat = world.material.get_mut(&"m".to_string()).unwrap();
    Sphere::set_material(s, &mat);
}

#[given(regex = r"m ← scale \* rot")]
fn make_mat(world: &mut RaysWorld, _: &[String]) {
    let scale = world.matrix.get(&"scale".to_string()).unwrap();
    let rot = world.matrix.get(&"rot".to_string()).unwrap();
    world.matrix.insert("m".to_string(), scale * rot);
}

fn set_transform(world: &mut RaysWorld, matches: &[String]) {
    let v: Vec<&str> = matches[0].split(", ").collect();
    let s = world.sphere.get(v[0]).unwrap();
    let t = world.matrix.get(v[1]).unwrap();
    Sphere::set_transform(s, t);
}

#[given(regex = r"set_transform\((.+)\)")]
fn given_set_transform(world: &mut RaysWorld, matches: &[String]) {
    set_transform(world, matches);
}

#[when(regex = r"set_transform\((.+)\)")]
fn when_set_transform(world: &mut RaysWorld, matches: &[String]) {
    set_transform(world, matches);
}

#[then(regex = r"(.+)\.(origin|direction|count|transform|material|intensity|color|ambient|diffuse|specular|shininess) = (.+)")]
fn check_prop(world: &mut RaysWorld, matches: &[String]) {
    let prop = matches[1].as_str();
    
    match prop {
        "origin" => {
            let r = world.ray.get(&matches[0]).unwrap();
            let t = world.tuple.get(&matches[2]).unwrap();
            assert!(r.origin().is_equal(t));
        },
        "direction" => {
            let r = world.ray.get(&matches[0]).unwrap();
            let t = world.tuple.get(&matches[2]).unwrap();
            assert!(r.direction().is_equal(t));
        },
        "count" => {
            let xs = world.inter.get(&matches[0]).unwrap();
            let t = &matches[2].parse::<usize>().unwrap();
            assert!(xs.count() == *t);
        },
        "transform" => {
            let s = world.sphere.get(&matches[0]).unwrap();
            match matches[2].as_str() {
                "identity_matrix" => {
                    let ident = Matrix::new(4);
                    s.borrow().get_transform().is_equal(&ident);
                },
                _ => {
                    let m = world.matrix.get(&matches[2]).unwrap();
                    s.borrow().get_transform().is_equal(m);
                },
            }
        },
        "material" => {
            let obj = world.sphere.get(&matches[0]).unwrap();
            let target = match matches[2].as_str() {
                "material()" => &Material::material(),
                _ => world.material.get(&matches[2]).unwrap(),
            };
            assert!(obj.borrow().get_material().is_equal(&target));
        },
    "color" => {
        let i = world.material.get(&matches[0]).unwrap();
        let target = world.tuple.get(&matches[2]).unwrap();
        assert!(i.color.is_equal(target));
    },
    "ambient" => {
        let i = world.material.get(&matches[0]).unwrap();
        let target = &matches[2].parse::<f64>().unwrap();
        assert!(is_equal_f64(i.ambient, *target));
    },
    "diffuse" => {
        let i = world.material.get(&matches[0]).unwrap();
        let target = &matches[2].parse::<f64>().unwrap();
        assert!(is_equal_f64(i.diffuse, *target));
    },
    "specular" => {
        let i = world.material.get(&matches[0]).unwrap();
        let target = &matches[2].parse::<f64>().unwrap();
        assert!(is_equal_f64(i.specular, *target));
    },
    "shininess" => {
        let i = world.material.get(&matches[0]).unwrap();
        let target = &matches[2].parse::<f64>().unwrap();
        assert!(is_equal_f64(i.shininess, *target));
    },
        _ => panic!(),
    }
}

#[then(regex = r"(.+)\[(.+)\]\.(origin|direction|t|object|count|material|intensity|color|ambient|diffuse|specular|shininess) = (.+)")]
fn check_sub_prop(world: &mut RaysWorld, matches: &[String]) {
    let prop = matches[2].as_str();
    
    let idx = matches[1].parse::<usize>().unwrap();
    

    match prop {
        "t" => {
            let i = world.inter.get(&matches[0]).unwrap();
            let obj = i.xs().get(idx).unwrap();
            let target = matches[3].parse::<f64>().unwrap();
            assert!(is_equal_f64(obj.t(), target));
        },
        "object" => {
            let i = world.inter.get(&matches[0]).unwrap();
            let obj = i.xs().get(idx).unwrap();
            let target = world.sphere.get(&matches[3]).unwrap();
            assert!(Rc::ptr_eq(obj.object(), &target));
        },
        "material" => {
            let obj = world.sphere.get(&matches[0]).unwrap();
            let target = match matches[3].as_str() {
                "material()" => &Material::material(),
                _ => world.material.get(&matches[3]).unwrap(),
            };
            assert!(obj.borrow().get_material().is_equal(&target));
        },
    "color" => {
        let i = world.material.get(&matches[0]).unwrap();
        let target = world.tuple.get(&matches[2]).unwrap();
        assert!(i.color.is_equal(target));
    },
    "ambient" => {
        let i = world.material.get(&matches[0]).unwrap();
        let target = &matches[2].parse::<f64>().unwrap();
        assert!(is_equal_f64(i.ambient, *target));
    },
    "diffuse" => {
        let i = world.material.get(&matches[0]).unwrap();
        let target = &matches[2].parse::<f64>().unwrap();
        assert!(is_equal_f64(i.diffuse, *target));
    },
    "specular" => {
        let i = world.material.get(&matches[0]).unwrap();
        let target = &matches[2].parse::<f64>().unwrap();
        assert!(is_equal_f64(i.specular, *target));
    },
    "shininess" => {
        let i = world.material.get(&matches[0]).unwrap();
        let target = &matches[2].parse::<f64>().unwrap();
        assert!(is_equal_f64(i.shininess, *target));
    },
        
        _ => panic!()
    }
}

#[then(regex = r"xs\[(.+)\] = (.+)")]
fn check_intersect_idx(world: &mut RaysWorld, matches: &[String]) {
    let idx = &matches[0].parse::<usize>().unwrap();
    let target = &matches[1].parse::<f64>().unwrap();
    let xs = world.inter.get(&"xs".to_string()).unwrap();
    let val = xs.xs().get(*idx).unwrap();
    assert!(val.t() == *target);
}

#[then(regex = r"position\((.+), (.+)\) = point\((.+)\)")]
fn check_pos(world: &mut RaysWorld, matches: &[String]) {
    let r = world.ray.get(&matches[0]).unwrap();
    let time = matches[1].parse::<f64>().unwrap();
    let v = parse_values_f64(&matches[2]);
    let p = Tuples::point(v[0], v[1], v[2]);

    let result = Ray::position(r, time);
    assert!(result.is_equal(&p));
}

#[then(regex = r"(.+) = vector\((.+)\)")]
fn check_vec(world: &mut RaysWorld, matches: &[String]) {
    let r = world.tuple.get(&matches[0]).unwrap();
    let v = parse_values_f64(&matches[1]);
    let p = Tuples::vector(v[0], v[1], v[2]);
    assert!(r.is_equal(&p));
}

#[then(regex = r"(n) = normalize\((n)\)")]
fn check_norm(world: &mut RaysWorld, matches: &[String]) {
    let n = world.tuple.get(&matches[0]).unwrap();
    let norm = n.clone().normalize();
    assert!(n.is_equal(&norm));
}

#[derive(Debug, Default, World)]
struct RaysWorld {
    ray: HashMap<String, Ray>,
    tuple: HashMap<String, Tuples>,
    sphere: HashMap<String, Rc<RefCell<Sphere>>>,
    inter:  HashMap<String, IntersectionList<Sphere>>,
    matrix: HashMap<String, Matrix>,
    material: HashMap<String, Material>,
}

fn main() {
    futures::executor::block_on(RaysWorld::run(
        "tests/features/spheres.feature",
    ));
}