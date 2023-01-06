pub mod subprocess;
pub mod types;

use log::{debug, error, info, trace, warn, LevelFilter};
use std::env;
use std::error::Error;
use types::StrOutput;

#[derive(Debug, Clone, PartialEq)]
pub struct Exec {
    shell: String,
}
impl Exec {
    pub fn new() -> Self {
        return Self {
            shell: "sh".to_owned(),
        };
    }
    /// Return user session shell if possible
    fn shell(&mut self) -> Self {
        let default_shell = "sh".to_owned();
        let shell_result = env::var("SHELL");
        match shell_result {
            Ok(res) => {
                self.shell = res;
                return self.to_owned();
            }
            Err(e) => {
                return self.to_owned();
            }
        };
    }
    /// Use for pipeline exewcution only
    pub fn simple(&mut self, command: &str) -> Result<StrOutput, Box<dyn Error>> {
        let shell = &self.shell();
        let output = subprocess::simple(&self.shell, command)?;
        Ok(output)
    }
    pub fn attached(&mut self, command: &str) -> Result<String, Box<dyn Error>> {
        let output = subprocess::attached(&self.shell, command);
        match output {
            Ok(output) => {
                return Ok(output.to_owned());
            }
            Err(e) => {
                warn!("command: {}\n output: {}", command, e);
                return Err(Box::from(e));
            }
        };
    }
    pub fn detached(&mut self, command: &str) -> Result<(), Box<dyn Error>> {
        let shell = &self.shell();
        let output = subprocess::detached(&self.shell, command);
        Ok(())
    }
}
