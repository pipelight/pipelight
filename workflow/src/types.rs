// Standard libs
use log::LevelFilter;
pub use pipelight_exec::dates::Duration;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

// Structs
use pipelight_exec::Process;
pub use pipelight_exec::Status;
use pipelight_utils::git::{Flag, Special};

// Event - Process
use chrono::Local;
use rustix::process::{getpgid, getpid, getsid, Pid};

// Traits - Enum workaround
use strum::EnumIter;

/**
* Options to tweak global pipelines behavior
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
impl Default for Node {
    fn default() -> Self {
        Node {
            value: None,
            status: None,
            duration: None,
            children: None,
            level: LevelFilter::Error,
        }
    }
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
impl Default for Pipeline {
    fn default() -> Self {
        let steps = vec![StepOrParallel::Step(Step::default())];
        Pipeline {
            uuid: Uuid::new_v4(),
            name: "default".to_owned(),
            event: None,
            status: None,
            duration: None,
            triggers: None,
            options: None,
            steps,
            fallback: None,
        }
    }
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
impl Default for StepOrParallel {
    fn default() -> Self {
        let step = Step::default();
        StepOrParallel::Step(step)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct Parallel {
    pub status: Option<Status>,
    pub duration: Option<Duration>,
    pub steps: Vec<Step>,
    // Fallback Hooks
    pub fallback: Option<Fallback>,
}
impl Default for Parallel {
    fn default() -> Self {
        Parallel {
            status: None,
            duration: None,
            steps: vec![Step::default()],
            fallback: None,
        }
    }
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
impl Default for Step {
    fn default() -> Self {
        let commands = vec![Command::default()];
        Step {
            name: "default".to_owned(),
            status: None,
            duration: None,
            commands,
            options: None,
            fallback: None,
        }
    }
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
impl Command {
    pub fn new(stdin: &str) -> Command {
        Command {
            process: Process::new().stdin(stdin).to_owned(),
            ..Command::default()
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, Ord, PartialEq, PartialOrd)]
#[serde(tag = "type")]
pub enum Trigger {
    TriggerBranch(TriggerBranch),
    TriggerTag(TriggerTag),
}
impl Default for Trigger {
    fn default() -> Self {
        Trigger::TriggerBranch(TriggerBranch::default())
    }
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
impl Default for TriggerBranch {
    fn default() -> Self {
        TriggerBranch {
            action: Some(Flag::Special(Special::Manual)),
            branch: None,
            commit: None,
        }
    }
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
impl Default for TriggerTag {
    fn default() -> Self {
        TriggerTag {
            action: Some(Flag::Special(Special::Manual)),
            tag: None,
            commit: None,
        }
    }
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
impl Default for Event {
    fn default() -> Self {
        // Get process info
        let pid = getpid();
        let pgid = getpgid(Some(pid)).unwrap();
        let sid = getsid(Some(pid)).unwrap();

        Event {
            trigger: Trigger::get().unwrap(),
            // Local instead of UTC to better stick to
            // most time lib iso8601
            date: Local::now().to_string(),
            pid: Some(Pid::as_raw(Some(pid))),
            pgid: Some(Pid::as_raw(Some(pgid))),
            sid: Some(Pid::as_raw(Some(sid))),
        }
    }
}

/**
A struct that contains convenience Logs methods
*/
#[derive(Default, Debug, Serialize, Deserialize, Clone)]
pub struct Logs {
    pub pipelines: Option<Vec<Pipeline>>,
}
