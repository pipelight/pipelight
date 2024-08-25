//! ## File - parse file types with pretty diagnostics.
//!
//! Well structured parsing error reports with the language specific error types.
//! thanks to the [thiserror](https://docs.rs/thiserror/latest/thiserror/) and
//! [miette](https://docs.rs/miette/latest/miette/index.html) crate.
//!
//! Let say you want to deserialize to a Config struct.
//!
//! ```rust
//! # use miette::Result;
//! # use serde_json::Value;
//! # use pipelight_files::{YamlError,TomlError,CastError};
//!
//! # fn main () -> Result<(), CastError> {
//! # let string = "";
//!
//! let res = serde_yaml::from_str::<Value>(&string);
//! match res {
//!     Ok(res) => {
//!         // do things
//!     },
//!     Err(e) => {
//!         let err = YamlError::new(e, &string);
//!         return Err(err.into());
//!     }
//! };
//!
//! # Ok(())
//! # }
//! ```
//!
//! ```rust
//! # use miette::Result;
//! # use serde_json::Value;
//! # use pipelight_files::{YamlError,TomlError,CastError};
//!
//! # fn main () -> Result<(), CastError> {
//! # let string = "";
//!
//! let res = toml::from_str::<Value>(&string);
//! match res {
//!     Ok(res) => {
//!         // do things
//!     },
//!     Err(e) => {
//!         let err = TomlError::new(e, &string);
//!         return Err(err.into());
//!     }
//! };
//!
//! # Ok(())
//! # }
//! ```
mod from;

// Re-export types
mod error;
pub use error::*;

mod is;
pub use is::*;

mod methods;
pub use methods::*;

mod types;
pub use types::*;
