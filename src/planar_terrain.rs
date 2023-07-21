use bevy::prelude::{shape::Plane, *};

use crate::terrain::Terrain;

pub struct PlanarTerrainConfig {
    pub width: u32,
    pub height: u32,
}

pub struct PlanarTerrain {
    config: PlanarTerrainConfig,
    height_map: Vec<u8>,
    mesh: Option<Handle<Mesh>>,
}

impl Terrain for PlanarTerrain {
    type Config = PlanarTerrainConfig;

    fn new(config: PlanarTerrainConfig) -> Self {
        let mut height_map = vec![];

        for _ in 0..(config.width * config.height) {
            height_map.push(0);
        }

        Self {
            config,
            height_map,
            mesh: None,
        }
    }

    fn config(&self) -> &PlanarTerrainConfig {
        &self.config
    }

    fn set_config(&mut self, config: PlanarTerrainConfig) {
        self.config = config;
    }

    fn write(&self) {
        todo!()
    }

    fn sample(&self) {
        todo!()
    }

    fn generate_mesh(&mut self, mut meshes: ResMut<Assets<Mesh>>) {
        let mut vertices = vec![];
        let width = self.config.width;
        let height = self.config.height;
        for x in 0..width {
            for z in 0..height {
                vertices.push(Vec3 {
                    x: (x as f32) / (width as f32),
                    y: self.height_map[(x * width + z) as usize] as f32,
                    z: (z as f32) / (height as f32),
                });
            }
        }
        let mut indices = vec![];
        for x in 0..width - 1 {
            for z in 0..height - 1 {
                indices.push(0);
                indices.push(1);
                indices.push(width);
                indices.push(1);
                indices.push(width);
                indices.push(width + 1);
            }
        }
        self.mesh = Some(meshes.add(Plane::from_size(5.).into()));
    }

    fn mesh(&self) -> Handle<Mesh> {
        self.mesh
            .clone()
            .expect("Attempted to read mesh before generating.")
    }
}
