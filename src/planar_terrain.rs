use bevy::prelude::*;

use crate::terrain::Terrain;

pub struct PlanarTerrainConfig {
    width: u32,
    height: u32,
}

pub struct PlanarTerrain {
    config: PlanarTerrainConfig,
    texture_maps: Vec<u32>,
}

impl Default for PlanarTerrain {
    fn default() -> Self {
        Self {
            config: PlanarTerrainConfig {
                width: 1,
                height: 1,
            },
            texture_maps: vec![],
        }
    }
}

impl Terrain<PlanarTerrainConfig> for PlanarTerrain {
    fn new(config: PlanarTerrainConfig) -> Self {
        Self {
            config: config,
            texture_maps: vec![],
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

    fn mesh(&self) {
        todo!()
    }
}
