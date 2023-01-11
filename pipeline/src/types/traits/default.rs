use crate::cast;
use crate::types::Config;

impl Default for Config {
    fn default() -> Self {
        Config {
            pipelines: None,
            logs: None,
            hooks: None,
        }
    }
}

impl Config {
    pub fn new() -> Self {
        let json = cast::Config::new();
        let config = Config::from(&json);
        return config;
    }
}
