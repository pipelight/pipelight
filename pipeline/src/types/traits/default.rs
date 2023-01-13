use crate::cast;
use crate::types::{Command, Config, Pipeline, Step};
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
        let json = cast::Config::new();
        let mut config = Config::from(&json);
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
            pid: None,
            name: "default".to_owned(),
            date: None,
            status: None,
            triggers: None,
            steps: steps,
        }
    }
}
impl Pipeline {
    pub fn new() -> Self {
        Self::default()
    }
}
