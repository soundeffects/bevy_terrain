use bevy::{
    prelude::*,
    render::{mesh::Indices, render_resource::PrimitiveTopology},
    utils::HashMap,
};
use ndshape::Shape;

use crate::chunk::Chunk2x64;

#[derive(Component)]
pub struct PlanarTerrain {
    chunks: HashMap<UVec2, Chunk2x64>,
    outdated_chunks: Vec<UVec2>,
}

impl PlanarTerrain {
    pub fn new() -> Self {
        Self {
            chunks: HashMap::new(),
            outdated_chunks: vec![UVec2 { x: 0, y: 0 }],
        }
    }
}

pub struct PlanarTerrainMeshingPlugin;

impl Plugin for PlanarTerrainMeshingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, planar_meshing);
    }
}

fn outdated_chunks() {}

fn planar_meshing(
    mut commands: Commands,
    mut query: Query<(&mut PlanarTerrain, Entity)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (mut terrain, entity) in query.iter_mut() {
        while !terrain.outdated_chunks.is_empty() {
            let coords = terrain.outdated_chunks.pop().unwrap();
            if !terrain.chunks.contains_key(&coords) {
                terrain.chunks.insert(coords, Chunk2::new());
            }
            let chunk = terrain.chunks.get(&coords).unwrap();

            let mut vertices = vec![];
            let mut normals = vec![];
            let mut texture_coordinates = vec![];

            for (pos, val) in chunk.iter() {
                vertices.push(Vec3 {
                    x: (pos[0] as f32) / (Chunk2::DIM as f32),
                    y: (val as f32) / (u8::MAX as f32),
                    z: (pos[1] as f32) / (Chunk2::DIM as f32),
                });
                normals.push(Vec3 {
                    x: 0.0,
                    y: 1.0,
                    z: 0.0,
                });
                texture_coordinates.push(Vec2 {
                    x: (pos[0] as f32) / (Chunk2::DIM as f32),
                    y: (pos[1] as f32) / (Chunk2::DIM as f32),
                });
            }

            let mut indices = vec![];
            for x in 0..Chunk2::DIM - 1 {
                for y in 0..Chunk2::DIM - 1 {
                    indices.push(Chunk2::SHAPE.linearize([x, y]) as u32);
                    indices.push(Chunk2::SHAPE.linearize([x, y + 1]) as u32);
                    indices.push(Chunk2::SHAPE.linearize([x + 1, y]) as u32);
                    indices.push(Chunk2::SHAPE.linearize([x + 1, y]) as u32);
                    indices.push(Chunk2::SHAPE.linearize([x, y + 1]) as u32);
                    indices.push(Chunk2::SHAPE.linearize([x + 1, y + 1]) as u32);
                }
            }

            let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
            mesh.insert_attribute(Mesh::ATTRIBUTE_POSITION, vertices);
            mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, normals);
            mesh.insert_attribute(Mesh::ATTRIBUTE_UV_0, texture_coordinates);
            mesh.set_indices(Some(Indices::U32(indices)));

            commands.entity(entity).insert(PbrBundle {
                mesh: meshes.add(mesh),
                material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
                transform: Transform::from_xyz(-2., 0., -2.).with_scale(Vec3 {
                    x: 4.0,
                    y: 4.0,
                    z: 4.0,
                }),
                ..default()
            });
        }
    }
}
