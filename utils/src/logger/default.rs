// Relative paths
use super::config;
use super::Logger;

// Absolute paths
use crate::git::Git;
use crate::teleport::Teleport;

use log::{error, trace, LevelFilter};
use std::env;

impl Default for Logger {
    fn default() -> Self {
        // Get Path With default values
        let level = LevelFilter::Error;
        let log_dir = ".pipelight/logs";

        let root = Teleport::new().root.unwrap();
        let path_string = format!("{}/{}", &root, log_dir);

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

        let mut portal = Teleport::new();
        portal.teleport();
        let logger = Self::default();
        portal.teleport();

        return logger;
    }
}
