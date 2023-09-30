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
// Error Handling
use miette::{Error, IntoDiagnostic, Result};

impl Ignore {
    pub fn new(path: &str) -> Result<Self> {
        let path = PathBuf::from(path);
        let origin = env::current_dir().unwrap();

        let files = [IgnoreFile {
            path,
            applies_in: Some(origin.clone()),
            applies_to: None,
        }];
        let filter;
        async {
          filter = IgnoreFilter::new(origin, &files).await.into_diagnostic();
        };
        let e = Ignore { filter: filter?  };
        Ok(e)
    }
}
impl Filterer for Ignore {
    fn check_event(&self, event: &Event, priority: Priority) -> Result<bool, RuntimeError> {
        for tag in event.tags {
            match tag {
                Tag::Path {
                  path,
                  file_type
                } => {
                  self.filter.match_path(path, path.is_dir());
                  Ok(true)
                }
                _ => Ok(false)
            }
        }
        Ok(true)
    }
}
