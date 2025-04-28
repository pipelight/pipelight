use crate::{Config, Pipeline};

// Error Handling
use miette::{Error, Result};
use pipelight_error::{PipelightError, TomlError, YamlError};

impl Config {
    /*
     * Convert the configuration into a toml string.
     */
    pub fn to_toml() -> Result<(), PipelightError> {
        Ok(())
    }
}
