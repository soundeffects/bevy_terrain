use bevy::{
    pbr::wireframe::{Wireframe, WireframeConfig, WireframePlugin},
    prelude::*,
    render::{
        settings::{WgpuFeatures, WgpuSettings},
        RenderPlugin,
    },
};
use bevy_terrain::planar_terrain::{PlanarTerrain, PlanarTerrainMeshingPlugin};
use smooth_bevy_cameras::{
    controllers::fps::{FpsCameraBundle, FpsCameraController, FpsCameraPlugin},
    LookTransformPlugin,
};

fn main() {
    App::new()
        .add_plugins(LookTransformPlugin)
        .add_plugins(FpsCameraPlugin::default())
        .add_plugins(DefaultPlugins.set(RenderPlugin {
            wgpu_settings: WgpuSettings {
                features: WgpuFeatures::POLYGON_MODE_LINE,
                ..default()
            },
        }))
        .add_plugins(WireframePlugin)
        .add_plugins(PlanarTerrainMeshingPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands, mut wireframe_config: ResMut<WireframeConfig>) {
    wireframe_config.global = false;
    commands.spawn((PlanarTerrain::new(), Wireframe));
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4., 8., 4.),
        ..default()
    });
    commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(-2., 2.5, 5.).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        })
        .insert(FpsCameraBundle::new(
            FpsCameraController::default(),
            Vec3::new(-2., 5., 5.),
            Vec3::new(0., 0., 0.),
            Vec3::Y,
        ));
}
