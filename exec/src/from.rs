use crate::types::Status::*;
use crate::types::{Status, StrOutput};
use convert_case::{Case, Casing};
use log::{error, warn};
use std::convert::From;
use std::process::exit;
pub use std::process::Output;

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

        let status = match s.status.success() {
            true => Status::Succeeded,
            false => Status::Failed,
        };
        return StrOutput {
            status: Some(status),
            stdout: stdout,
            stderr: stderr,
        };
    }
}
impl From<&String> for Status {
    fn from(status: &String) -> Status {
        let cased: &str = &status.to_case(Case::Snake);
        match cased {
            "started" => return Started,
            "succeeded" => return Succeeded,
            "failed" => return Failed,
            "running" => return Running,
            "aborted" => return Aborted,
            _ => {
                let message = format!("The pipeline status {} is not known", cased);
                error!("{}", message);
                exit(1);
            }
        };
    }
}
impl From<&Status> for String {
    fn from(action: &Status) -> String {
        match action {
            Started => return "started".to_owned(),
            Succeeded => return "succeeded".to_owned(),
            Failed => return "failed".to_owned(),
            Running => return "running".to_owned(),
            Aborted => return "aborted".to_owned(),
        };
    }
}
