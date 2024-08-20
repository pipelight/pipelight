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
//! ```
//! # use miette::Report;
//!
//! let mut portal = Portal::new()?;
//! // Set a pattern to search for.
//! portal.seed("pipelight").search()?;
//!
//! // Get the file path
//! portal.target.file_path.unwrap();
//! // Get the directory path
//! portal.target.directory_path.unwrap();
//!
//! // Go to the target directory
//! portal.teleport()?;
//! // Go back to the origin directory
//! portal.origin()?;
//!
//! # Ok::<(), Report>(())
//! ```
//!
//! ## File - parse file types with pretty diagnostics.
//!
//! Well structured parsing error reports with the language specific error types.
//! thanks to the [thiserror](https://docs.rs/thiserror/latest/thiserror/) and
//! [miette](https://docs.rs/miette/latest/miette/index.html) crate.
//!
//! Let say you want to deserialize to a Config struct.
//! ```
//! # use miette::Report;
//! use cast::Config;
//!
//! let res = serde_yaml::from_str::<Config>(&string);
//! match res {
//!     Ok(res) => Ok(res),
//!     Err(e) => {
//!         let err = YamlError::new(e, &string);
//!         Err(err.into())
//!     }
//! }
//!
//! let res = serde_toml::from_str::<Config>(&string);
//! match res {
//!     Ok(res) => Ok(res),
//!     Err(e) => {
//!         let err = TomlError::new(e, &string);
//!         Err(err.into())
//!     }
//! }
//!
//! # Ok::<(), Report>(())
//! ```
//!
//! <img src="" alt="pretty parsing error report">
//!
//! ## Git - easy git repo manipulation.
//!
//! ```
//! # use miette::Report;
//!
//! let repo = Git::new();
//! let branch = repo.get_branch()?;
//! let tag = repo.get_tag()?;
//! let commit = repo.get_commit()?;
//!
//! # Ok::<(), Report>(())
//! ```

pub mod globals;
// Internal Imports
pub mod dates;
pub mod error;
pub mod files;
pub mod git;
pub mod logger;
pub mod signal;
pub mod teleport;
