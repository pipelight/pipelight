// Test
mod test;
// Struct
use crate::files::types::Ignore;
// Ignore file
use ignore_files::{IgnoreFile, IgnoreFilter};
use project_origins::{origins, types, ProjectType};
// Watchexec filter
use watchexec::{error::RuntimeError, filter::Filterer};
use watchexec_events::{Event, Priority, Tag};
// Filesystem manipulatoion
use std::env;
use std::path::{Path, PathBuf};
// Global vars
use once_cell::sync::Lazy;
use std::sync::{Arc, Mutex};
// Async
use futures::executor::block_on;
// Error Handling
use miette::{Error, IntoDiagnostic, Result};

pub static LOGGER: Lazy<Arc<Mutex<Option<Ignore>>>> = Lazy::new(|| Arc::new(Mutex::new(None)));

impl Ignore {
    pub async fn new(path: &str) -> Result<Self> {
        let path = PathBuf::from(path);
        let origin = env::current_dir().unwrap();

        let files = [IgnoreFile {
            path,
            applies_in: Some(origin.clone()),
            applies_to: None,
        }];

        let filter = IgnoreFilter::new(origin, &files).await.into_diagnostic()?;
        let e = Ignore { filter };

        *LOGGER.lock().unwrap() = Some(e.clone());
        Ok(e)
    }
}
impl Filterer for Ignore {
    fn check_event(&self, event: &Event, _priority: Priority) -> Result<bool, RuntimeError> {
        let mut is_watched = false;
        for tag in event.tags.clone() {
            match tag {
                Tag::Path {
                  path,
                  file_type: _
                } => {
                  let glob_match = self.filter.match_path(path.as_path(), path.is_dir());
                  is_watched  = glob_match.is_whitelist();
                }
                _ => {}
            }
        }
        Ok(is_watched)
    }
}
