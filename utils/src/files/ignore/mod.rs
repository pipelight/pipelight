// Test
mod test;
// Struct
use crate::files::types::Ignore;
// Ignore file
pub use ignore::gitignore::{Gitignore, GitignoreBuilder};
use ignore_files::{IgnoreFile, IgnoreFilter};
use project_origins::{origins, types, ProjectType};
// Watchexec filter
use watchexec::{error::RuntimeError, filter::Filterer};
use watchexec_events::{Event, Priority, Tag};
// Filesystem manipulatoion
use std::path::{Path, PathBuf};
// Env
use std::env;
// Global vars
use once_cell::sync::Lazy;
use std::sync::{Arc, Mutex};
// Error Handling
use log::{error, info, trace, warn};
use miette::{Error, IntoDiagnostic, Result};

pub static IGNORE: Lazy<Arc<Mutex<Option<Ignore>>>> = Lazy::new(|| Arc::new(Mutex::new(None)));

impl Ignore {
    pub fn new(path: &str) -> Result<Self> {
        let path = Path::new(path);
        // Set Filter
        let file = IgnoreFile {
            path: path.into(),
            applies_in: None,
            applies_to: None,
        };

        let filter: IgnoreFilter = IgnoreFilter::empty(env::current_dir().into_diagnostic()?)
            .add_file(&file)
            .into_diagnostic()?;

        // let filter: Gitignore = GitignoreBuilder::new(path)
        // Add pipelight generated files to ignore list
        // .add_line(None, "/.pipelight")
        // .into_diagnostic()?
        // .add_line(None, ".pipelight")
        // .into_diagnostic()?
        // .build()
        // .into_diagnostic()?;

        let e = Ignore { filter: filter };
        *IGNORE.lock().unwrap() = Some(e.clone());
        Ok(e)
    }
}
impl Filterer for Ignore {
    fn check_event(&self, event: &Event, _priority: Priority) -> Result<bool, RuntimeError> {
        let is = event.paths().any(|(path, _)| {
            // Globbing pattern matching
            let glob_match = self.filter.matched(path, path.is_dir());
            if glob_match.is_whitelist() {
                return glob_match.is_whitelist();
            } else {
                return glob_match.is_none();
            }
        });
        Ok(is)
    }
}
