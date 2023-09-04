pub use log::{debug, error, info, trace, warn, LevelFilter, SetLoggerError};
use log4rs;
use log4rs::Handle;
use std::clone::Clone;
use std::fs;
use uuid::Uuid;

// Global var
// use arc_swap::ArcSwap;
use once_cell::sync::Lazy;
use std::sync::{Arc, Mutex};

// Error Handling
use miette::{IntoDiagnostic, Result};

pub mod config;
pub mod default;

pub static logger: Lazy<Arc<Mutex<Logger>>> = Lazy::new(|| Arc::new(Mutex::new(Logger::new())));

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
        let args = LoggerArgs {
            internals: LogFile {
                level: level.to_owned(),
                ..self.internals.clone()
            },
            pipelines: self.pipelines.clone(),
        };
        let config = config::default_with_file(args.clone());
        self.handle.set_config(config);
        self.internals = args.internals;
        self.pipelines = args.pipelines;
        self.to_owned()
    }
    pub fn level(&mut self, level: &LevelFilter) -> Self {
        let args = LoggerArgs {
            pipelines: LogFile {
                level: level.to_owned(),
                ..self.pipelines.clone()
            },
            internals: self.internals.clone(),
        };
        let config = config::default_with_file(args.clone());
        self.handle.set_config(config);
        self.internals = args.internals;
        self.pipelines = args.pipelines;
        self.to_owned()
    }
    /// Set log level and logging file, and return handler to change logLevels at runtime
    pub fn file(&self, uuid: &Uuid) -> Self {
        let mut args = LoggerArgs::default();
        args.pipelines.name = uuid.to_string();
        let config = config::default_with_file(args);
        self.handle.set_config(config);
        self.to_owned()
    }

    /// Get handler to change logLevel at runtime
    /// Delete logs directory
    pub fn clear(&self) -> Result<()> {
        fs::remove_dir_all(&self.pipelines.directory).into_diagnostic()?;
        Ok(())
    }
}
