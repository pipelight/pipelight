//! !! API not stabilized - lacking documentation - do not use !!
//!
//! ## About
//!
//! Pipelight_exec is a crate for easy process management.
//! It makes a best effort to leverage standard library.
//!
//! Features:
//!
//! - Spawn and Kill background processes.
//! - Display a detached running process standard outputs.
//! - Get process execution time.
//!
//! - Interoperability with [rustix](https://docs.rs/rustix/latest/rustix/)
//!   and [sysinfo](https://docs.rs/sysinfo/latest/sysinfo/)
//!   crates.
//!
//! ## Example
//!
//! Spawn a simple process in the background.
//! or in other words, execute a process and detach it.
//!
//! It keeps running after parent process exit and terminal exit.
//!
//! ```rust
//! # use pipelight_exec::Process;
//! # use miette::Report;
//!
//! let mut process = Process::new("echo test");
//! process.run_detached()?;
//!
//! # Ok::<(), Report>(())
//! ```
//!
//! Pipe the process standards outputs to the parent.
//!
//! ```rust
//! # use pipelight_exec::Process;
//! # use miette::Report;
//!
//! let mut process = Process::new("echo test");
//! process.run_detached()?;
//!
//! # Ok::<(), Report>(())
//! ```
//!
//! Find a running process, with handy search options.
//!
//! ```rust
//! # use pipelight_exec::Finder;
//! # use miette::Report;
//!
//! let process_finder = Finder::new().seed("my_proc").root("/my/dir").search()?;
//!
//! let pid = 1792;
//! let process_finder = Finder::new().pid(&pid).search()?;
//!
//! # Ok::<(), Report>(())
//! ```
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
