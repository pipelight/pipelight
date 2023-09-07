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
    pub fn full(&mut self, root: &str) -> Self {
        let e = LoggerArgs {
            internals: LogFile {
                directory: format!("{}/{}", root, LoggerArgs::default().internals.directory),
                level: self.internals.level,
                ..LoggerArgs::default().internals
            },
            pipelines: LogFile {
                directory: format!("{}/{}", root, LoggerArgs::default().pipelines.directory),
                level: self.pipelines.level,
                ..LoggerArgs::default().pipelines
            },
        };
        let config = config::default_set_file(e.clone());
        self.handle.set_config(config);
        self.internals = e.internals;
        self.pipelines = e.pipelines;
        self.to_owned()
    }
    pub fn early() -> Self {
        let e = LoggerArgs::default();
        let config = config::default(e.clone());
        let handle = log4rs::init_config(config).unwrap();
        Logger {
            handle,
            internals: e.internals,
            pipelines: e.pipelines,
        }
    }
}
