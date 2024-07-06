pub mod statuable;

use serde::{Deserialize, Serialize};
// Colors and Formatting
use colored::Colorize;
use std::fmt;

// Unix process output
use std::process::Output;
// Casing
use convert_case::{Case, Casing};
// Logger
use log::warn;
use utils::dates::Duration;

/**
* A convenience struct to store the process status and its duration.
*/
#[derive(Default, Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct State {
    pub duration: Option<Duration>,
    pub status: Option<Status>,
}

/**
* An enum for the different possible process state.
*/
#[derive(Default, Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[serde(rename_all = "kebab-case")]
pub enum Status {
    #[default]
    // The process has been started.
    Started,
    // The process has run successfuly (returns a unix status of 0).
    Succeeded,
    // The process has failed or has been gently stopped (returns a unix status of 1).
    Failed,
    // The process is running (is in the os running process list).
    Running,
    // The process has been abruptly halted.
    Aborted,
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
                warn!("unexpected string, assuminng default state");
                Status::default()
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

impl From<&Output> for Status {
    fn from(output: &Output) -> Status {
        match output.status.success() {
            true => Status::Succeeded,
            false => Status::Failed,
        }
    }
}
/**
* Displays a nice and colorful string for easy status checks.
*/
impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let icon = "●";
        match *self {
            Status::Started => write!(f, "{} Started", icon),
            Status::Succeeded => write!(f, "{} {}", icon.blue(), "Succeeded".bold()),
            Status::Failed => write!(f, "{} {}", icon.red(), "Failed".normal().bold()),
            Status::Running => write!(f, "{} {}", icon.green(), "Running".bold()),
            Status::Aborted => write!(f, "{} {}", icon.yellow(), "Aborted".bold()),
        };
        Ok(())
    }
}
