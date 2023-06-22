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
// Error Handling
use miette::{Diagnostic, Error, IntoDiagnostic, NamedSource, Report, Result, SourceSpan};
use thiserror::Error;

#[derive(Debug, Clone, PartialEq)]
pub struct Exec {
    shell: String,
}
impl Default for Exec {
    fn default() -> Self {
        return Self {
            shell: "sh".to_owned(),
        };
    }
}
impl Exec {
    pub fn new() -> Self {
        return Exec::default();
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
    pub fn os(&mut self, command: &str) -> Result<StrOutput> {
        let output = subprocess::os(&self.shell(), command)?;
        Ok(output)
    }
    pub fn detached(&mut self, command: &str) -> Result<()> {
        subprocess::detached(&self.shell, command)?;
        Ok(())
    }
}
