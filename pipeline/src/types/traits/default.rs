use crate::cast;
use crate::types::{
    Command, Config, Event, Logs, Mode, Node, Parallel, Pipeline, Step, StepOrParallel, Trigger,
};
use chrono::Utc;
use log::LevelFilter;
use log::{info, trace, warn};
use std::env;
use std::process;
use utils::git::Git;
use uuid::Uuid;

// Error Handling
use miette::{miette, Diagnostic, Error, IntoDiagnostic, NamedSource, Report, Result, SourceSpan};
use thiserror::Error;

// sys
use sysinfo::{Pid, PidExt, ProcessExt, System, SystemExt};

// Global var
use once_cell::sync::Lazy;
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Mutex, RwLock};

// Global var
static mut CONFIG: Lazy<Config> = Lazy::new(|| Config::default());

impl Default for Config {
    fn default() -> Self {
        Config { pipelines: None }
    }
}
impl Config {
    pub fn new(args: Option<Vec<String>>) -> Result<Self> {
        unsafe {
            if *CONFIG == Config::default() {
                let mut config: Config;
                let json = cast::Config::get(args).unwrap();
                config = Config::from(&json);
                config.dedup_pipelines();
                *CONFIG = config;
            }
            let ptr = (*CONFIG).clone().to_owned();
            return Ok(ptr);
        }
    }
    /// Remove pipelines with the same name
    pub fn dedup_pipelines(&mut self) -> Self {
        if self.pipelines.is_some() {
            let init_length = &self.pipelines.clone().unwrap().len();
            &self
                .pipelines
                .as_mut()
                .unwrap()
                .sort_by_key(|p| p.clone().name);
            &self
                .pipelines
                .as_mut()
                .unwrap()
                .dedup_by_key(|p| p.clone().name);

            let end_length = &self.pipelines.clone().unwrap().len();
            if init_length != end_length {
                let message = "Removed pipelines with identical names";
                warn!("{}", message)
            }
        }
        return self.to_owned();
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
            steps: steps,
            fallback: None,
        }
    }
}
impl Pipeline {
    pub fn new() -> Self {
        Pipeline::default()
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
        StepOrParallel::default()
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
        Parallel::default()
    }
}
impl Default for Step {
    fn default() -> Self {
        let commands = vec![Command::default()];
        Step {
            name: "default".to_owned(),
            status: None,
            duration: None,
            commands: commands,
            mode: Some(Mode::StopOnFailure),
            fallback: None,
        }
    }
}
impl Step {
    pub fn new() -> Self {
        Step::default()
    }
}
impl Default for Command {
    fn default() -> Self {
        Command {
            status: None,
            duration: None,
            stdin: "".to_owned(),
            output: None,
        }
    }
}
impl Command {
    pub fn new() -> Command {
        Self::default()
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
        // Get process pid
        let pid = process::id();
        // Get current process info
        let mut sys = System::new_all();
        sys.refresh_all();
        let current_process = sys.process(PidExt::from_u32(pid)).unwrap();
        // Get process spid
        let sid = current_process.session_id().unwrap();

        Event {
            trigger: Trigger::env().unwrap(),
            date: Utc::now().to_string(),
            pid: Some(pid),
            sid: Some(sid.as_u32()),
        }
    }
}
impl Event {
    pub fn new() -> Event {
        Self::default()
    }
}
