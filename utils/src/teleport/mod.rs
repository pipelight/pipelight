// Standard libs
use std::env;

mod types;
pub use self::types::{FileType, Internal, NaiveFileInfo, Teleport};
mod from;
// Tests
mod test;

// Enum workaround
use std::string::ToString;
use strum::{EnumIter, IntoEnumIterator};

use super::git::Git;
use std::path::Path;
// Error Handling
use log::{debug, error, trace, warn};
use miette::{miette, Diagnostic, Error, IntoDiagnostic, NamedSource, Report, Result, SourceSpan};
use std::process::exit;
use thiserror::Error;

impl Teleport {
    /// Jump between PWD and the directory of the loaded config file.
    pub fn teleport(&mut self) -> Self {
        let cwd = env::current_dir().unwrap().display().to_string();
        if cwd == self.origin {
            env::set_current_dir(self.internal.clone().directory_path.unwrap()).unwrap();
        }
        if cwd == self.internal.clone().directory_path.unwrap() {
            env::set_current_dir(self.origin.clone()).unwrap();
        }
        self.to_owned()
    }
    pub fn preffix(&mut self, string: &str) -> Result<Self> {
        self.file_info.preffix = string.to_owned();
        Ok(self.to_owned())
    }
    pub fn parent(&mut self) -> Result<Self> {
        if !self.has_reached_root()? {
            let parent = Path::new(&self.current).parent();
            if let Some(parent) = parent {
                self.current = parent.display().to_string();
            } else {
                return Err(Error::msg("File has no parent"));
            }
        }
        Ok(self.to_owned())
    }
    pub fn has_reached_root(&mut self) -> Result<bool> {
        // If teleport (search method) has reached git repo root
        if Git::new().exists() {
            return Ok(self.current
                == Git::new()
                    .repo
                    .unwrap()
                    .workdir()
                    .unwrap()
                    .to_str()
                    .unwrap());
        }
        // Else if teleport (search method) has reached filesystem root
        else {
            return Ok(self.current == "/");
        }
    }
    pub fn file(&mut self, path: &str) -> Result<Self> {
        // Test filename validity
        let err = Error::msg("The provided file name is not a valid one");
        Path::new(path)
            .file_name()
            .ok_or(err)?
            .to_str()
            .unwrap()
            .to_owned();
        self.file_info.path = Some(path.to_owned());
        Ok(self.to_owned())
    }
    pub fn search_file(&mut self) -> Result<()> {
        // Guard
        if self.file_info.path.is_some() {
            let name = self.file_info.path.clone().unwrap();

            let file_str = format!("{}/{}", self.current, name);
            let path = Path::new(&file_str);
            if path.exists() {
                self.internal.file_path = Some(path.display().to_string());
                self.internal.directory_path = Some(path.parent().unwrap().display().to_string());
            } else {
                if self.parent().is_ok() {
                    self.search_file()?;
                } else {
                    return Err(Error::msg("Couldn't find file"));
                }
            }
        }
        Ok(())
    }
    pub fn search_path(&mut self) -> Result<()> {
        let path_str = self.file_info.path.clone();
        if path_str.is_some() {
            let mut path_str = path_str.unwrap();
            let mut path = Path::new(&path_str);
            if path.is_relative() {
                path_str = path.canonicalize().into_diagnostic()?.display().to_string();
                path = Path::new(&path_str);
            }
            if path.exists() {
                self.internal.file_path = Some(path.display().to_string());
                self.internal.directory_path = Some(path.parent().unwrap().display().to_string());
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
    pub fn search_preffix(&mut self) -> Result<()> {
        let mut exists = false;
        // Loop through file types
        for file_type in FileType::iter() {
            let extension = String::from(&file_type);
            let file_str =
                format!("{}/{}.{}", self.current, self.file_info.preffix, extension).to_owned();
            let path = Path::new(&file_str);
            if path.exists() {
                exists = true;
                self.internal.file_path = Some(path.display().to_string());
                self.internal.directory_path = Some(path.parent().unwrap().display().to_string());
                break;
            }
        }
        if !exists {
            if self.parent().is_ok() {
                self.search_preffix()?;
            } else {
                return Err(Error::msg("Couldn't find the a file"));
            }
        }
        Ok(())
    }
    /// Recursively search a file throught parent dir and return path if exists
    pub fn search(&mut self) -> Result<()> {
        // If a file path provided
        let naive_path = self.file_info.path.clone();
        if naive_path.is_some() {
            let binding = &naive_path.clone().unwrap();
            let path = Path::new(binding);
            // If file path is only a file name
            let name = Path::new(path).file_name();
            if let Some(name) = name {
                if name.to_str().unwrap() == naive_path.unwrap() {
                    return Ok(self.search_file()?);
                } else {
                    return Ok(self.search_path()?);
                }
            }
        } else {
            return Ok(self.search_preffix()?);
        }
        Ok(())
    }
}
