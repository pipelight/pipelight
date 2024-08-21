//!
//! This crate cast config files (ex: pipelight.<file_extension>) into dummy structs.
//! Files are converted into intermediate rust structs by the well-known rust serde crate.
//!
mod config;
mod error;
mod logs;

// Re-export
pub use config::types::*;
pub use logs::Logs;
pub use logs::*;
