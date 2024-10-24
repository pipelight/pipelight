mod build;
mod is;
// Test
mod test;

use std::sync::Arc;
use watchexec::{action::ActionHandler, filter::Filterer, Config, Watchexec};

// Reexport
pub use build::*;

#[derive(Debug, Clone, Default)]
pub struct Watcher {
    pub config: Arc<Config>,
}
