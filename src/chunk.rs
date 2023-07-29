use bevy::prelude::*;

const CHUNK2_DIM: usize = 64;

#[derive(Component)]
pub struct Chunk2 {
    data: [u8; CHUNK2_DIM * CHUNK2_DIM],
}

impl Chunk2 {
    pub const DIM: usize = CHUNK2_DIM;

    pub fn new() -> Self {
        Self {
            data: [0; CHUNK2_DIM * CHUNK2_DIM],
        }
    }

    pub fn write(&mut self, x: usize, y: usize, val: u8) {
        self.data[Self::linearize(x, y)] = val;
    }

    pub fn sample(&self, x: usize, y: usize) -> u8 {
        self.data[Self::linearize(x, y)]
    }

    pub fn linearize(x: usize, y: usize) -> usize {
        x + (y * Self::DIM)
    }
}

const CHUNK3_DIM: usize = 16;

#[derive(Component)]
pub struct Chunk3 {
    data: [u8; CHUNK3_DIM * CHUNK3_DIM * CHUNK3_DIM],
}

impl Chunk3 {
    pub const DIM: usize = CHUNK3_DIM;

    pub fn new() -> Self {
        Self {
            data: [0; CHUNK3_DIM * CHUNK3_DIM * CHUNK3_DIM],
        }
    }

    pub fn write(&mut self, x: usize, y: usize, z: usize, val: u8) {
        self.data[Self::linearize(x, y, z)] = val;
    }

    pub fn sample(&self, x: usize, y: usize, z: usize) -> u8 {
        self.data[Self::linearize(x, y, z)]
    }

    pub fn linearize(x: usize, y: usize, z: usize) -> usize {
        x + (y * Self::DIM) + (z * Self::DIM * Self::DIM)
    }
}
