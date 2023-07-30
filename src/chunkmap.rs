use bevy::{prelude::*, utils::HashMap};

use crate::chunk::Chunk3x16;

pub struct ChunkmapPlugin;

impl Plugin for ChunkmapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, drop_chunks);
    }
}

#[derive(Component)]
pub struct Chunkmap {
    chunks: HashMap<UVec3, Chunk3x16>,
    chunk_scale: u8,
    outdated_chunks: Vec<UVec3>,
    channel: u8,
}

impl Chunkmap {
    fn update_from_prev_pos(prev: UVec3, curr: UVec3) {}
}

pub struct Lod(u8, u8);

#[derive(Component)]
pub struct ChunkmapAgent {
    previous_position: UVec3,
    channel: u8,
}

fn mark_chunks_from_agents(
    mut commands: Commands,
    chunkmap_query: Query<&Chunkmap>,
    agent_query: Query<&ChunkmapAgent>,
) {
    for agent in &agent_query {
        let chunkmap = commands.entity(agent.chunkmap);
    }
}
