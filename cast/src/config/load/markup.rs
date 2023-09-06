// Error Handling
use crate::error::{TomlError, YamlError};
use miette::{IntoDiagnostic, Result};

// Read file
use std::fs;

use crate::Config;

impl Config {
    /// Return a Config struct from a provided toml file path
    pub fn tml(file_path: &str) -> Result<Config> {
        let tml = fs::read_to_string(file_path).into_diagnostic()?;
        let res = toml::from_str::<Config>(&tml);
        match res {
            Ok(res) => Ok(res),
            Err(e) => {
                let err = TomlError::new(e, &tml);
                Err(err.into())
            }
        }
    }
    /// Return a Config struct from a provided yaml file path
    pub fn yml(file_path: &str) -> Result<Config> {
        let yml = fs::read_to_string(file_path).into_diagnostic()?;
        let res = serde_yaml::from_str::<Config>(&yml);
        match res {
            Ok(res) => Ok(res),
            Err(e) => {
                let err = YamlError::new(e, &yml);
                Err(err.into())
            }
        }
    }
}
