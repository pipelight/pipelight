pub use log::{debug, error, info, trace, warn, LevelFilter, SetLoggerError};
// Fylesystem
use std::{env, fs};
// Error Handling
use miette::{IntoDiagnostic, Result};

// Re-export

#[derive(Debug, Clone)]
pub struct Logger {
    pub pipelines: LogInfo,
    pub internals: LogInfo,
}
#[derive(Debug, Clone)]
pub struct LogInfo {
    pub file_info: Option<LogFile>,
    pub level: LevelFilter,
}
#[derive(Debug, Clone)]
pub struct LogFile {
    pub directory: String,
    pub name: String,
}

impl Default for Logger {
    fn default() -> Self {
        let logger = Logger {
            internals: LogInfo {
                file_info: None,
                level: LevelFilter::Error,
            },
            pipelines: LogInfo {
                file_info: None,
                level: LevelFilter::Error,
            },
        };
        logger
    }
}

impl Logger {
    pub fn set_internal_level(&mut self, level: &LevelFilter) -> Result<Self> {
        self.internals.level = level.to_owned();
        Ok(self.to_owned())
    }
    pub fn set_level(&mut self, level: &LevelFilter) -> Result<Self> {
        self.pipelines.level = level.to_owned();
        Ok(self.to_owned())
    }
}
