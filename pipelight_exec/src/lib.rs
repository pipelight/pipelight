//! ## Warning - Crate is not stable.
//!
//! The API is not stabilized and much likely to change.
//!
//! - v0.4 : Many breaking changes.
//!
//! ## About
//!
//! pipelight_exec is a crate for easy process management (on Unix systems).
//!
//! It makes a best effort to leverage
//! the [standard library process crate](https://doc.rust-lang.org/std/process/struct.Command.html).
//!
//! Features:
//!
//! - Get process execution time.
//! - Spawn and Kill background processes.
//! - Display a running process standard inputs/outputs.
//!
//! - Interoperability with [rustix](https://docs.rs/rustix/latest/rustix/)
//!   and [sysinfo](https://docs.rs/sysinfo/latest/sysinfo/)
//!   crates.
//!
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
//! ## Beware - Unconventional process management
//!
//! In linux, a process inputs and outputs are
//! exposed at /proc/<process_id>/fd/
//! and are deleted as soon as the process finishes its execution.
//!
//! A very good straightforward rustacean overview at [procfs crate](https://docs.rs/procfs/0.17.0/procfs/)
//!
//! To keep track of running and dead processes, this crate can
//! redirect some process i/o into its own managed files.
//! Pipelight managed processes are stored in a the .pipelight/proc/ directory.
//! It has the same structure as your /proc/.
//!
//! ###
//!
//! To ease ones developer life, standard outputs are poll, read and finally to stored as text files.
//! /proc/<pid>/2 (file descriptor / buffer)-> ./pipelight/proc/<uuid>/2(text file)
//!
//!
//! For example,
//! The following code runs a process whose outputs are redirected
//! to pipelight temporary filesystem thanks to the `fs()` method.
//!
//! ```rust
//! # use miette::Report;
//!
//! let proc = Process::new().stdin("pwd").fs().run()?;
//! let stout: String = proc.stdout()?;
//!
//! # Ok::<(), Report>(())
//! ```
//! It allows us to read a process outputs **during** and long **after** execution.
//!
// Rules
// #![allow(unused_variables)]
#![allow(unused_imports)]
// #![allow(unused_must_use)]

pub mod dates;
mod globals;
mod io;
mod process;
mod state;

// Re-export
pub use io::*;
pub use process::*;
pub use state::statuable::Statuable;
pub use state::*;
