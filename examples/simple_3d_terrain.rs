use bevy::prelude::*;
use bevy_terrain::{
    planar_terrain::{PlanarTerrain, PlanarTerrainConfig},
    terrain::Terrain,
    TerrainPlugin,
};

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, TerrainPlugin))
        .add_systems(Startup, setup)
        .run();
}

fn setup(
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let mut terrain = PlanarTerrain::new(PlanarTerrainConfig {
        width: 5,
        height: 5,
    });
    terrain.generate_mesh(meshes);
    commands.spawn(PbrBundle {
        mesh: terrain.mesh(),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4., 8., 4.),
        ..default()
    });
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(-2., 2.5, 5.).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}
