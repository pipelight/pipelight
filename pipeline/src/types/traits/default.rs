use crate::cast;
use crate::types::{Command, Config, Event, Logs, Pipeline, Step, Trigger};
use chrono::Utc;
use std::env;
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
        let config = Config::from(&json);
        env::set_current_dir(origin).unwrap();
        return config;
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
        Event {
            trigger: Trigger::env().unwrap(),
            date: Utc::now().to_string(),
            pid: None,
        }
    }
}
impl Event {
    pub fn new() -> Event {
        Self::default()
    }
}
