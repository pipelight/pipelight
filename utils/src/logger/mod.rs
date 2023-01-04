// use crate::types::logs::PipelineLog;
pub use log::Level;
pub use log::{debug, error, info, trace, warn, LevelFilter, SetLoggerError};
use log4rs;
use log4rs::Handle;
use std::error::Error;
use std::fs;
use std::path::Path;
use uuid::Uuid;
pub mod config;
use std::clone::Clone;

#[derive(Debug, Clone)]
pub struct Logs {
    path: String,
    level: LevelFilter,
    handle: Option<Handle>,
}
impl Logs {
    pub fn new() -> Self {
        return Logs {
            path: "./.pipelight/logs".to_owned(),
            level: LevelFilter::Trace,
            handle: None,
        };
    }
    /// Set log level and return handler to change logLevels at runtime
    pub fn set_file(&mut self, level: &LevelFilter, uuid: Uuid) -> Self {
        self.ensure().unwrap();
        self.level = level.to_owned();
        let config = config::file(self.level, uuid);
        let handle = log4rs::init_config(config).unwrap();
        self.handle = Some(handle);
        return self.to_owned();
    }
    /// Set log level and return handler to change logLevels at runtime
    pub fn set(&mut self, level: &LevelFilter) -> Self {
        self.ensure().unwrap();
        self.level = level.to_owned();
        let config = config::default(self.level);
        self.handle = Some(log4rs::init_config(config).unwrap());
        return self.to_owned();
    }
    /// Get handler to change logLevels at runtime
    pub fn get(&self) -> Self {
        self.ensure().unwrap();
        return self.to_owned();
    }
    /// Delete logs directory
    pub fn clear() -> Result<(), Box<dyn Error>> {
        let path = Path::new(".pipelight/logs");
        fs::remove_dir_all(path)?;
        Ok(())
    }
    /// Ensure log directory
    fn ensure(&self) -> Result<(), Box<dyn Error>> {
        let path = Path::new(&self.path);
        fs::create_dir_all(path)?;
        Ok(())
    }
}
