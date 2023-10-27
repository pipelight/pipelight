// Environment
use std::env;
// File
use crate::files::FileType;
// Struct
use crate::teleport::types::Portal;
// Trait
use strum::IntoEnumIterator;
// Git
use crate::git::Git;
// Filesystem
use std::path::Path;
// Error Handling
use log::{info, trace};
use miette::{Error, IntoDiagnostic, Result};

impl Portal {
    /// Jump between PWD and the directory of the loaded config file.
    pub fn teleport(&mut self) -> Result<Self> {
        let target = self.target.directory_path.clone().unwrap();
        env::set_current_dir(target.clone()).into_diagnostic()?;
        info!("working directory changed to -> {}", &target);
        Ok(self.to_owned())
    }
    pub fn origin(&mut self) -> Result<Self> {
        let target = self.origin.directory_path.clone().unwrap();
        env::set_current_dir(target.clone()).into_diagnostic()?;
        info!("working directory changed to -> {}", &target);
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
            // Sub portals
            let mut path_portal = self.clone();
            let mut file_portal = self.clone();
            let mut prefix_portal = self.clone();

            if path_portal.search_path().is_ok() {
                *self = path_portal;
                return Ok(self.to_owned());
            } else if file_portal.search_file().is_ok() {
                *self = file_portal;
                return Ok(self.to_owned());
            } else if prefix_portal.search_prefix().is_ok() {
                *self = prefix_portal;
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
        if !self.has_reached_root()? && self.current.directory_path.is_some() {
            let current = self.current.directory_path.clone().unwrap();
            let parent = Path::new(&current).parent();
            if parent.is_some() {
                self.current.directory_path = Some(parent.unwrap().display().to_string());
                return Ok(self.to_owned());
            }
        }
        Err(Error::msg("File has no parent"))
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
    pub fn search_file(&mut self) -> Result<()> {
        trace!("search file");
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
                self.target.file(path.display().to_string())?;
            } else if self.parent().is_ok() {
                self.search_file()?;
            } else {
                return Err(Error::msg("Couldn't find file"));
            }
        }
        Ok(())
    }
    pub fn search_path(&mut self) -> Result<()> {
        trace!("search path");
        let path_str = self.seed.clone();
        if let Some(mut path_str) = path_str {
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
                self.target.file(path.display().to_string())?;
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
    pub fn search_prefix(&mut self) -> Result<()> {
        trace!("search prefix");
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
                    self.target.file(path.display().to_string())?;
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
