use crate::cast;
use crate::types::{
    Command, Config, Event, Logs, Parallel, Pipeline, Step, StepOrParallel, Trigger,
};
use chrono::Utc;
use log::{info, trace, warn};
use std::env;
use std::process;
use utils::git::Git;
use uuid::Uuid;

impl Default for Config {
    fn default() -> Self {
        Config {
            pipelines: None,
            hooks: None,
        }
    }
}
impl Config {
    pub fn new() -> Self {
        let origin = env::current_dir().unwrap();
        Git::new().teleport();
        let json = cast::Config::get();
        let mut config = Config::from(&json);
        config.dedup_pipelines();
        env::set_current_dir(origin).unwrap();
        return config;
    }
    /// Remove pipelines with the same name
    pub fn dedup_pipelines(&mut self) -> Self {
        if self.pipelines.is_some() {
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

            let message = "Removed pipelines with identical names";
            warn!("{}", message)
        }
        return self.to_owned();
    }
}
impl Default for StepOrParallel {
    fn default() -> Self {
        let commands = vec![Command::default()];
        let step = Step {
            name: "default".to_owned(),
            status: None,
            commands: commands,
            non_blocking: None,
            on_failure: None,
        };
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
            parallel: vec![Step::default()],
            status: None,
            non_blocking: None,
            on_failure: None,
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
            commands: commands,
            non_blocking: None,
            on_failure: None,
            status: None,
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
            stdin: "".to_owned(),
            output: None,
        }
    }
}

impl Default for Pipeline {
    fn default() -> Self {
        let steps = vec![StepOrParallel::Step(Step::default())];
        Pipeline {
            uuid: Uuid::new_v4(),
            name: "default".to_owned(),
            duration: None,
            event: None,
            status: None,
            triggers: None,
            steps: steps,
        }
    }
}
impl Pipeline {
    pub fn new() -> Self {
        Pipeline::default()
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
        let pid = process::id();
        Event {
            trigger: Trigger::env().unwrap(),
            date: Utc::now().to_string(),
            pid: Some(pid),
        }
    }
}
impl Event {
    pub fn new() -> Event {
        Self::default()
    }
}
