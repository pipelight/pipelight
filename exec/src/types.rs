use serde::{Deserialize, Serialize};
use std::clone::Clone;
use std::cmp::PartialEq;
use std::process::Output;

// #[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[derive(Debug, Clone, PartialEq)]
pub struct StrOutput {
    pub status: bool,
    pub stdout: Option<String>,
    pub stderr: Option<String>,
}
impl From<&Output> for StrOutput {
    fn from(s: &Output) -> Self {
        let stdout = String::from_utf8(s.clone().stdout).unwrap().to_owned();
        let stderr = String::from_utf8(s.clone().stderr).unwrap().to_owned();
        return StrOutput {
            status: s.status.success(),
            stdout: Some(stdout),
            stderr: Some(stderr),
        };
    }
}
