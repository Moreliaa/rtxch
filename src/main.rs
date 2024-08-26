extern crate rtxch_lib;

fn main() {
    let mut proj = rtxch_lib::projectile::Projectile::from_values(0.0,0.0,0.0, 1.0,0.0,3.0);
    let env = rtxch_lib::environment::Environment::from_values(0.0, 0.0, -0.5, 0.0, 0.1, 0.0);

    for i in 0..100 {
        proj.tick(&env);
        println!("Step {}: {:?}", {i}, {proj.position});
    }
    
}