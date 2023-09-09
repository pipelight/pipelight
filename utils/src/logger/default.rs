// Relative paths
use super::config;
use super::{LogFile, LogInfo, Logger};

use log::LevelFilter;

// Absolute paths
// use crate::git::Git;
// use crate::teleport::Teleport;
// use std::path::Path;

impl Default for Logger {
    fn default() -> Self {
        let mut logger = Logger {
            handle: None,
            internals: LogInfo {
                name: "internals".to_owned(),
                file_info: None,
                pattern: "{d(%Y-%m-%d %H:%M:%S)} | {h({l}):5.5} | {f}:{L} — {m}{n}".to_owned(),
                level: LevelFilter::Error,
            },
            pipelines: LogInfo {
                name: "pipelines".to_owned(),
                file_info: None,
                pattern: "{m}{n}".to_owned(),
                level: LevelFilter::Error,
            },
        };
        logger.update();
        logger
    }
}
impl Logger {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn to_file(&mut self) -> Self {
        let logger = Logger {
            handle: None,
            internals: LogInfo {
                file_info: Some(LogFile {
                    name: "_unlinked".to_owned(),
                    directory: ".pipelight/_internals/logs".to_owned(),
                }),
                ..self.internals.clone()
            },
            pipelines: LogInfo {
                file_info: Some(LogFile {
                    name: "_unlinked".to_owned(),
                    directory: ".pipelight/logs".to_owned(),
                }),
                ..self.pipelines.clone()
            },
        };
        self.update();
        logger
    }
    pub fn to_stdout(&mut self) -> Self {
        let logger = Logger {
            handle: None,
            internals: LogInfo {
                file_info: None,
                ..self.internals.clone()
            },
            pipelines: LogInfo {
                file_info: None,
                ..self.pipelines.clone()
            },
        };
        self.update();
        logger
    }
}
