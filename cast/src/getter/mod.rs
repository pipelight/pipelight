use super::Config;

// Error Handling
use crate::error::{JsonError, TomlError, YamlError};
use miette::{Error, Result};

// Exec
use exec::Process;

// Standard lib
// use std::env::current_dir;
use std::path::Path;

use super::typescript::main_script;
use utils::teleport::{FileType, Teleport};

//Loaders
mod load;

impl Config {
    /// Browse through the filesystem to find the config file
    /// Immediately set the cwd to the config file location if found
    pub fn get(file: Option<String>, args: Option<Vec<String>>) -> Result<Config> {
        // let pwd: String = current_dir().unwrap().display().to_string();
        let mut teleport = Teleport::new();
        if let Some(file) = file {
            teleport.file(&file)?;
            teleport.search()?;
        } else {
            // Search default file
            teleport.preffix("pipelight")?;
            teleport.search()?;
        }
        let res = Config::load_from_file(&teleport.internal.file_path.clone().unwrap(), args);
        match res {
            Ok(res) => {
                teleport.teleport();
                Ok(res)
            }
            Err(e) => Err(e.wrap_err("Error in configuration file")),
        }
    }
    /// Browse through the filesystem to find the config file
    /// Immediately set the cwd to the config file location if found
    pub fn get_with_teleport(
        file: Option<String>,
        args: Option<Vec<String>>,
    ) -> Result<(Config, Teleport)> {
        // let pwd: String = current_dir().unwrap().display().to_string();
        let mut teleport = Teleport::new();
        if let Some(file) = file {
            teleport.file(&file)?;
            teleport.search()?;
        } else {
            // Search default file
            teleport.preffix("pipelight")?;
            teleport.search()?;
        }
        let res = Config::load_from_file(&teleport.internal.file_path.clone().unwrap(), args);
        match res {
            Ok(res) => {
                teleport.teleport();
                Ok((res, teleport))
            }
            Err(e) => Err(e.wrap_err("Error in configuration file")),
        }
    }
}
