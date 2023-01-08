use super::{Command, Pipeline, Step, Trigger};
use crate::cast;
use crate::types;
use chrono::Utc;
use std::convert::From;
use utils::git::Hook;
use uuid::Uuid;

impl From<&cast::Pipeline> for Pipeline {
    fn from(e: &cast::Pipeline) -> Self {
        let steps = e.steps.iter().map(|e| Step::from(e)).collect::<Vec<Step>>();
        // let triggers = vec![];
        // for e in e.triggers.unwrap() {
        //     let tuples = Trigger::from(&e);
        //     for tuple in tuples {
        //         triggers.push(tuple);
        //     }
        // }
        // println!("{:?}", triggers);
        let p = Pipeline {
            pid: None,
            uuid: Uuid::new_v4(),
            date: Some(Utc::now().to_string()),
            name: e.name.to_owned(),
            steps: steps,
            status: None,
            // triggers: Some(triggers),
            triggers: None,
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
                tuplelist.push(types::Trigger {
                    branch: Some(branch.to_owned()),
                    action: Some(Hook::from_str(&action)),
                })
            }
        }
        return tuplelist;
    }
}
