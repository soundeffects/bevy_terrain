use bevy::prelude::*;

pub mod planar_terrain;
pub mod terrain;

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {}
}

#[test]
fn general_test() {
    println!("Hello, world!");
}
