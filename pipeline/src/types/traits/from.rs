use crate::cast;
use crate::types::characters::Characters;
use crate::types::{
    Command, Config, Event, Node, Parallel, Pipeline, Step, StepOrParallel, Trigger,
};
use chrono::Utc;
use chrono::{DateTime, Local};
use colored::{ColoredString, Colorize};
use convert_case::{Case, Casing};
use exec::types::Status;
use log::LevelFilter;
use std::convert::From;
use std::process::exit;
use utils::git::Flag;
use utils::git::Hook;
use uuid::Uuid;

impl From<&Event> for String {
    fn from(e: &Event) -> String {
        let mut string = "".to_owned();
        let mut date = e.date.parse::<DateTime<Local>>().unwrap().to_rfc2822();
        date = format!("{}\n", date);
        let header = "action: ";
        let action = format!(
            "{}{}\n",
            header.white(),
            String::from(&e.trigger.action.clone().unwrap()).white()
        );
        let header = "branch: ";
        let branch = format!(
            "{}{}\n",
            header.white(),
            String::from(&e.trigger.branch.clone().unwrap()).white()
        );
        string.push_str(&date);
        string.push_str(&action);
        string.push_str(&branch);
        return string;
    }
}
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
        // Convert post-run steps
        let mut on_failure = None;
        if e.on_failure.is_some() {
            let binding = e.on_failure.clone().unwrap();
            on_failure = Some(
                binding
                    .iter()
                    .map(|e| StepOrParallel::from(e))
                    .collect::<Vec<StepOrParallel>>(),
            );
        }
        // Convert post-run steps
        let mut on_success = None;
        if e.on_success.is_some() {
            let binding = e.on_success.clone().unwrap();
            on_success = Some(
                binding
                    .iter()
                    .map(|e| StepOrParallel::from(e))
                    .collect::<Vec<StepOrParallel>>(),
            );
        }
        // Convert post-run steps
        let mut on_abortion = None;
        if e.on_abortion.is_some() {
            let binding = e.on_abortion.clone().unwrap();
            on_abortion = Some(
                binding
                    .iter()
                    .map(|e| StepOrParallel::from(e))
                    .collect::<Vec<StepOrParallel>>(),
            );
        }
        // Convert steps
        let steps = &e
            .steps
            .iter()
            .map(|e| StepOrParallel::from(e))
            .collect::<Vec<StepOrParallel>>();

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
            on_success: on_success,
            on_failure: on_failure,
            on_abortion: on_abortion,
        };
        return p;
    }
}

impl From<&cast::StepOrParallel> for StepOrParallel {
    fn from(e: &cast::StepOrParallel) -> Self {
        match e {
            cast::StepOrParallel::Step(res) => StepOrParallel::Step(Step::from(res)),
            cast::StepOrParallel::Parallel(res) => StepOrParallel::Parallel(Parallel::from(res)),
        }
    }
}

impl From<&cast::Step> for Step {
    fn from(e: &cast::Step) -> Self {
        // Convert post-run steps
        let mut on_failure = None;
        if e.on_failure.is_some() {
            let binding = e.on_failure.clone().unwrap();
            on_failure = Some(
                binding
                    .iter()
                    .map(|e| StepOrParallel::from(e))
                    .collect::<Vec<StepOrParallel>>(),
            );
        }
        // Convert post-run steps
        let mut on_success = None;
        if e.on_success.is_some() {
            let binding = e.on_success.clone().unwrap();
            on_success = Some(
                binding
                    .iter()
                    .map(|e| StepOrParallel::from(e))
                    .collect::<Vec<StepOrParallel>>(),
            );
        }
        // Convert post-run steps
        let mut on_abortion = None;
        if e.on_abortion.is_some() {
            let binding = e.on_abortion.clone().unwrap();
            on_abortion = Some(
                binding
                    .iter()
                    .map(|e| StepOrParallel::from(e))
                    .collect::<Vec<StepOrParallel>>(),
            );
        }
        let commands = e
            .commands
            .iter()
            .map(|e| Command::from(e))
            .collect::<Vec<Command>>();
        let default_step = Step::new();
        Step {
            name: e.clone().name,
            non_blocking: e.clone().non_blocking,
            commands: commands,
            status: None,
            on_success: on_success,
            on_failure: on_failure,
            on_abortion: on_abortion,
            ..Step::default()
        }
    }
}
impl From<&cast::Parallel> for Parallel {
    fn from(e: &cast::Parallel) -> Self {
        let mut res = Parallel {
            steps: vec![],
            ..Parallel::new()
        };
        for step in &e.parallel {
            res.steps.push(Step::from(step));
        }
        return res;
    }
}

impl From<&String> for Command {
    fn from(s: &String) -> Self {
        Command {
            status: None,
            stdin: s.to_owned(),
            output: None,
            ..Command::default()
        }
    }
}

impl Trigger {
    pub fn flatten(e: &cast::Trigger) -> Vec<Trigger> {
        let mut tuplelist: Vec<Trigger> = vec![];
        for branch in e.branches.clone() {
            if e.actions.clone().is_some() {
                for action in e.actions.clone().unwrap() {
                    tuplelist.push(Trigger {
                        branch: Some(branch.to_owned()),
                        action: Some(Flag::from(&action)),
                    })
                }
            } else {
                tuplelist.push(Trigger {
                    branch: Some(branch.to_owned()),
                    action: Some(Flag::Manual),
                })
            }
        }
        return tuplelist;
    }
}

impl From<&Pipeline> for Node {
    fn from(e: &Pipeline) -> Self {
        let mut tag: String = "".to_owned();
        if e.status.is_some() {
            let status = format!("{} - ", e.status.clone().unwrap());
            tag.push_str(&status);
        }
        if e.event.is_some() {
            let event = String::from(&e.event.clone().unwrap());
            tag.push_str(&event);
        }
        tag = format!("{}", tag.white());

        let name = format!("pipeline: {}", e.name.clone());
        tag.push_str(&name);

        let children = e.steps.iter().map(|e| Node::from(e)).collect();
        let node = Node {
            value: Some(tag),
            status: e.status.clone(),
            duration: e.duration,
            children: Some(children),
            ..Node::default()
        };
        return node;
    }
}
impl From<&StepOrParallel> for Node {
    fn from(e: &StepOrParallel) -> Self {
        match e {
            StepOrParallel::Step(res) => Node::from(res),
            StepOrParallel::Parallel(res) => Node::from(res),
        }
    }
}
impl From<&Parallel> for Node {
    fn from(e: &Parallel) -> Self {
        let children = e.steps.iter().map(|el| Node::from(el)).collect();
        let node = Node {
            value: Some("parallel".to_owned()),
            status: e.status.clone(),
            duration: e.duration,
            children: Some(children),
            level: LevelFilter::Warn,
            ..Node::default()
        };
        return node;
    }
}
impl From<&Step> for Node {
    fn from(e: &Step) -> Self {
        let tag = format!("step: {}", e.name.clone());
        let children = e.commands.iter().map(|el| Node::from(el)).collect();
        let node = Node {
            value: Some(tag),
            status: e.status.clone(),
            duration: e.duration,
            children: Some(children),
            level: LevelFilter::Warn,
            ..Node::default()
        };
        return node;
    }
}

impl From<&Command> for Node {
    fn from(e: &Command) -> Self {
        let mut node = Node {
            level: LevelFilter::Info,
            duration: e.duration,
            ..Node::default()
        };
        // Convert command output as child node
        if e.output.is_some() {
            if e.output.clone().unwrap().stdout.is_some()
                | e.output.clone().unwrap().stderr.is_some()
            {
                let out = match e.status {
                    Some(Status::Succeeded) => e.output.clone().unwrap().stdout,
                    Some(Status::Failed) => e.output.clone().unwrap().stderr,
                    Some(Status::Started) => None,
                    Some(Status::Aborted) => None,
                    Some(Status::Running) => None,
                    None => None,
                };
                let child = Node {
                    value: out,
                    status: e.clone().status,
                    children: None,
                    level: LevelFilter::Debug,
                    ..Node::new()
                };
                node.children = Some(vec![child]);
            }
        }
        node.value = Some(e.stdin.clone());
        node.status = e.status.clone();
        return node;
    }
}
