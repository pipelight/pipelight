// Types
use crate::types::{Io, Process, State, Status};
use std::process::Output;
// Casing
use convert_case::{Case, Casing};
// Logger
use log::{error, warn};

// Error Handling
use miette::{IntoDiagnostic, Result};

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
                warn!("unexpected string, assuminng default state");
                return Status::default();
            }
        }
    }
}
impl From<&Status> for String {
    fn from(status: &Status) -> String {
        match status {
            Status::Started => "started".to_owned(),
            Status::Succeeded => "succeeded".to_owned(),
            Status::Failed => "failed".to_owned(),
            Status::Running => "running".to_owned(),
            Status::Aborted => "aborted".to_owned(),
        }
    }
}

impl From<&Output> for State {
    fn from(output: &Output) -> State {
        let status = match output.status.success() {
            true => Some(Status::Succeeded),
            false => Some(Status::Failed),
        };
        State {
            status,
            duration: None,
        }
    }
}
