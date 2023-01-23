// Rules
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_must_use)]
#[allow(dead_code)]
// External Imports
use log::warn;
use std::env;
use std::error::Error;
use types::StrOutput;
// Internal Imports
pub mod subprocess;
pub mod types;

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
    fn shell(&mut self) -> String {
        let shell_result = env::var("SHELL");
        match shell_result {
            Ok(res) => {
                self.shell = res;
                return self.shell.clone();
            }
            Err(_) => {
                return self.shell.clone();
            }
        };
    }
    /// Use for pipeline exewcution only
    pub fn simple(&mut self, command: &str) -> Result<StrOutput, Box<dyn Error>> {
        let output = subprocess::simple(&self.shell(), command)?;
        Ok(output)
    }
    pub fn detached(&mut self, command: &str) -> Result<(), Box<dyn Error>> {
        subprocess::detached(&self.shell, command)?;
        Ok(())
    }
}
