use super::Config;

impl Default for Config {
    fn default() -> Self {
        Config { pipelines: None }
    }
}
impl Config {
    pub fn new() -> Self {
        Config::default()
    }
}
