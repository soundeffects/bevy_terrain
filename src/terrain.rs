pub trait Terrain<Config>: Default {
    fn new(config: Config) -> Self;
    fn config(&self) -> &Config;
    fn set_config(&mut self, config: Config);
    fn write(&self);
    fn sample(&self);
    fn mesh(&self);
}
