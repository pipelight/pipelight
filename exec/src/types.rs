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
    pub status: Status,
    pub stdout: Option<String>,
    pub stderr: Option<String>,
}
impl Default for StrOutput {
    fn default() -> Self {
        StrOutput {
            status: Status::Started,
            stdout: None,
            stderr: None,
        }
    }
}
impl StrOutput {
    pub fn new() -> StrOutput {
        Self::default()
    }
}
