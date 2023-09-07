pub use log::{debug, error, info, trace, warn, LevelFilter, SetLoggerError};
use log4rs;
use log4rs::Handle;
use std::clone::Clone;
use std::fs;
use uuid::Uuid;

// Error Handling
use miette::{IntoDiagnostic, Result};

pub mod config;
pub mod default;

#[derive(Debug, Clone)]
pub struct Logger {
    pub pipelines: LogFile,
    pub internals: LogFile,
    pub handle: Handle,
}
#[derive(Debug, Clone)]
pub struct LogFile {
    pub directory: String,
    pub name: String,
    pub level: LevelFilter,
}
#[derive(Debug, Clone)]
pub struct LoggerArgs {
    pipelines: LogFile,
    internals: LogFile,
}

impl Logger {
    pub fn internal_level(&mut self, level: &LevelFilter) -> Self {
        let e = LoggerArgs {
            internals: LogFile {
                level: level.to_owned(),
                ..self.internals.clone()
            },
            pipelines: self.pipelines.clone(),
        };
        let config = config::default_set_file(e.clone());
        self.handle.set_config(config);
        self.internals = e.internals;
        self.pipelines = e.pipelines;
        self.to_owned()
    }
    pub fn level(&mut self, level: &LevelFilter) -> Self {
        let e = LoggerArgs {
            pipelines: LogFile {
                level: level.to_owned(),
                ..self.pipelines.clone()
            },
            internals: self.internals.clone(),
        };
        let config = config::default_set_file(e.clone());
        self.handle.set_config(config);
        self.internals = e.internals;
        self.pipelines = e.pipelines;
        self.to_owned()
    }
    /// Set log level and logging file, and return handler to change logLevels at runtime
    pub fn file(&mut self, uuid: &Uuid) -> Self {
        let e = LoggerArgs {
            pipelines: LogFile {
                name: uuid.to_string(),
                ..self.pipelines.clone()
            },
            internals: self.internals.clone(),
        };
        let config = config::default_set_file(e.clone());
        self.handle.set_config(config);
        self.internals = e.internals;
        self.pipelines = e.pipelines;
        self.to_owned()
    }

    /// Get handler to change logLevel at runtime
    /// Delete logs directory
    pub fn clear(&self) -> Result<()> {
        fs::remove_dir_all(&self.pipelines.directory).into_diagnostic()?;
        let message = format!("Soft delete directory {}", &self.pipelines.directory);
        trace!("{}", message);
        fs::remove_dir_all(&self.internals.directory).into_diagnostic()?;
        let message = format!("Soft delete directory {}", &self.internals.directory);
        trace!("{}", message);
        Ok(())
    }
}
