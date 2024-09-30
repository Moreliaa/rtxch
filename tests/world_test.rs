extern crate rtxch_lib;

use std::collections::HashMap;
use cucumber::{given, when, then, World};
use rtxch_lib::*;

#[given(regex = r"w ← world()")]
fn given_world(world: &mut WorldWorld, _: &[String]) {
    world.world = rtxch_lib::World::new();
}

#[then(regex = r"w contains no objects")]
fn no_obj(world: &mut WorldWorld, _: &[String]) {
    assert!(world.world.get_objects().len() == 0);
}

#[then(regex = r"w has no light source")]
fn no_light(world: &mut WorldWorld, _: &[String]) {
    assert!(world.world.get_point_lights().len() == 0);
}

    


#[derive(Debug, Default, World)]
struct WorldWorld {
    world: rtxch_lib::World,
}



fn main() {
    futures::executor::block_on(WorldWorld::run(
        "tests/features/world.feature",
    ));
}