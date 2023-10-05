// Test
mod test;
// Struct
use crate::files::types::Ignore;
// Ignore file
pub use ignore::gitignore::{Gitignore, GitignoreBuilder};
use project_origins::{origins, types, ProjectType};
// Watchexec filter
use watchexec::{error::RuntimeError, filter::Filterer};
use watchexec_events::{Event, Priority, Tag};
// Filesystem manipulatoion
use std::path::Path;
// Global vars
use once_cell::sync::Lazy;
use std::sync::{Arc, Mutex};
// Error Handling
use miette::{Error, IntoDiagnostic, Result};

pub static LOGGER: Lazy<Arc<Mutex<Option<Ignore>>>> = Lazy::new(|| Arc::new(Mutex::new(None)));

impl Ignore {
    pub fn new(path: &str) -> Result<Self> {
        let path = Path::new(path);
        let ignore_dir = ".pipelight/";
        let filter: Gitignore = GitignoreBuilder::new(path)
            // Add pipelight generated files to ignore list
            .add_line(None, ignore_dir)
            .into_diagnostic()?
            .build()
            .into_diagnostic()?;

        let e = Ignore { filter };
        *LOGGER.lock().unwrap() = Some(e.clone());

        Ok(e)
    }
}
impl Filterer for Ignore {
    fn check_event(&self, event: &Event, _priority: Priority) -> Result<bool, RuntimeError> {
        let is_watched = event.paths().any(|(path, _)| {
            let glob_match = self.filter.matched(path, path.is_dir());
            return glob_match.is_whitelist();
        });
        Ok(is_watched)
    }
}
