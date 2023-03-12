// Standard libs
use std::env;

use super::git::Git;
use std::path::Path;
// Error Handling
use miette::{miette, Diagnostic, Error, IntoDiagnostic, NamedSource, Report, Result, SourceSpan};
use thiserror::Error;

#[derive(Debug, Clone)]
pub struct Teleport {
    pub root_path: Option<String>,
    pub current_path: Option<String>,
}

impl Default for Teleport {
    fn default() -> Self {
        let cwd = env::current_dir().unwrap().display().to_string();

        let root: Option<String>;
        let res = Teleport::search("pipelight.config.ts", &cwd);
        if res.is_ok() {
            root = Some(res.unwrap());
        } else {
            root = None;
        }
        return Teleport {
            root_path: root,
            current_path: None,
        };
    }
}

impl Teleport {
    pub fn new() -> Self {
        Teleport::default()
    }
    pub fn teleport(&mut self) {
        let cwd = env::current_dir().unwrap().display().to_string();
        let root = self.clone().root_path.unwrap();
        let current = self.clone().current_path.unwrap();
        if cwd != root || cwd != root {
            return;
        }
        if cwd == root {
            env::set_current_dir(&current).unwrap();
        }
        if cwd == current {
            env::set_current_dir(&root).unwrap();
        }
    }
    /// Recursively search a file throught parent dir
    pub fn search(file_name: &str, dir_str: &str) -> Result<String> {
        // Convert args to path
        let path_str = format!("{}/{}", dir_str, file_name);
        let path = Path::new(&path_str);
        let dir = Path::new(&dir_str);
        let exists = Path::new(path).exists();

        // Config try get
        if exists {
            Ok(path.display().to_string())
            // Load config from str -> Path
        } else {
            let message = "Couldn't find a configuration file";
            // Reached git repo root
            if Git::new().exists() {
                if dir_str
                    == Git::new()
                        .repo
                        .unwrap()
                        .workdir()
                        .unwrap()
                        .display()
                        .to_string()
                {
                    return Err(Error::msg(message));
                }
            }
            let parent = dir.parent();
            if parent.is_some() {
                let new_path = Teleport::search(file_name, &parent.unwrap().display().to_string())?;
                Ok(new_path)
            } else {
                // No more accessible parents
                Err(Error::msg(message))
            }
        }
    }
}
