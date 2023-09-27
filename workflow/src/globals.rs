// Global vars
use once_cell::sync::Lazy;
use std::sync::{Arc, Mutex};
// Types
use crate::{Config, Trigger};
// Error Handling
use log::{info, trace, LevelFilter};
use miette::{IntoDiagnostic, Result};
use utils::logger::Logger;

pub static LOGGER: Lazy<Arc<Mutex<Logger>>> = Lazy::new(|| Arc::new(Mutex::new(Logger::new())));
pub static mut CONFIG: Lazy<Config> = Lazy::new(Config::default);

// Hydrate logs
pub fn full_hydrate_logger() -> Result<()> {
    // Set internal verbosity level
    let verbosity = LevelFilter::Error;
    LOGGER.lock().unwrap().set_level(&verbosity)?;
    // Set verbosity level
    let verbosity = LevelFilter::Error;
    LOGGER.lock().unwrap().set_internal_level(&verbosity)?;
    LOGGER.lock().unwrap().to_file();
    Ok(())
}

// Set every main globals
pub fn default_globals() -> Result<()> {
    full_hydrate_logger()?;
    trace!("Init globals to default");
    Ok(())
}
// Set every main globals
pub fn set_globals(config: Option<Config>, logger: Option<Logger>) -> Result<()> {
    if let Some(config) = config {
        unsafe { *CONFIG = config }
    }
    if let Some(logger) = logger {
        *LOGGER.lock().unwrap() = logger;
    }
    trace!("Set globals");
    Ok(())
}

impl Config {
    pub fn get() -> Result<Self> {
        let config;
        unsafe { config = (*CONFIG).clone() };
        Ok(config)
    }
}
