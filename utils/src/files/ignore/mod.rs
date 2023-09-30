// Ignore file
use ignore_files::{IgnoreFile, IgnoreFilter};
use project_origins::{origins, types, ProjectType};
// Watchexec filter
use watchexec::{error::RuntimeError, filter::Filterer};
use watchexec_events::{Event, Priority, Tag};
// Filesystem manipulatoion
use std::env;
use std::path::{Path, PathBuf};
// Error Handling
use miette::{Error, Result};

/**

*/
pub struct Ignore;

impl Ignore {
    pub fn get(path: &str) -> Result<()> {
        let path = PathBuf::from(path);
        let origin = env::current_dir().unwrap();

        let files = [IgnoreFile {
            path,
            applies_in: Some(origin.clone()),
            applies_to: None,
        }];

        let filter = IgnoreFilter::new(origin, &files);

        Ok(())
    }
}
impl Filterer for Ignore {
    fn check_event(&self, event: &Event, priority: Priority) -> Result<bool, RuntimeError> {
        match event.tags.contains(Path) {}
    }
}
