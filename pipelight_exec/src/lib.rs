//! ## Warning - Crate is not stable.
//!
//! The API is not stabilized and much likely to change.
//!
//! It is used internaly in a lightweight cicd engine:
//! https://github.com/pipelight/pipelight
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
//! ## Spawn a process.
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
//! // Runs a child process and wait until execution is over.
//! let mut process = Process::new()
//!     .stdin("echo test")
//! process.run()?;
//!
//! ```
//!
//! Runs child process in the background.
//! Do not wait until process is over and return as soon as child is spawned.
//!
//! ```rust
//! let mut process = Process::new()
//!     .stdin("echo test")
//!     .background();
//! process.run()?;
//! ```
//!
//! Runs a disowned child process that won't be killed if parent is killed.
//! Do not wait until process is over and return as soon as child is spawned.
//! ```rust
//! let mut process = Process::new()
//!     .stdin("echo test")
//!     .detach();
//! process.run()?;
//! ```
//!
//! Runs a disowned child process that won't be killed if parent is killed.
//! and stores process outputs in ./.pipelight/proc/<uuid>/
//! Practical if you want to process child output from another program.
//!
//! ```rust
//! let mut process = Process::new()
//!     .stdin("echo test")
//!     .fs()
//!     .detach();
//! process.run()?;
//!
//! # Ok::<(), Report>(())
//! ```
//!
//! Read a process i/o.
//!
//! ```rust
//! let mut process = Process::new()
//!     .stdin("echo test")
//!     .detach();
//! process.run()?;
//!
//! process.stdout()?;
//!
//! # Ok::<(), Report>(())
//! ```
//!
//!
//! ## Find a process.
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
//! However, to ease ones developer life, standard outputs are polled, read and finally stored as text files.
//! /proc/<pid>/2 (file descriptor / buffer)-> ./pipelight/proc/<uuid>/2(text file)
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
