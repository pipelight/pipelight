//modules
mod case;
pub mod print;
pub mod prompt;
pub mod traits;
pub mod types;

// Re-export
pub use case::get_args;

// New and Default methods
// use super::interface::traits::default;

use super::Cli;

// Global vars
use once_cell::sync::Lazy;

pub static mut CLI: Lazy<Cli> = Lazy::new(|| Cli::new());
