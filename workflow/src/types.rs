// Standard libs
use log::LevelFilter;
use serde::{Deserialize, Serialize};
pub use utils::dates::Duration;
use uuid::Uuid;

// Structs
use exec::Process;
pub use exec::Status;
use utils::git::Flag;

// Traits - Enum workaround
use strum::EnumIter;

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct Fallback {
    pub on_started: Option<Vec<StepOrParallel>>,
    pub on_failure: Option<Vec<StepOrParallel>>,
    pub on_success: Option<Vec<StepOrParallel>>,
    pub on_abortion: Option<Vec<StepOrParallel>>,
}
#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub struct Config {
    pub pipelines: Option<Vec<Pipeline>>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Node {
    pub value: Option<String>,
    pub duration: Option<String>,
    pub status: Option<Status>,
    pub children: Option<Vec<Node>>,
    pub level: LevelFilter,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct Pipeline {
    pub uuid: Uuid,
    pub name: String,
    pub event: Option<Event>,
    pub status: Option<Status>,
    pub duration: Option<Duration>,
    pub triggers: Option<Vec<Trigger>>,
    pub fallback: Option<Fallback>,
    pub steps: Vec<StepOrParallel>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
#[serde(untagged)]
pub enum StepOrParallel {
    Step(Step),
    Parallel(Parallel),
}
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct Parallel {
    pub status: Option<Status>,
    pub duration: Option<Duration>,
    pub steps: Vec<Step>,
    // Failure Handling mode
    pub mode: Option<Mode>,
    // Fallback Hooks
    pub fallback: Option<Fallback>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct Step {
    pub name: String,
    pub status: Option<Status>,
    pub duration: Option<Duration>,
    pub commands: Vec<Command>,
    // Failure Handling mode
    pub mode: Option<Mode>,
    // Fallback Hooks
    pub fallback: Option<Fallback>,
}
#[derive(Debug, EnumIter, Serialize, Deserialize, Clone, PartialEq, PartialOrd, Eq, Ord)]
#[serde(untagged)]
pub enum Mode {
    StopOnFailure,
    JumpNextOnFailure,
    ContinueOnFailure,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct Command {
    // Pretty computatoin (Time, duration...)
    pub duration: Option<Duration>,
    // Things relevant to unix process
    pub process: Process,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Trigger {
    pub action: Option<Flag>,
    pub branch: Option<String>,
    pub tag: Option<String>,
}
/**
The event/environment that triggered the piepline execution.
*/
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct Event {
    pub trigger: Trigger,
    pub date: String,
    // The commit id
    pub commit: Option<String>,
    // Unix process info
    pub pid: Option<u32>,
    pub pgid: Option<u32>,
    pub sid: Option<u32>,
}

/**
A struct that contains convenience Logs methods
*/
#[derive(Debug, Serialize, Deserialize, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct Logs;
