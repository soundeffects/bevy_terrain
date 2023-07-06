use bevy::prelude::*;
use bevy_terrain::TerrainPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(TerrainPlugin)
        .run();
}
