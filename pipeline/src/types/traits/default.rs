use crate::cast;
use crate::types::{Command, Config, Event, Logs, Pipeline, Step, Trigger};
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
impl Default for Pipeline {
    fn default() -> Self {
        let commands = vec![Command {
            stdin: "".to_owned(),
            output: None,
        }];
        let steps = vec![Step {
            name: "default".to_owned(),
            commands: commands,
            non_blocking: None,
            on_failure: None,
        }];
        Pipeline {
            uuid: Uuid::new_v4(),
            name: "default".to_owned(),
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
