// Exec
use exec::Process;

// Error Handling
use crate::error::{TomlError, YamlError};
use crate::Config;
use miette::Result;

/// Return a Config struct from a provided toml file path
impl Config {
    pub fn tml(file_path: &str) -> Result<Config> {
        let executable = "cat";
        let command = format!("{} {}", executable, file_path);
        let p = Process::new(&command).simple()?;

        let tml = p.state.stdout.unwrap();
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
        let executable = "cat";
        let command = format!("{} {}", executable, file_path);
        let p = Process::new(&command).simple()?;

        let yml = p.state.stdout.unwrap();
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
