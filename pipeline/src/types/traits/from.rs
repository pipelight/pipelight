use crate::cast;
use crate::types::{Command, Config, Pipeline, Step, Trigger};
use chrono::Utc;
use std::convert::From;
use std::str::FromStr;
use utils::git::Hook;
use uuid::Uuid;

impl From<&cast::Config> for Config {
    fn from(e: &cast::Config) -> Self {
        let mut config = Config::default();
        if e.pipelines.is_some() {
            println!("{:?}", e);
            let pipelines = e
                .clone()
                .pipelines
                .unwrap()
                .iter()
                .map(|e| Pipeline::from(e))
                .collect();
            config.pipelines = Some(pipelines);
        }
        return config;
    }
}

impl From<&cast::Pipeline> for Pipeline {
    fn from(e: &cast::Pipeline) -> Self {
        let steps = &e.steps.iter().map(|e| Step::from(e)).collect::<Vec<Step>>();
        // Flatten triggers
        let triggers = &e
            .clone()
            .triggers
            .unwrap()
            .into_iter()
            .map(|e| Trigger::flatten(&e))
            .collect::<Vec<Vec<Trigger>>>()
            .into_iter()
            .flatten()
            .collect::<Vec<Trigger>>();
        let p = Pipeline {
            pid: None,
            uuid: Uuid::new_v4(),
            date: Some(Utc::now().to_string()),
            name: e.name.to_owned(),
            steps: steps.to_owned(),
            status: None,
            triggers: Some(triggers.to_owned()),
        };
        return p;
    }
}

impl From<&cast::Step> for Step {
    fn from(e: &cast::Step) -> Self {
        let commands = e
            .commands
            .iter()
            .map(|e| Command::from(e))
            .collect::<Vec<Command>>();
        Step {
            name: e.clone().name,
            commands: commands,
            non_blocking: e.non_blocking,
            on_failure: e.clone().on_failure,
        }
    }
}

impl From<&String> for Command {
    fn from(s: &String) -> Self {
        Command {
            stdin: s.to_owned(),
            output: None,
        }
    }
}

impl Trigger {
    pub fn flatten(e: &cast::Trigger) -> Vec<Trigger> {
        let mut tuplelist: Vec<Trigger> = vec![];
        for branch in e.branches.clone() {
            for action in e.actions.clone().unwrap() {
                tuplelist.push(Trigger {
                    branch: Some(branch.to_owned()),
                    action: Some(Hook::from_str(&action).unwrap()),
                })
            }
        }
        return tuplelist;
    }
}
