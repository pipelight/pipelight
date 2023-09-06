// Standard libs
use std::env;

// File type usage
use crate::files::FileType;
use strum::IntoEnumIterator;

mod types;
pub use self::types::{Gate, Portal};
// Tests
mod test;

// Enum workaround
use std::string::ToString;

use super::git::Git;
use std::path::Path;

// Error Handling
use miette::{Error, IntoDiagnostic, Result};

impl Portal {
    /// Jump between PWD and the directory of the loaded config file.
    pub fn teleport(&mut self) -> Result<Self> {
        env::set_current_dir(self.current.directory_path.clone().unwrap()).into_diagnostic()?;
        Ok(self.to_owned())
    }
    pub fn origin(&mut self) -> Result<Self> {
        env::set_current_dir(self.origin.directory_path.clone().unwrap()).into_diagnostic()?;
        Ok(self.to_owned())
    }
    // Set seed string, file name, relative path, absolute path
    pub fn seed(&mut self, string: &str) -> Self {
        self.seed = Some(string.to_owned());
        self.to_owned()
    }
    /// Recursively search a file throught parent
    pub fn search(&mut self) -> Result<Self> {
        let seed = self.seed.clone();
        if let Some(seed) = seed {
            let path = Path::new(&seed);
            if self.search_path().is_ok() {
                return Ok(self.to_owned());
            } else if self.search_file().is_ok() {
                return Ok(self.to_owned());
            } else if self.search_prefix().is_ok() {
                return Ok(self.to_owned());
            } else {
                return Err(Error::msg(format!(
                    "Couldn't find a file with the provided seed: {}",
                    seed
                )));
            }
        }
        Ok(self.to_owned())
    }
    fn parent(&mut self) -> Result<Self> {
        if !self.has_reached_root()? {
            if self.current.directory_path.is_some() {
                let current = self.current.directory_path.clone().unwrap();
                let parent = Path::new(&current).parent();
                if let Some(parent) = parent {
                    self.current.directory_path = Some(parent.display().to_string());
                } else {
                    return Err(Error::msg("File has no parent"));
                }
            }
        }
        Ok(self.to_owned())
    }
    fn has_reached_root(&mut self) -> Result<bool> {
        // If teleport (search method) has reached git repo root
        if Git::new().exists() {
            let boolean = self.current.directory_path
                == Some(
                    Git::new()
                        .repo
                        .unwrap()
                        .workdir()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_owned(),
                );
            Ok(boolean)
        }
        // Else if teleport (search method) has reached filesystem root
        else {
            Ok(self.current.directory_path == Some("/".to_owned()))
        }
    }
    fn search_file(&mut self) -> Result<()> {
        // SafeGuard
        if self.seed.is_some() {
            let name = self.seed.clone().unwrap();
            let file_str = format!("{}/{}", self.current.directory_path.clone().unwrap(), name);
            let path = Path::new(&file_str);
            // SafeGuard
            if path.extension().is_none() {
                return Err(Error::msg("Couldn't find file"));
            }

            if path.exists() {
                self.target.file(path.display().to_string());
            } else if self.parent().is_ok() {
                self.search_file()?;
            } else {
                return Err(Error::msg("Couldn't find file"));
            }
        }
        Ok(())
    }
    fn search_path(&mut self) -> Result<()> {
        let path_str = self.seed.clone();
        if let Some(..) = path_str {
            let mut path_str = path_str.unwrap();
            let mut path = Path::new(&path_str);
            if path.is_relative() {
                path_str = path.canonicalize().into_diagnostic()?.display().to_string();
                path = Path::new(&path_str);
            }
            // SafeGuard
            if path.extension().is_none() {
                return Err(Error::msg("Couldn't find file"));
            }
            if path.exists() {
                self.target.file(path.display().to_string());
                Ok(())
            } else {
                Err(Error::msg(format!(
                    "Couldn't find file at path {}",
                    path_str
                )))
            }
        } else {
            Err(Error::msg("No path was provided"))
        }
    }
    fn search_prefix(&mut self) -> Result<()> {
        let mut exists = false;
        // Loop through file types
        for file_type in FileType::iter() {
            let extension = String::from(&file_type);
            if self.seed.is_some() {
                let path_str = format!(
                    "{}/{}.{}",
                    self.current.directory_path.clone().unwrap(),
                    self.seed.clone().unwrap(),
                    extension
                )
                .to_owned();
                let path = Path::new(&path_str);
                if path.exists() {
                    exists = true;
                    self.target.file(path.display().to_string());
                    break;
                }
            }
        }
        if !exists {
            if self.parent().is_ok() {
                self.search_prefix()?;
            } else {
                return Err(Error::msg("Couldn't find file"));
            }
        }
        Ok(())
    }
}
