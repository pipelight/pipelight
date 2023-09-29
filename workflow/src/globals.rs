// Global vars
use once_cell::sync::Lazy;
use std::sync::{Arc, Mutex};
use utils::globals::LOGGER;
// Struct
use crate::{Config, Trigger};
use utils::logger::Logger;
// Error Handling
use log::{trace, LevelFilter};
use miette::Result;

pub static CONFIG: Lazy<Arc<Mutex<Config>>> = Lazy::new(|| Arc::new(Mutex::new(Config::default())));
pub static TRIGGER_ENV: Lazy<Arc<Mutex<Trigger>>> =
    Lazy::new(|| Arc::new(Mutex::new(Trigger::default())));

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
        *CONFIG.lock().unwrap() = config;
    }
    if let Some(logger) = logger {
        *LOGGER.lock().unwrap() = logger;
    }
    trace!("Set globals");
    Ok(())
}

impl Config {
    pub fn get() -> Result<Self> {
        let config = CONFIG.lock().unwrap().clone();
        Ok(config)
    }
}
