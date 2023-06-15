use log::Level;
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
use miette::{miette, Diagnostic, Error, IntoDiagnostic, NamedSource, Report, Result, SourceSpan};

pub mod config;
pub mod default;

pub static logger: Lazy<Arc<Mutex<Logger>>> = Lazy::new(|| Arc::new(Mutex::new(Logger::new())));

#[derive(Debug, Clone)]
pub struct Logger {
    pub directory: String,
    pub handle: Handle,
    pub level: LevelFilter,
}

impl Logger {
    /// Set log level and logging file, and return handler to change logLevels at runtime
    pub fn file(&self, uuid: &Uuid) -> Self {
        let message = "Setting pipeline log file";
        // info!("{}", message);
        let level = self.level;
        let config = config::default_with_file(&self.directory, &level, uuid);
        self.handle.set_config(config);
        return self.to_owned();
    }

    pub fn level(&mut self, level: &LevelFilter) -> Self {
        let config = config::default(level);
        self.handle.set_config(config);
        self.level = level.to_owned();
        return self.to_owned();
    }

    /// Get handler to change logLevel at runtime
    /// Delete logs directory
    pub fn clear(&self) -> Result<()> {
        fs::remove_dir_all(&self.directory).into_diagnostic()?;
        Ok(())
    }
}
