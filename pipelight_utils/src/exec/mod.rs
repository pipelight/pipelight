//! # pipelight::exec
//!
//! A crate for easy process management.
//!
//! Features:
//!
//! - Spawn and Kill background processes.
//! - Retrieve standard outputs as the process runs.
//! - Get execution time and other metrics.
//!
//! - Interoperability with the most known rustix and sysinfo crates.
//!
//! ## Example
//!
//! Spawn a simple process in the background.
//!

// Rules
#![allow(unused_variables)]
// #![allow(unused_imports)]
#![allow(unused_must_use)]

// Internal Imports
mod globals;
mod io;
mod process;
mod state;

// Re-export
pub use io::*;
pub use process::*;
pub use state::statuable::Statuable;
pub use state::*;
