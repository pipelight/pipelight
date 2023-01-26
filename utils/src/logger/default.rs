use super::config;
use super::Logger;
use crate::git::Git;
pub use log::{error, trace, LevelFilter};
use std::env;
use std::env::current_dir;

impl Default for Logger {
    fn default() -> Self {
        // Get Path With default values
        let level = LevelFilter::Error;
        let directory = ".pipelight/logs";
        let pwd = current_dir().unwrap();
        let path_string = format!("{}/{}", &pwd.display().to_string(), directory);

        // Get default config
        let config = config::default(&level);
        let handle = log4rs::init_config(config).expect("Couldn't init logger");

        let logger = Logger {
            directory: path_string.clone(),
            handle: handle,
            level: LevelFilter::Error,
        };

        return logger;
    }
}
impl Logger {
    pub fn new() -> Self {
        let origin = env::current_dir().unwrap();

        Git::new().teleport();
        let logger = Self::default();

        env::set_current_dir(origin).unwrap();
        return logger;
    }
}
