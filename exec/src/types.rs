use serde::{Deserialize, Serialize};
use std::clone::Clone;
use std::cmp::PartialEq;
use std::os::unix::io::RawFd;
use utils::dates::Duration;
use uuid::Uuid;

/**
Simplified process status to abstract process raw status
*/
#[derive(Default, Debug, Serialize, Deserialize, Clone, Eq, PartialEq, Ord, PartialOrd)]
#[serde(rename_all = "kebab-case")]
pub enum Status {
    #[default]
    Started,
    Succeeded,
    Failed,
    Running,
    Aborted,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct Process {
    pub uuid: Option<Uuid>,
    pub pid: Option<u32>,
    pub state: State,
    pub io: Io,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct State {
    pub duration: Option<Duration>,
    pub status: Option<Status>,
}

/**
Process input/outputs
*/
#[derive(Default, Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct Io {
    pub uuid: Option<Uuid>,
    pub stdin: Option<String>,
    pub stdout: Option<String>,
    pub stderr: Option<String>,
}
