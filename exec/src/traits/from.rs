// Convert subprocess and process crate output to pipelighyt internal type

use crate::types::{State, Status};
// Casing
use convert_case::{Case, Casing};
// Logger
use log::{error, warn};

use std::convert::From;

// Process Types
use std::process::exit;
pub use std::process::Output;
pub use subprocess::CaptureData;

impl From<&Output> for State {
    fn from(s: &Output) -> Self {
        let mut stdout = None;
        let mut stderr = None;
        let stdout_str = String::from_utf8(s.clone().stdout)
            .unwrap()
            // .strip_suffix("\r\n")
            // .unwrap()
            .to_owned();
        let stderr_str = String::from_utf8(s.clone().stderr)
            .unwrap()
            // .strip_suffix("\r\n")
            // .unwrap()
            .to_owned();

        if !stdout_str.is_empty() {
            stdout = Some(stdout_str);
        }
        if !stderr_str.is_empty() {
            stderr = Some(stderr_str);
        }
        // println!("{:?}", s.status.code());

        let status = match s.status.success() {
            true => Status::Succeeded,
            false => Status::Failed,
        };
        return State {
            status: Some(status),
            stdin: None,
            stdout: stdout,
            stderr: stderr,
        };
    }
}
impl From<&CaptureData> for State {
    fn from(s: &CaptureData) -> Self {
        let mut stdout = None;
        let mut stderr = None;
        let stdout_str = String::from_utf8(s.stdout.clone())
            .unwrap()
            // .strip_suffix("\r\n")
            // .unwrap()
            .to_owned();
        let stderr_str = String::from_utf8(s.stderr.clone())
            .unwrap()
            // .strip_suffix("\r\n")
            // .unwrap()
            .to_owned();

        if !stdout_str.is_empty() {
            stdout = Some(stdout_str);
        }
        if !stderr_str.is_empty() {
            stderr = Some(stderr_str);
        }
        // println!("{:?}", s.status.code());

        let status = match s.exit_status.success() {
            true => Status::Succeeded,
            false => Status::Failed,
        };
        return State {
            status: Some(status),
            stdin: None,
            stdout: stdout,
            stderr: stderr,
        };
    }
}
impl From<&String> for Status {
    fn from(status: &String) -> Status {
        let cased: &str = &status.to_case(Case::Snake);
        match cased {
            "started" => return Status::Started,
            "succeeded" => return Status::Succeeded,
            "failed" => return Status::Failed,
            "running" => return Status::Running,
            "aborted" => return Status::Aborted,
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
            Status::Started => return "started".to_owned(),
            Status::Succeeded => return "succeeded".to_owned(),
            Status::Failed => return "failed".to_owned(),
            Status::Running => return "running".to_owned(),
            Status::Aborted => return "aborted".to_owned(),
        };
    }
}
