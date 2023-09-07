// Relative paths
use super::config;
use super::{LogFile, Logger, LoggerArgs};

use log::LevelFilter;

// Absolute paths
// use crate::git::Git;
// use crate::teleport::Teleport;
// use std::path::Path;

impl Default for LoggerArgs {
    fn default() -> Self {
        LoggerArgs {
            internals: LogFile {
                name: "_unlinked".to_owned(),
                directory: ".pipelight/_internals/logs".to_owned(),
                level: LevelFilter::Error,
            },
            pipelines: LogFile {
                name: "_unlinked".to_owned(),
                directory: ".pipelight/logs".to_owned(),
                level: LevelFilter::Error,
            },
        }
    }
}
impl Default for Logger {
    fn default() -> Self {
        Self::early()
    }
}

impl Logger {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn full(&self) -> Self {
        let mut e = LoggerArgs::default();
        e.pipelines.level = self.pipelines.level;
        e.internals.level = self.internals.level;
        let config = config::default_set_file(e.clone());
        self.handle.set_config(config);
        Logger {
            handle: self.handle.to_owned(),
            ..self.clone()
        }
    }
    pub fn early() -> Self {
        let e = LoggerArgs::default();
        let config = config::default(e.clone());
        let handle = log4rs::init_config(config).unwrap();
        Logger {
            handle,
            internals: e.internals.clone(),
            pipelines: e.pipelines.clone(),
        }
    }
}
