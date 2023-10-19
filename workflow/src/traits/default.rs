// Structs
use crate::types::{Command, Event, Logs, Mode, Node, Parallel, Pipeline, Step, StepOrParallel};
use crate::types::{Trigger, TriggerBranch, TriggerTag};
use exec::Process;
use log::LevelFilter;
use utils::git::Git;
use utils::git::{Flag, Special};
use uuid::Uuid;
// Date and time
use chrono::Local;
// Unix process structs
use rustix::process::{getpgid, getpid, getsid, Pid};

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
impl Node {
    pub fn new() -> Node {
        Self::default()
    }
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
            steps,
            fallback: None,
        }
    }
}
impl Pipeline {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for StepOrParallel {
    fn default() -> Self {
        let step = Step::default();
        StepOrParallel::Step(step)
    }
}
impl StepOrParallel {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for Parallel {
    fn default() -> Self {
        Parallel {
            status: None,
            duration: None,
            steps: vec![Step::default()],
            mode: Some(Mode::StopOnFailure),
            fallback: None,
        }
    }
}
impl Parallel {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Trigger {
    pub fn new() -> Self {
        Self::default()
    }
}
impl Default for Trigger {
    fn default() -> Self {
        Trigger::TriggerBranch(TriggerBranch::default())
    }
}
impl Default for TriggerBranch {
    fn default() -> Self {
        TriggerBranch {
            action: Some(Flag::Special(Special::Manual)),
            branch: None,
        }
    }
}
impl Default for TriggerTag {
    fn default() -> Self {
        TriggerTag {
            action: Some(Flag::Special(Special::Manual)),
            tag: None,
        }
    }
}

impl Default for Step {
    fn default() -> Self {
        let commands = vec![Command::default()];
        Step {
            name: "default".to_owned(),
            status: None,
            duration: None,
            commands,
            mode: Some(Mode::StopOnFailure),
            fallback: None,
        }
    }
}
impl Step {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Command {
    pub fn new(stdin: &str) -> Command {
        Command {
            process: Process::new(stdin),
            ..Command::default()
        }
    }
}

impl Logs {
    pub fn new() -> Self {
        Self::default()
    }
}

impl Default for Event {
    fn default() -> Self {
        // Get process info
        let pid = getpid();
        let pgid = getpgid(Some(pid)).unwrap();
        let sid = getsid(Some(pid)).unwrap();

        // Get repo info if any
        let git = Git::new();
        let commit = git.get_commit().ok();

        Event {
            trigger: Trigger::get().unwrap(),
            commit,
            // Local instead of UTC to better stick to
            // most time lib iso8601
            date: Local::now().to_string(),
            pid: Some(Pid::as_raw(Some(pid))),
            pgid: Some(Pid::as_raw(Some(pgid))),
            sid: Some(Pid::as_raw(Some(sid))),
        }
    }
}
impl Event {
    pub fn new() -> Event {
        Self::default()
    }
}
