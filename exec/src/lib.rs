// Rules
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_must_use)]
#[allow(dead_code)]
// External Imports
use log::warn;
use std::env;
use std::path::Path;
use std::rc::Rc;
use std::thread::Builder;
use tokio;
use types::StrOutput;
// Internal Imports
mod display;
mod from;
pub mod subprocess;
pub mod types;

// Export statuable trait definitions
mod traits;
pub use traits::statuable;

// Error Handling
use miette::{Diagnostic, Error, IntoDiagnostic, NamedSource, Report, Result, SourceSpan};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq)]
pub struct Command {
    shell: String,
    attached: bool,
    pid: Option<u32>,
    output:

}
impl Default for Command {
    fn default() -> Self {
        return Self {
            shell: "sh".to_owned(),
            attached: true,
        };
    }
}
impl Command {
    pub fn new() -> Self {
        return Command::default();
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
    pub fn simple(&mut self, command: &str) -> Result<StrOutput> {
        let output = subprocess::simple(&self.shell(), command)?;
        Ok(output)
    }
    pub fn detached(&mut self, command: &str) -> Result<()> {
        subprocess::detached(&self.shell, command)?;
        Ok(())
    }
}
