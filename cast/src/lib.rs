// # Module description
//
// This module is to find and read config files (ex: pipelight.<file_extension>)
//
// Files are converted into intermediate rust structs with the serde crate.
// Those intermediate structs are practical to define a config file
// but harsh to use as is in a rust ecosystem.
//
// They are only used here to cast config files
// and are then converted into practical structs to be used outside the crate.

// import modules
mod config;
mod error;
mod test;
mod types;
mod typescript;

// Re-export
pub use types::*;
