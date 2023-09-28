// Module description
//
// This module read config files (ex: pipelight.<file_extension>)
//
// Files are converted into intermediate rust structs with the serde crate.
// Those intermediate structs are practical to define a config file

mod config;
mod error;
mod logs;

// Re-export
pub use config::types::*;
pub use logs::Logs;
pub use logs::*;
