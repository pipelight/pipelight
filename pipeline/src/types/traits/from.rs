use crate::cast;
use crate::types::{Command, Config, Event, Pipeline, Step, Trigger};
use chrono::Utc;
use convert_case::{Case, Casing};
use log::error;
use std::convert::From;
use std::process::exit;
use utils::git::Flag;
use utils::git::Hook;
use uuid::Uuid;

impl From<&cast::Config> for Config {
    fn from(e: &cast::Config) -> Self {
        let mut config = Config::default();
        if e.pipelines.is_some() {
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
        let triggers: Option<Vec<Trigger>>;
        if e.triggers.is_none() {
            triggers = None
        } else {
            Hook::new().unwrap();
            triggers = Some(
                e.clone()
                    .triggers
                    .unwrap()
                    .into_iter()
                    .map(|e| Trigger::flatten(&e))
                    .collect::<Vec<Vec<Trigger>>>()
                    .into_iter()
                    .flatten()
                    .collect::<Vec<Trigger>>(),
            )
        }
        let p = Pipeline {
            uuid: Uuid::new_v4(),
            name: e.name.to_owned(),
            duration: None,
            event: None,
            status: None,
            triggers: triggers,
            steps: steps.to_owned(),
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
        let default_step = Step::new();
        Step {
            name: e.clone().name,
            commands: commands,
            non_blocking: e.clone().non_blocking,
            on_failure: e.clone().on_failure,
            ..default_step
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
                    action: Some(Flag::from(&action)),
                })
            }
        }
        return tuplelist;
    }
}
