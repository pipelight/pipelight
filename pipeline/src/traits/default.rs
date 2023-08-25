use crate::types::{
    Command, Config, Event, Logs, Mode, Node, Parallel, Pipeline, Step, StepOrParallel, Trigger,
};
// External imports
use utils::git::{Flag, Git};
use utils::teleport::Teleport;

use exec::Process;
// Date and time
use chrono::Local;
//Logs
use log::warn;
use log::LevelFilter;

use uuid::Uuid;

// Error Handling
use miette::Result;

// sys
use rustix::process::{getpgid, getpid, getsid, Pid};

// Global var
use once_cell::sync::Lazy;

// Global var
pub static mut CONFIG: Lazy<Config> = Lazy::new(Config::default);
pub static mut TELEPORT: Lazy<Teleport> = Lazy::new(Teleport::default);
pub static mut TRIGGER_ENV: Lazy<Trigger> = Lazy::new(Trigger::default);

impl Trigger {
    /// Return actual triggering env with a modified flag
    pub fn flag(flag: Option<Flag>) -> Result<Trigger> {
        let mut env;
        unsafe {
            env = (*TRIGGER_ENV).clone();
        }
        // Set git env
        if Git::new().exists() {
            env.branch = Git::new().get_branch()?;
            env.tag = Git::new().get_tag()?;
        }
        if flag.is_some() {
            env.action = flag;
        } else if env.action.is_none() {
            env.action = Some(Flag::default());
        }
        // Set the gloabl env
        unsafe {
            *TRIGGER_ENV = env.clone();
        }
        Ok(env)
    }
}

impl Config {
    /// Remove pipelines with the same name
    pub fn dedup_pipelines(&mut self) -> Self {
        if self.pipelines.is_some() {
            let init_length = &self.pipelines.clone().unwrap().len();
            self.pipelines
                .as_mut()
                .unwrap()
                .sort_by_key(|p| p.clone().name);
            self.pipelines
                .as_mut()
                .unwrap()
                .dedup_by_key(|p| p.clone().name);

            let end_length = &self.pipelines.clone().unwrap().len();
            if init_length != end_length {
                let message = "Removed pipelines with identical names";
                warn!("{}", message)
            }
        }
        self.to_owned()
    }
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

        Event {
            trigger: Trigger::flag(None).unwrap(),
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
