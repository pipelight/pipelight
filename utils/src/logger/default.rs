// Relative paths
use super::config;
use super::{LogFile, LogInfo, Logger};

use log::LevelFilter;

// Absolute paths
// use std::path::Path;
use std::env;

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
        logger.update().unwrap();
        logger
    }
}
impl Logger {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn to_file(&mut self) -> Self {
        let logger = Logger {
            handle: self.handle.clone(),
            internals: LogInfo {
                file_info: Some(LogFile {
                    name: "_unlinked".to_owned(),
                    directory: format!(
                        "{}/.pipelight/_internals/logs",
                        &env::current_dir().unwrap().to_str().unwrap()
                    ),
                }),
                ..self.internals.clone()
            },
            pipelines: LogInfo {
                file_info: Some(LogFile {
                    name: "_unlinked".to_owned(),
                    directory: format!(
                        "{}/.pipelight/logs",
                        &env::current_dir().unwrap().to_str().unwrap()
                    ),
                }),
                ..self.pipelines.clone()
            },
        };
        *self = logger;
        self.update().unwrap();
        self.to_owned()
    }
    pub fn to_stdout(&mut self) -> Self {
        let logger = Logger {
            handle: self.handle.clone(),
            internals: LogInfo {
                file_info: None,
                ..self.internals.clone()
            },
            pipelines: LogInfo {
                file_info: None,
                ..self.pipelines.clone()
            },
        };
        *self = logger;
        self.update().unwrap();
        self.to_owned()
    }
}
