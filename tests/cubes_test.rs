extern crate rtxch_lib;

use std::collections::HashMap;
use cucumber::{given, when, then, World, gherkin::Step};
use rtxch_lib::*;
use rtxch_lib::utils::parse_values_f64;
use std::rc::Rc;
use std::cell::RefCell;

#[given(regex = r"w ← world()")]
fn given_world(world: &mut CubesWorld, _: &[String]) {
    world.world = rtxch_lib::World::new();
}

#[when(regex = r"w ← default_world()")]
#[given(regex = r"w ← default_world()")]
fn given_default_world(world: &mut CubesWorld, _: &[String]) {
    world.world = rtxch_lib::World::default_world();
}
#[given(regex = r"(shape) ← plane\(\).+$")]
fn given_plane(world: &mut CubesWorld, matches: &[String]) {
    let plane = Plane::new();
    plane.borrow_mut().set_transform(&Matrix::translate(0.0,-1.0,0.0));
    plane.borrow_mut().get_mut_material().reflective = 0.5;
    plane.borrow_mut().get_mut_material().refractive_index = 1.5;
    world.shape.insert(matches[0].to_string(), plane);
}

#[given(regex = r"(lower) ← plane\(\).+$")]
fn given_lower_plane(world: &mut CubesWorld, matches: &[String]) {
    let plane = Plane::new();
    plane.borrow_mut().set_transform(&Matrix::translate(0.0,-1.0,0.0));
    plane.borrow_mut().get_mut_material().reflective = 1.0;
    world.shape.insert(matches[0].to_string(), plane);
}

#[given(regex = r"(upper) ← plane\(\).+$")]
fn given_upper_plane(world: &mut CubesWorld, matches: &[String]) {
    let plane = Plane::new();
    plane.borrow_mut().set_transform(&Matrix::translate(0.0,1.0,0.0));
    plane.borrow_mut().get_mut_material().reflective = 1.0;
    world.shape.insert(matches[0].to_string(), plane);
}

#[given(regex = r"(.+) ← (local_intersect|local_normal_at|cube|reflected_color|refracted_color|intersections|point|vector|ray|intersect|translation|scaling|normal_at|rotation_z|color|point_light|sphere|prepare_computations)\((.*)\)$")]
#[when(regex = r"(.+) ← (local_intersect|local_normal_at|cube|reflected_color|refracted_color|intersections|point|vector|ray|sphere|intersect|translation|scaling|normal_at|point_light|intersect_world|prepare_computations|shade_hit|color_at)\((.*)\)")]
fn given_item(world: &mut CubesWorld, matches: &[String]) {
    create_item(world, matches);
}

#[given(regex = r"^(floor|ball) ← (plane|sphere)\(\) with:$")]
fn floor_plane(world: &mut CubesWorld, step: &Step, matches: &[String]) {
    let name = &matches[0];
    let shape: Rc<RefCell<dyn Shape>> = match matches[1].as_str() {
        "plane" => Plane::new(),
        "sphere" => Sphere::new(),
        _ => panic!(),
    };
    
    if let Some(table) = step.table.as_ref() {
        for row in table.rows.iter() {
            let prop = &row[0];
            let val = row[1].parse::<f64>();
            match prop.as_str() {
                "transform" => {
                    let rx = cucumber::codegen::Regex::new(r"(translation)\((.+)\)").unwrap();
                    let m = rx.captures(&row[1]);
                    if let Some(m) = m {
                        let fun = &m[1];
                        let values = parse_values_f64(&m[2].to_string());
                        
                        let transform = match fun {
                            "translation" => {
                                Matrix::translate(values[0], values[1], values[2])
                            },
                            _ => panic!("transform not implemented {fun}"),
                        };
                        shape.borrow_mut().set_transform(&transform);
                    } else {
                        panic!("failed to match transform: {:?}", row[1]);
                    }
                },
                "material.transparency" => shape.borrow_mut().get_mut_material().transparency = val.unwrap(),
                "material.refractive_index" => shape.borrow_mut().get_mut_material().refractive_index = val.unwrap(),
                "material.color" => {
                    let rx = cucumber::codegen::Regex::new(r"\((.+)\)").unwrap();
                    let m = rx.captures(&row[1]).unwrap();
                    let values = parse_values_f64(&m[1].to_string());
                    let color = Tuples::color(values[0], values[1], values[2]);
                    shape.borrow_mut().get_mut_material().pattern = SingleColorPattern::new(color);
                },
                "material.ambient" => shape.borrow_mut().get_mut_material().ambient = val.unwrap(),
                "material.reflective" => shape.borrow_mut().get_mut_material().reflective = val.unwrap(),
                _ => panic!("Prop: {prop} not implemented"),
            }
        }
    }

    world.shape.insert(name.clone(), shape);
}

#[given(regex = r"^(shape|A|B) has:$")]
fn shape_has(world: &mut CubesWorld, step: &Step, matches: &[String]) {
    let shape = world.shape.get(&matches[0]).unwrap();
    if let Some(table) = step.table.as_ref() {
        for row in table.rows.iter() {
            let prop = &row[0];
            let val = row[1].parse::<f64>();
            match prop.as_str() {
                "material.transparency" => shape.borrow_mut().get_mut_material().transparency = val.unwrap(),
                "material.refractive_index" => shape.borrow_mut().get_mut_material().refractive_index = val.unwrap(),
                "material.pattern" => shape.borrow_mut().get_mut_material().pattern = TestPattern::new(),
                "material.ambient" => shape.borrow_mut().get_mut_material().ambient = val.unwrap(),
                _ => panic!("Prop: {prop} not implemented"),
            }
        }
    }
}

#[given("s1 ← sphere() with: material.color(0.8, 1.0, 0.6) | material.diffuse(0.7) | material.specular(0.2) |")]
fn sphere1(world: &mut CubesWorld) {
    let sphere = Sphere::new();
    let mut material = Material::material();
    material.pattern = SingleColorPattern::new(Tuples::color(0.8,1.0,0.6));
    material.diffuse = 0.7;
    material.specular = 0.2;
    sphere.borrow_mut().set_material(&material);

    world.shape.insert("s1".to_string(), sphere);
}

#[given("s2 ← sphere() with: transform | scaling(0.5, 0.5, 0.5) |")]
fn sphere2(world: &mut CubesWorld) {
    let sphere = Sphere::new();
    let transform = Matrix::scale(0.5, 0.5, 0.5);
    sphere.borrow_mut().set_transform(&transform);

    world.shape.insert("s2".to_string(), sphere);
}

#[given("s2 ← sphere() with: | transform | translation(0, 0, 10) |")]
fn sphere2_alter(world: &mut CubesWorld) {
    let sphere = Sphere::new();
    let transform = Matrix::translate(0.0,0.0,10.0);
    sphere.borrow_mut().set_transform(&transform);

    world.shape.insert("s2".to_string(), sphere);
}

#[then(regex = r"(.+) = (color|vector)\((.+)\)")]
fn check_result(world: &mut CubesWorld, matches: &[String]) {
    let fun = matches[1].as_str();
    let val = parse_values_f64(&matches[2]);
    let col = match fun {
        "color" => Tuples::color(val[0], val[1], val[2]),
        "vector" => Tuples::vector(val[0], val[1], val[2]),
        _ => panic!(),
    };
    let r = world.tuple.get(&matches[0].to_string()).unwrap();
    assert!(r.is_equal(&col), "{:?}", r);
}

fn create_item(world: &mut CubesWorld, matches: &[String]) {
    let t = matches[0].clone();
    let func = matches[1].as_str();
    match func {
        "local_intersect" => {
            let v: Vec<&str> = matches[2].split(", ").collect();
            let s = world.shape.get(&v[0].to_string()).unwrap();
            let r = world.ray.get(&v[1].to_string()).unwrap();
            let result = s.borrow().intersect_local(r);
            let il = IntersectionList::new(result, s);
            world.inter_list.insert(t, il);
        },
        "local_normal_at" => {
            let v: Vec<&str> = matches[2].split(", ").collect();
            let s = world.shape.get(&v[0].to_string()).unwrap();
            let p = world.tuple.get(&v[1].to_string()).unwrap();
            let n = s.borrow().normal_at_local(p);
            world.tuple.insert(t, n);
        },
        "cube" =>  {
            world.shape.insert(t, Cube::new());
        },
        "reflected_color" => {
            let v: Vec<&str> = matches[2].split(", ").collect();
            let w = &world.world;
            let comps = world.comps.get(&v[1].to_string()).unwrap();
            let remaining= if v.len() == 3 { v[2].parse::<i32>().unwrap() } else { MAX_ITERATIONS };
            world.tuple.insert(t, rtxch_lib::World::reflected_color(w, comps, remaining));
        },
        "refracted_color" => {
            let v: Vec<&str> = matches[2].split(", ").collect();
            let w = &world.world;
            let comps = world.comps.get(&v[1].to_string()).unwrap();
            let remaining= if v.len() == 3 { v[2].parse::<i32>().unwrap() } else { MAX_ITERATIONS };
            world.tuple.insert(t, rtxch_lib::World::refracted_color(w, comps, remaining));
        },
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
        "intersection" => {
            let v: Vec<&str> = matches[2].split(", ").collect();
            let time = v[0].parse::<f64>().unwrap();
            let obj = world.shape.get(&v[1].to_string()).unwrap();
            world.inter.insert(t, Intersection::new(time, obj));
        },
        "intersections" => {
            let v: Vec<&str> = matches[2].split(", ").collect();
            let i: Vec<Intersection> = v.into_iter().map(|val| world.inter.get(&val.to_string()).unwrap().clone()).collect();
            let l = IntersectionList::intersections_from_vec(i);
            world.inter_list.insert(t, l);
        },
        "color" => {
            let v = parse_values_f64(&matches[2]);
            let p = Tuples::color(v[0], v[1], v[2]);
            world.tuple.insert(t, p);
        },
        "point_light" => {
            let v: Vec<&str> = matches[2].split(", ").collect();
            let pos = world.tuple.get(&v[0].to_string()).unwrap();
            let intensity = world.tuple.get(&v[1].to_string()).unwrap();
            world.plight.insert(t, lights::point_light(pos, intensity));
        },
        "prepare_computations" => {
            let v: Vec<&str> = matches[2].split(", ").collect();
            let i = world.inter.get(&v[0].to_string()).unwrap();
            let r = world.ray.get(&v[1].to_string()).unwrap();
            let il = if v.len() > 2 {
                world.inter_list.get(&v[2].to_string()).unwrap()
            } else {
                &IntersectionList::intersections_from_vec(vec![])
            };
            let comps = Intersection::prep_computations(i, r, il);
            world.comps.insert(t, comps);
        },
        "ray" => {
            let v: Vec<&str> = matches[2].split(", ").collect();
            let o = world.tuple.get(&v[0].to_string()).unwrap();
            let d = world.tuple.get(&v[1].to_string()).unwrap();
            let r = Ray::new(o.clone(), d.clone());
            world.ray.insert(t, r);
        },
        "intersect_world" => {
            let v: Vec<&str> = matches[2].split(", ").collect();
            let w = &world.world;
            let r = world.ray.get(&v[1].to_string()).unwrap();
            let intersections = rtxch_lib::World::intersect_world(w, &r);
            world.inter_list.insert(t, intersections);
        },
        "shade_hit" => {
            let w = &world.world;
            let comps = world.comps.get(&"comps".to_string()).unwrap();
            let hit = rtxch_lib::World::shade_hit(w, comps, MAX_ITERATIONS);
            world.tuple.insert(t, hit);
        },
        "color_at" => {
            let v: Vec<&str> = matches[2].split(", ").collect();
            let w = &world.world;
            let r = world.ray.get(&v[1].to_string()).unwrap();
            let color = rtxch_lib::World::color_at(w, r, MAX_ITERATIONS);
            world.tuple.insert(t, color);
        },
        "sphere" => {
            world.shape.insert(t, Sphere::new());
        }
        _ => panic!("{func} not implemented")
    }
}

#[given(regex = r"(s|s.|shape|plane|lower|upper|floor|ball) is added to w")]
fn add_sphere(world: &mut CubesWorld, matches: &[String]) {
    let sphere = world.shape.get(&matches[0]).unwrap();
    world.world.add_object(Rc::clone(sphere));
}

#[given(regex = r"(.+) ← the (first|second) object in w")]
fn first(world: &mut CubesWorld, matches: &[String]) {
    let idx = if matches[1].as_str() == "first" { 0 } else { 1 };
    let shape = Rc::clone(world.world.get_objects().get(idx).unwrap());
    world.shape.insert(matches[0].clone(), shape);
}

#[given(regex = r"(.+).material.ambient ← 1")]
fn set_ambient(world: &mut CubesWorld, matches: &[String]) {
    let sphere = world.shape.get(&matches[0]).unwrap();
    sphere.borrow_mut().get_mut_material().ambient = 1.0;
}

#[given(regex = r"w.light ← light")]
fn set_light(world: &mut CubesWorld, _: &[String]) {
    let light = world.plight.get(&"light".to_string()).unwrap();
    world.world.remove_lights();
    world.world.add_point_light(light.clone());
}



#[given(regex = r"^(.+) ← intersection\((.+), (.+)\)$")]
fn intershape(world: &mut CubesWorld, matches: &[String]) {
    let t = matches[1].parse::<f64>().unwrap();
    let shape = world.shape.get(&matches[2].to_string()).unwrap();
    let i = Intersection::new(t, shape);
    world.inter.insert(matches[0].clone(), i);
}

#[then(regex = r"w.light = light")]
fn then_light(world: &mut CubesWorld, _: &[String]) {
    let light = world.plight.get(&"light".to_string()).unwrap();
    let world_light = world.world.get_point_lights().get(0).unwrap();
    assert!(world_light.is_equal(&light));
}

#[then(regex = r"color_at\(w, r\) should terminate successfully")]
fn check_inf(world: &mut CubesWorld, _: &[String]) {
    let r = world.ray.get(&"r".to_string()).unwrap();
    let _ = rtxch_lib::World::color_at(&world.world, r, MAX_ITERATIONS);
    assert!(true);
}

#[then(regex = r"w contains (s\d)")]
fn contains(world: &mut CubesWorld, matches: &[String]) {
    let sphere = world.shape.get(&matches[0]).unwrap();
    let world_objects = world.world.get_objects();
    let mut result = false;
    for o in world_objects {
        if <dyn Shape>::is_equal(o, sphere) {
            result = true;
            break;
        }
    }
    assert!(result);
}

#[then(regex = r"is_shadowed\(w, p\) is (true|false)")]
fn check_shadow(world: &mut CubesWorld, matches: &[String]) {
    let p = world.tuple.get(&"p".to_string()).unwrap();
    let target = match matches[0].as_str() {
        "true" => true,
        "false" => false,
        _ => panic!(),
    };
    assert!(rtxch_lib::World::is_shadowed(&world.world, p, world.world.get_point_lights().get(0).unwrap()) == target);
}

#[then(regex = r"c = inner.material.color")]
fn check_inner_color(world: &mut CubesWorld, _: &[String]) {
    let sphere = world.shape.get(&"inner".to_string()).unwrap();
    let c = world.tuple.get(&"c".to_string()).unwrap();
    assert!(sphere.borrow().get_material().pattern.borrow().color_a().is_equal(c));
}

#[then(regex = r"(comps)\.(t|object|point|eyev|normalv|inside) = (.+)")]
fn check_prop_comps(world: &mut CubesWorld, matches: &[String]) {
    let comps = world.comps.get(&matches[0]).unwrap();
    let prop = matches[1].as_str();
    
    match prop {
        "t" => {
            let i = world.inter.get(&"i".to_string()).unwrap();
            assert!(rtxch_lib::utils::is_equal_f64(comps.t, i.t()));
        },
        "object" => {
            let i = world.inter.get(&"i".to_string()).unwrap();
            assert!(Rc::ptr_eq(i.object(), &comps.object));
        },
        "point" => {
            let target = world.tuple.get(&matches[2]).unwrap();
            assert!(comps.point.is_equal(&target));
        },
        "eyev" => {
            let target = world.tuple.get(&matches[2]).unwrap();
            assert!(comps.eye_v.is_equal(&target));
        },
        "normalv" => {
            let target = world.tuple.get(&matches[2]).unwrap();
            assert!(comps.normal_v.is_equal(&target));
        },
        "inside" => {
            let target = match matches[2].as_str() {
                "false" => false,
                "true" => true,
                _ => panic!(),
            };
            assert!(comps.inside == target);

        }
        _ => panic!()
    }
}

#[then(regex = r"w contains no objects")]
fn no_obj(world: &mut CubesWorld, _: &[String]) {
    assert!(world.world.get_objects().len() == 0);
}

#[then(regex = r"w has no light source")]
fn no_light(world: &mut CubesWorld, _: &[String]) {
    assert!(world.world.get_point_lights().len() == 0);
}

#[then(regex = r"xs\.(count) = (.+)")]
fn check_prop_intersection(world: &mut CubesWorld, matches: &[String]) {
    let xs = world.inter_list.get(&"xs".to_string()).unwrap();
    let prop = matches[0].as_str();
    match prop {
        "count" => {
            let target = matches[1].parse::<usize>().unwrap();
            assert!(xs.count() == target);
        },
        _ => panic!()
    }
}

#[then(regex = r"xs\[(\d)\]\.(t) = (.+)")]
fn check_prop_intersection_idx(world: &mut CubesWorld, matches: &[String]) {
    let xs = world.inter_list.get(&"xs".to_string()).unwrap();
    let idx = matches[0].parse::<usize>().unwrap();
    let entry = xs.xs().get(idx).unwrap();
    let prop = matches[1].as_str();
    match prop {
        "t" => {
            let target = matches[2].parse::<f64>().unwrap();
            assert!(rtxch_lib::utils::is_equal_f64(entry.t(), target), "{:?}", xs.xs().into_iter().map(|a| a.t()).collect::<Vec<f64>>());
        },
        _ => panic!()
    }
}


#[derive(Debug, Default, World)]
struct CubesWorld {
    world: rtxch_lib::World,
    plight: HashMap<String, PointLight>,
    tuple: HashMap<String, Tuples>,
    shape: HashMap<String, Rc<RefCell<dyn Shape>>>,
    ray: HashMap<String, Ray>,
    inter_list: HashMap<String, IntersectionList>,
    inter: HashMap<String, Intersection>,
    comps: HashMap<String, Computations>,
}



fn main() {
    futures::executor::block_on(CubesWorld::run(
        "tests/features/cubes.feature",
    ));
}