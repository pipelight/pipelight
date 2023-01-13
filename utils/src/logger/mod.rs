// use crate::types::logs::pipeline;
pub use log::Level;
pub use log::{debug, error, info, trace, warn, LevelFilter, SetLoggerError};
use log4rs;
use log4rs::Handle;
use std::clone::Clone;
use std::env::current_dir;
use std::error::Error;
use std::fs;
use std::path::Path;
use uuid::Uuid;

// Global Vars
use once_cell::sync::Lazy;
use std::sync::Mutex;

pub mod config;
pub mod default;

#[derive(Debug, Clone)]
pub struct Logger {
    pub directory: String,
    level: LevelFilter,
    pub handle: Option<Handle>,
}

const LOGGER: Lazy<Mutex<Logger>> = Lazy::new(|| Mutex::new(Logger::new()));

impl Logger {
    /// Set log level and logging file, and return handler to change logLevels at runtime
    pub fn file(uuid: &Uuid) -> Self {
        let binding = LOGGER;
        let mut global = binding.lock().unwrap();
        let config = config::default_with_file(&global.level, uuid);
        let handle = log4rs::init_config(config).unwrap();
        global.handle = Some(handle);
        return global.to_owned();
    }
    pub fn level(level: &LevelFilter) -> Result<Logger, Box<dyn Error>> {
        let binding = LOGGER;
        let mut global = binding.lock().unwrap();
        Logger::ensure().unwrap();
        global.level = level.to_owned();
        let config = config::default(level);
        global.handle = Some(log4rs::init_config(config).unwrap());
        Ok(global.to_owned())
    }
    /// Get handler to change logLevel at runtime
    pub fn get() -> Logger {
        return LOGGER.lock().unwrap().to_owned();
    }
    /// Delete logs directory
    pub fn clear() -> Result<(), Box<dyn Error>> {
        let binding = LOGGER;
        let global = binding.lock().unwrap();
        let pwd = current_dir().unwrap();
        let string = format!("{}/{}", &pwd.display().to_string(), global.directory);
        let path = Path::new(&string);
        fs::remove_dir_all(path)?;
        Ok(())
    }
    /// Ensure log directory
    pub fn ensure() -> Result<(), Box<dyn Error>> {
        let pwd = current_dir().unwrap();
        let string = format!(
            "{}/{}",
            &pwd.display().to_string(),
            LOGGER.lock().unwrap().directory
        );
        let path = Path::new(&string);
        fs::create_dir_all(path)?;
        Ok(())
    }
}
