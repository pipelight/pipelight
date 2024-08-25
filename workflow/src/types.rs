// Standard libs
use log::LevelFilter;
pub use pipelight_exec::dates::Duration;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// Structs
use pipelight_exec::Process;
pub use pipelight_exec::Status;
use pipelight_utils::git::Flag;

// Traits - Enum workaround
use strum::EnumIter;

/**
Options to tweak global pipelines behavior
*/
#[derive(Default, Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct ConfigOpts {
    // Wheteher the pipeline should be attached or detached from the standard I/O
    // when triggered by a git hook.
    pub attach: Option<bool>,
    pub log_level: Option<LevelFilter>,
}

#[derive(Default, Debug, Clone, Eq, PartialEq)]
pub struct Config {
    pub pipelines: Option<Vec<Pipeline>>,
    pub options: Option<ConfigOpts>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Node {
    pub value: Option<String>,
    pub duration: Option<String>,
    pub status: Option<Status>,
    pub children: Option<Vec<Node>>,
    pub level: LevelFilter,
}

/**
Options to tweak pipelines behavior
*/
#[derive(Default, Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct PipelineOpts {
    // Wheteher the pipeline should be attached or detached from the standard I/O
    // when triggered by a git hook.
    pub attach: Option<bool>,
    pub log_level: Option<LevelFilter>,
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
    pub options: Option<PipelineOpts>,
}

#[derive(Default, Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct StepOpts {
    // The step's command execution behavior
    // Failure Handling mode
    pub mode: Option<Mode>,
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
    pub options: Option<StepOpts>,
    // Fallback Hooks
    pub fallback: Option<Fallback>,
}
#[derive(Default, Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct Fallback {
    pub on_started: Option<Vec<StepOrParallel>>,
    pub on_failure: Option<Vec<StepOrParallel>>,
    pub on_success: Option<Vec<StepOrParallel>>,
    pub on_abortion: Option<Vec<StepOrParallel>>,
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

#[derive(Debug, Serialize, Deserialize, Clone, Eq, Ord, PartialEq, PartialOrd)]
#[serde(untagged)]
pub enum Trigger {
    TriggerBranch(TriggerBranch),
    TriggerTag(TriggerTag),
}
#[derive(Debug, Serialize, Deserialize, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct TriggerBranch {
    pub action: Option<Flag>,
    pub branch: Option<String>,
    // Storage value. Not used in any computation
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub commit: Option<String>,
}
#[derive(Debug, Serialize, Deserialize, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub struct TriggerTag {
    pub action: Option<Flag>,
    pub tag: Option<String>,
    // Storage value. Not used in any computation
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(default)]
    pub commit: Option<String>,
}
/**
The event/environment that triggered the piepline execution.
*/
#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct Event {
    pub trigger: Trigger,
    pub date: String,
    // Unix process info
    pub pid: Option<i32>,
    pub pgid: Option<i32>,
    pub sid: Option<i32>,
}

/**
A struct that contains convenience Logs methods
*/
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct Logs {
    pub pipelines: Option<Vec<Pipeline>>,
}
