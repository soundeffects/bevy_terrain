use bevy::prelude::*;

pub trait Terrain {
    type Config;

    fn new(config: Self::Config) -> Self;
    fn config(&self) -> &Self::Config;
    fn set_config(&mut self, config: Self::Config);
    fn write(&self);
    fn sample(&self);
    fn generate_mesh(&mut self, meshes: ResMut<Assets<Mesh>>);
    fn mesh(&self) -> Handle<Mesh>;
}
