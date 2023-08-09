// Relative paths
use super::config;
use super::Logger;

// Absolute paths
use crate::git::Git;
use crate::teleport::Teleport;
use std::path::Path;

use log::{error, trace, LevelFilter};
use std::env;

impl Default for Logger {
    fn default() -> Self {
        // Get Path With default values
        let level = LevelFilter::Error;
        let log_dir = ".pipelight/logs".to_owned();
        let config = config::default(&level);
        let handle = log4rs::init_config(config).expect("Couldn't init logger");

        let exists = Path::new(&log_dir).exists();
        if exists {
            Logger {
                directory: log_dir,
                handle: handle,
                level: LevelFilter::Error,
            }
        } else {
            let root = Teleport::new().origin;
            let path_string = format!("{}/{}", &root, log_dir);
            // Get default config
            Logger {
                directory: path_string.clone(),
                handle: handle,
                level: LevelFilter::Error,
            }
        }
    }
}
impl Logger {
    pub fn new() -> Self {
        Self::default()
    }
}
