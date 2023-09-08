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
        Logger {
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
        }
    }
}
impl Logger {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn full(&mut self) -> Self {
        Logger {
            handle: None,
            internals: LogInfo {
                name: "internals".to_owned(),
                file_info: Some(LogFile {
                    name: "_unlinked".to_owned(),
                    directory: ".pipelight/_internals/logs".to_owned(),
                }),
                pattern: "{d(%Y-%m-%d %H:%M:%S)} | {h({l}):5.5} | {f}:{L} — {m}{n}".to_owned(),
                level: self.internals.level,
            },
            pipelines: LogInfo {
                name: "pipelines".to_owned(),
                file_info: Some(LogFile {
                    name: "_unlinked".to_owned(),
                    directory: ".pipelight/logs".to_owned(),
                }),
                pattern: "{m}{n}".to_owned(),
                level: self.pipelines.level,
            },
        }
    }
    pub fn partial(&mut self) -> Self {
        Logger {
            handle: None,
            internals: LogInfo {
                name: "internals".to_owned(),
                file_info: None,
                pattern: "{d(%Y-%m-%d %H:%M:%S)} | {h({l}):5.5} | {f}:{L} — {m}{n}".to_owned(),
                level: self.internals.level,
            },
            pipelines: LogInfo {
                name: "pipelines".to_owned(),
                file_info: None,
                pattern: "{m}{n}".to_owned(),
                level: self.pipelines.level,
            },
        }
    }
}
