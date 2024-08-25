//! !! API not stabilized - lacking documentation - do not use !!
//!
//! ## About
//!
//! Pipelight_utils is a crate that gather a set of trivial utilities for command line tools.
//!
//! These are the most cleaned up modules coming from and named after
//! [pipelight](https://github.com/pipelight/pipelight), a tiny automation tool.
//!
//! ## Teleport - find a file recursively.
//!
//! You can browse the filesystem for a configuration file.
//!
//! ```rust
//! # use miette::Report;
//! use pipelight_utils::teleport::Portal;
//!
//! let mut portal = Portal::new()?;
//! // Set a pattern to search for.
//! portal.seed("pipelight").search()?;
//!
//! // Get the file path
//! let file_path = portal.target.file_path.clone().unwrap();
//! // Get the directory path
//! let directory_path = portal.target.directory_path.clone().unwrap();
//!
//! // Go to the target directory
//! portal.teleport()?;
//! // Go back to the origin directory
//! portal.origin()?;
//!
//! # Ok::<(), Report>(())
//! ```
//!
//! <img src="" alt="pretty parsing error report">
//!
//! ## Git - easy git repo manipulation.
//!
//! ```rust
//! # use miette::Report;
//! use pipelight_utils::git::Git;
//!
//! let repo = Git::new();
//! let branch = repo.get_branch()?;
//! let tag = repo.get_tag()?;
//! let commit = repo.get_commit()?;
//!
//! # Ok::<(), Report>(())
//! ```

pub mod globals;

pub mod error;
pub use error::*;

pub mod git;
pub mod logger;
pub mod signal;
pub mod teleport;
