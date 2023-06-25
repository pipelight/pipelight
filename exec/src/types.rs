use super::display;
use super::from;
use serde::{Deserialize, Serialize};
use std::clone::Clone;
use std::cmp::PartialEq;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Status {
    Started,
    Succeeded,
    Failed,
    Running,
    Aborted,
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct StrOutput {
    pub status: Option<Status>,
    pub stdin: Option<String>,
    pub stdout: Option<String>,
    pub stderr: Option<String>,
}
impl Default for StrOutput {
    fn default() -> Self {
        StrOutput {
            status: None,
            stdin: None,
            stdout: None,
            stderr: None,
        }
    }
}
impl StrOutput {
    pub fn new(stdin: &str) -> StrOutput {
        stdin: Some(stdin),
        ..Self::default()
    }
}

pub trait Statuable {
    fn get_status(&self) -> Option<Status>;
    fn set_status(&mut self, status: Option<Status>);
}
impl Statuable for StrOutput {
    fn get_status(&self) -> Option<Status> {
        return self.status.clone();
    }
    fn set_status(&mut self, status: Option<Status>) {
        self.status = status;
    }
}
