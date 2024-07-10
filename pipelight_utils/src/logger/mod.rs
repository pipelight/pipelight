pub use log::{debug, error, info, trace, warn, LevelFilter, SetLoggerError};

mod config_abstraction;
mod default;
mod methods;
pub mod types;

// Re-export
pub use types::*;
