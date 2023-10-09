use serde::{Deserialize, Serialize};
use std::clone::Clone;
use std::cmp::PartialEq;
use utils::dates::Duration;
use uuid::Uuid;

/**
Simplified process status to abstract process raw unix status
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
pub struct SelfProcess;

/**
Simplified process struct.
*/
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct Process {
    pub uuid: Option<Uuid>,
    pub pid: Option<u32>,
    pub state: State,
    pub io: Io,
    // pub cwd: Option<String>,
}

/**
The process sate is defined by its status(running, succeedded...) and its duration.
*/
#[derive(Default, Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct State {
    pub duration: Option<Duration>,
    pub status: Option<Status>,
}

/**
Process self managed input/outputs struct.
*/
#[derive(Default, Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct Io {
    pub uuid: Option<Uuid>,
    pub stdin: Option<String>,
    pub stdout: Option<String>,
    pub stderr: Option<String>,
}
