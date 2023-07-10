use crate::types::{
    Command, Config, Event, Logs, Mode, Node, Parallel, Pipeline, Step, StepOrParallel, Trigger,
    TriggerBranch, TriggerTag,
};
// use cast;
use exec::Process;

use chrono::Utc;
use log::LevelFilter;
use log::{info, trace, warn};
use uuid::Uuid;

// External imports
use utils::git::{Flag, Git, Hook};

// Error Handling
use miette::Result;

// sys
use rustix::process::{getpgid, getpid, getsid, Pid};

// Global var
use once_cell::sync::Lazy;

// Global var
pub static mut CONFIG: Lazy<Config> = Lazy::new(Config::default);
pub static mut TRIGGER_ENV: Lazy<Option<Trigger>> = Lazy::new(|| None);

impl Trigger {
    /// Return actual triggering env with a modified flag
    pub fn flag(flag: Flag) -> Result<Trigger> {
        // Get the gloabl env
        let mut env = Trigger::env()?;
        env.set_action(Some(flag));
        unsafe {
            // Set the gloabl env
            *TRIGGER_ENV = Some(env.clone());
        }
        Ok(env)
    }
    /// Return actual triggering env
    pub fn env() -> Result<Trigger> {
        // Get the gloabl env
        let global_env;
        unsafe {
            global_env = (*TRIGGER_ENV).clone();
        }
        let env: Trigger;

        if global_env.is_some() {
            env = global_env.unwrap();
        } else {
            // Default trigger values
            let mut branch = None;
            let mut tag = None;
            let action = Some(Hook::origin()?);

            if Git::new().exists() {
                branch = Git::new().get_branch()?;
                tag = Git::new().get_tag()?;
            }
            if tag.is_some() {
                env = Trigger::TriggerTag(TriggerTag { action, tag });
            } else if branch.is_some() {
                env = Trigger::TriggerBranch(TriggerBranch { action, branch });
            } else {
                env = Trigger::TriggerBranch(TriggerBranch {
                    action,
                    branch: None,
                });
            }
            unsafe {
                // Set the gloabl env
                *TRIGGER_ENV = Some(env.clone());
            }
        }
        Ok(env)
    }
}

impl Config {
    pub fn new(file: Option<String>, args: Option<Vec<String>>) -> Result<Self> {
        unsafe {
            if *CONFIG == Config::default() {
                let mut config: Config;
                let json = cast::Config::get(file, args)?;
                config = Config::from(&json);
                config.dedup_pipelines();
                *CONFIG = config;
            }
            let ptr = (*CONFIG).to_owned();
            Ok(ptr)
        }
    }
    /// Remove pipelines with the same name
    fn dedup_pipelines(&mut self) -> Self {
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
            commands,
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
// impl Default for Command {
//     fn default() -> Self {
//         Command {
//             duration: None,
//             process: Process::default(),
//         }
//     }
// }
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
            trigger: Trigger::env().unwrap(),
            date: Utc::now().to_string(),
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
