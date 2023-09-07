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
        Self::early()
    }
    pub fn full(&self) -> Self {
        let e = LoggerArgs::default();
        let config = config::default_stdout_and_files(e.clone());
        self.handle.set_config(config);
        Logger {
            handle: self.handle.to_owned(),
            internals: e.internals.clone(),
            pipelines: e.pipelines.clone(),
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
