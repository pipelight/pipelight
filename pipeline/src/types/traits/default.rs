use crate::cast;
use crate::types::Config;
use utils::log::Logs;

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
        let mut config = Config::from(&json);
        config.logs = Some(Logs::new());
        return config;
    }
}
