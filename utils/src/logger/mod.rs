// use crate::types::logs::pipeline;
pub use log::Level;
pub use log::{debug, error, info, trace, warn, LevelFilter, SetLoggerError};
use log4rs;
use log4rs::Handle;
use once_cell::sync::Lazy;
use std::clone::Clone;
use std::error::Error;
use std::fs;
use uuid::Uuid;

pub mod config;
pub mod default;

pub static logger: Lazy<Logger> = Lazy::new(|| Logger::new());

#[derive(Debug, Clone)]
pub struct Logger {
    pub directory: String,
    pub handle: Handle,
}

impl Logger {
    /// Set log level and logging file, and return handler to change logLevels at runtime
    pub fn file(&self, uuid: &Uuid) -> Self {
        let level = LevelFilter::Trace;
        let config = config::default_with_file(&level, uuid);
        self.handle.set_config(config);
        return self.to_owned();
    }
    pub fn level(&self, level: &LevelFilter) -> Self {
        let config = config::default(level);
        self.handle.set_config(config);
        return self.to_owned();
    }
    /// Get handler to change logLevel at runtime
    /// Delete logs directory
    pub fn clear(&self) -> Result<(), Box<dyn Error>> {
        fs::remove_dir_all(&self.directory)?;
        Ok(())
    }
}
