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
            "started" => Status::Started,
            "succeeded" => Status::Succeeded,
            "failed" => Status::Failed,
            "running" => Status::Running,
            "aborted" => Status::Aborted,
            _ => {
                let message = format!("The pipeline status {} is not known", cased);
                error!("{}", message);
                exit(1);
            }
        }
    }
}
impl From<&Status> for String {
    fn from(action: &Status) -> String {
        match action {
            Status::Started => "started".to_owned(),
            Status::Succeeded => "succeeded".to_owned(),
            Status::Failed => "failed".to_owned(),
            Status::Running => "running".to_owned(),
            Status::Aborted => "aborted".to_owned(),
        }
    }
}
