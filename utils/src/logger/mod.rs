pub use log::{debug, error, info, trace, warn, LevelFilter, SetLoggerError};

mod config_abstraction;
mod default;
mod methods;
mod types;

// Re-export
pub use types::*;
