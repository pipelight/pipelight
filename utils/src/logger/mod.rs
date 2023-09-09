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
    pub handle: Option<Handle>,
    pub pipelines: LogInfo,
    pub internals: LogInfo,
}
#[derive(Debug, Clone)]
pub struct LogInfo {
    file_info: Option<LogFile>,
    pattern: String,
    name: String,
    pub level: LevelFilter,
}
#[derive(Debug, Clone)]
pub struct LogFile {
    pub directory: String,
    pub name: String,
}

impl Logger {
    pub fn set_internal_level(&mut self, level: &LevelFilter) -> Result<Self> {
        self.internals.level = level.to_owned();
        self.update();
        Ok(self.to_owned())
    }
    pub fn set_level(&mut self, level: &LevelFilter) -> Result<Self> {
        self.pipelines.level = level.to_owned();
        self.update();
        Ok(self.to_owned())
    }

    /// Set log level and logging file, and return handler to change logLevels at runtime
    pub fn set_file(&mut self, uuid: &Uuid) -> Self {
        self.pipelines.file_info = Some(LogFile {
            name: uuid.to_string(),
            ..self.pipelines.file_info.clone().unwrap()
        });
        self.to_owned()
    }
    /// Get handler to change logLevel at runtime
    ///
    /// Delete logs directory
    pub fn clear(&self) -> Result<()> {
        let file_info = &self.pipelines.file_info;
        if let Some(file_info) = file_info {
            let dir = file_info.directory.clone();
            fs::remove_dir_all(&dir).into_diagnostic()?;
            let message = format!("Soft deleted log directory: {}", dir);
            trace!("{}", message);
        };
        let file_info = &self.pipelines.file_info;
        if let Some(file_info) = file_info {
            let dir = file_info.directory.clone();
            fs::remove_dir_all(&dir).into_diagnostic()?;
            let message = format!("Soft deleted internal log directory: {}", dir);
            trace!("{}", message);
        };
        Ok(())
    }
}
