use crate::types::{
    Command, Event, Logs, Mode, Node, Parallel, Pipeline, Step, StepOrParallel, Trigger,
};
// External imports
use exec::Process;
use utils::git::{Git};
// Date and time
use chrono::Local;
//Logs

use log::LevelFilter;

use uuid::Uuid;

// Error Handling


// sys
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
        let commands = vec![Command::default()];
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
impl Default for Logs {
    fn default() -> Self {
        Logs
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
            trigger: Trigger::flag(None).unwrap(),
            commit: commit,
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
