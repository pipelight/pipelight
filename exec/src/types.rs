use serde::{Deserialize, Serialize};
use std::clone::Clone;
use std::cmp::PartialEq;
pub use std::process::Output;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct StrOutput {
    pub status: bool,
    pub stdout: Option<String>,
    pub stderr: Option<String>,
}
impl From<&Output> for StrOutput {
    fn from(s: &Output) -> Self {
        let mut stdout = None;
        let mut stderr = None;
        let stdout_str = String::from_utf8(s.clone().stdout).unwrap().to_owned();
        let stderr_str = String::from_utf8(s.clone().stderr).unwrap().to_owned();

        if !stdout_str.is_empty() {
            stdout = Some(stdout_str);
        }
        if !stderr_str.is_empty() {
            stderr = Some(stderr_str);
        }

        return StrOutput {
            status: s.status.success(),
            stdout: stdout,
            stderr: stderr,
        };
    }
}
