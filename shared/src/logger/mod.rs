pub use log::Level::{Debug, Trace};
pub use log::{debug, error, info, trace, warn, LevelFilter, SetLoggerError};
use log4rs;
use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Config, Logger, Root};
use log4rs::encode::pattern::PatternEncoder;
use log4rs::Handle;
use project_root::get_project_root;
use std::error::Error;
use std::fs;
use std::path::Path;
use uuid::{uuid, Uuid};
pub mod config;

/// Create logs directory
pub fn ensure_log_directory() -> Result<(), Box<dyn Error>> {
    let path = Path::new("./.pipelight/logs");
    fs::create_dir_all(path)?;
    Ok(())
}
/// Delete logs directory
pub fn clear_logs() -> Result<(), Box<dyn Error>> {
    let path = Path::new(".pipelight/logs");
    fs::remove_dir_all(path)?;
    Ok(())
}

/// Set loggers and return handler to change logLevels at runtime
pub fn set_logger(level: LevelFilter) -> Result<Handle, Box<dyn Error>> {
    ensure_log_directory()?;
    let config = config::set(level)?;
    // use handle to change logger configuration at runtime
    let handle = log4rs::init_config(config).unwrap();
    Ok(handle)
}
