use crate::cast;
use crate::types::characters::Characters;
use crate::types::Mode::*;
use crate::types::{
    Command, Config, Event, Fallback, Mode, Node, Parallel, Pipeline, Step, StepOrParallel,
    Trigger, TriggerBranch, TriggerTag,
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
use utils::logger::logger;
use uuid::Uuid;

use log::error;

impl From<&Event> for String {
    fn from(e: &Event) -> String {
        let mut string = "".to_owned();
        let mut date = e.date.parse::<DateTime<Local>>().unwrap().to_rfc2822();
        date = format!("{}\n", date);
        string.push_str(&date);

        let header = "action: ";
        let action = format!(
            "{}{}\n",
            header.white(),
            String::from(&e.trigger.action().clone().unwrap()).white()
        );
        string.push_str(&action);
        if e.trigger.tag().is_some() {
            let header = "branch: ";
            let tag = format!(
                "{}{}\n",
                header.white(),
                String::from(&e.trigger.tag().clone().unwrap()).white()
            );
            string.push_str(&tag);
        } else if e.trigger.branch().is_some() {
            let header = "branch: ";
            let branch = format!(
                "{}{}\n",
                header.white(),
                String::from(&e.trigger.branch().clone().unwrap()).white()
            );
            string.push_str(&branch);
        }
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
        // Convert steps
        let steps = &e
            .steps
            .iter()
            .map(|e| StepOrParallel::from(e))
            .collect::<Vec<StepOrParallel>>();

        // Convert fallback
        let mut fallback = None;
        if e.fallback.is_some() {
            fallback = Some(Fallback::from(e.fallback.as_ref().unwrap()));
        }
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
            fallback: fallback,
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
        let commands = e
            .commands
            .iter()
            .map(|e| Command::from(e))
            .collect::<Vec<Command>>();

        // Convert fallback
        let mut fallback = None;
        if e.fallback.is_some() {
            fallback = Some(Fallback::from(e.fallback.as_ref().unwrap()));
        }

        // Convert mode
        let mut mode = None;
        if e.mode.is_some() {
            mode = Some(Mode::from(e.mode.as_ref().unwrap()));
        }

        let default_step = Step::new();
        Step {
            name: e.clone().name,
            mode: mode,
            commands: commands,
            status: None,
            fallback: fallback,
            ..Step::default()
        }
    }
}
impl From<&cast::Parallel> for Parallel {
    fn from(e: &cast::Parallel) -> Self {
        // Convert fallback
        let mut fallback = None;
        if e.fallback.is_some() {
            fallback = Some(Fallback::from(e.fallback.as_ref().unwrap()));
        }

        // Convert mode
        let mut mode = None;
        if e.mode.is_some() {
            mode = Some(Mode::from(e.mode.as_ref().unwrap()));
        }

        let mut res = Parallel {
            mode: mode,
            fallback: fallback,
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

impl From<&cast::Fallback> for Fallback {
    fn from(e: &cast::Fallback) -> Self {
        // Convert post-run steps
        let mut on_started = None;
        if e.on_started.is_some() {
            let binding = e.on_started.clone().unwrap();
            on_started = Some(
                binding
                    .iter()
                    .map(|e| StepOrParallel::from(e))
                    .collect::<Vec<StepOrParallel>>(),
            );
        }
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
        return Fallback {
            on_started: on_started,
            on_failure: on_failure,
            on_success: on_success,
            on_abortion: on_abortion,
        };
    }
}
impl Trigger {
    pub fn flatten(e: &cast::Trigger) -> Vec<Trigger> {
        let mut tuplelist: Vec<Trigger> = vec![];
        match &e {
            cast::Trigger::TriggerBranch(res) => {
                if res.branches.is_none() {
                    for action in res.actions.clone().unwrap() {
                        tuplelist.push(Trigger::TriggerBranch(TriggerBranch {
                            action: Some(Flag::from(&action)),
                            branch: None,
                        }))
                    }
                }
                if res.actions.is_none() {
                    for branch in res.branches.clone().unwrap() {
                        tuplelist.push(Trigger::TriggerBranch(TriggerBranch {
                            action: None,
                            branch: Some(branch.to_owned()),
                        }))
                    }
                }
                if res.branches.is_some() && res.actions.is_some() {
                    for branch in res.branches.clone().unwrap() {
                        for action in res.actions.clone().unwrap() {
                            tuplelist.push(Trigger::TriggerBranch(TriggerBranch {
                                action: Some(Flag::from(&action)),
                                branch: Some(branch.to_owned()),
                            }))
                        }
                    }
                }
            }
            cast::Trigger::TriggerTag(res) => {
                if res.tags.is_none() {
                    for action in res.actions.clone().unwrap() {
                        tuplelist.push(Trigger::TriggerTag(TriggerTag {
                            action: Some(Flag::from(&action)),
                            tag: None,
                        }))
                    }
                }
                if res.actions.is_none() {
                    for tag in res.tags.clone().unwrap() {
                        tuplelist.push(Trigger::TriggerTag(TriggerTag {
                            action: None,
                            tag: Some(tag.to_owned()),
                        }))
                    }
                }
                if res.tags.is_some() && res.actions.is_some() {
                    for tag in res.tags.clone().unwrap() {
                        for action in res.actions.clone().unwrap() {
                            tuplelist.push(Trigger::TriggerTag(TriggerTag {
                                action: Some(Flag::from(&action)),
                                tag: Some(tag.to_owned()),
                            }))
                        }
                    }
                }
            }
        }
        return tuplelist;
    }
}
impl From<&String> for Mode {
    fn from(mode: &String) -> Mode {
        let cased: &str = &mode.to_case(Case::Snake);
        // let cased: &str = &mode.to_case(Case::Kebab);
        match cased {
            "stop" => return Mode::StopOnFailure,
            "jump_next" => return Mode::JumpNextOnFailure,
            "continue" => return Mode::ContinueOnFailure,
            _ => {
                let message = format!("The step execution mode {} is not known", cased);
                error!("{}", message);
                exit(1);
            }
        };
    }
}
impl From<&Mode> for String {
    fn from(mode: &Mode) -> String {
        match mode {
            StopOnFailure => return "stop".to_owned(),
            JumpNextOnFailure => return "jump_next".to_owned(),
            ContinueOnFailure => return "continue".to_owned(),
        };
    }
}

impl From<&Pipeline> for Node {
    fn from(e: &Pipeline) -> Self {
        let mut head: String = "".to_owned();
        if e.status.is_some() {
            let separator = format!("{}", " - ".white());
            let status = format!("{}{}", e.status.clone().unwrap(), separator);
            head.push_str(&status);
        }
        if e.event.is_some() {
            let event = String::from(&e.event.clone().unwrap());
            head.push_str(&format!("{}", &event.white()))
        }
        head = format!("{}", head);

        let name = format!("pipeline: {}", e.name.clone());
        head.push_str(&name);
        let mut children: Vec<Node> = e.steps.iter().map(|e| Node::from(e)).collect();

        // Fallback
        if e.fallback.is_some() {
            if e.fallback.clone().unwrap().on_failure.is_some() {
                let on_failure = e.fallback.clone().unwrap().on_failure.unwrap();

                let on_failure_children = on_failure.iter().map(|e| Node::from(e)).collect();
                let node = Node {
                    children: Some(on_failure_children),
                    value: Some("on_failure".to_owned()),
                    ..Node::default()
                };
                children.push(node);
            }
            if e.fallback.clone().unwrap().on_success.is_some() {
                let on_success = e.fallback.clone().unwrap().on_success.unwrap();
                let on_success_children = on_success.iter().map(|e| Node::from(e)).collect();
                let node = Node {
                    children: Some(on_success_children),
                    value: Some("on_success".to_owned()),
                    ..Node::default()
                };
                children.push(node);
            }
            if e.fallback.clone().unwrap().on_abortion.is_some() {
                let on_abortion = e.fallback.clone().unwrap().on_abortion.unwrap();
                let on_abortion_children = on_abortion.iter().map(|e| Node::from(e)).collect();
                let node = Node {
                    children: Some(on_abortion_children),
                    value: Some("on_abortion".to_owned()),
                    ..Node::default()
                };
                children.push(node);
            }
        }

        let node = Node {
            value: Some(head),
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
        let mut children: Vec<Node> = e.steps.iter().map(|el| Node::from(el)).collect();

        // Fallback
        if e.fallback.is_some() {
            if e.fallback.clone().unwrap().on_failure.is_some() {
                let on_failure = e.fallback.clone().unwrap().on_failure.unwrap();

                let on_failure_children = on_failure.iter().map(|e| Node::from(e)).collect();
                let node = Node {
                    children: Some(on_failure_children),
                    value: Some("on_failure".to_owned()),
                    ..Node::default()
                };
                children.push(node);
            }
            if e.fallback.clone().unwrap().on_success.is_some() {
                let on_success = e.fallback.clone().unwrap().on_success.unwrap();
                let on_success_children = on_success.iter().map(|e| Node::from(e)).collect();
                let node = Node {
                    children: Some(on_success_children),
                    value: Some("on_success".to_owned()),
                    ..Node::default()
                };
                children.push(node);
            }
            if e.fallback.clone().unwrap().on_abortion.is_some() {
                let on_abortion = e.fallback.clone().unwrap().on_abortion.unwrap();
                let on_abortion_children = on_abortion.iter().map(|e| Node::from(e)).collect();
                let node = Node {
                    children: Some(on_abortion_children),
                    value: Some("on_abortion".to_owned()),
                    ..Node::default()
                };
                children.push(node);
            }
        }

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
        let head = format!("step: {}", e.name.clone());
        let mut children: Vec<Node> = e.commands.iter().map(|el| Node::from(el)).collect();

        // Fallback
        if e.fallback.is_some() {
            if e.fallback.clone().unwrap().on_failure.is_some() {
                let on_failure = e.fallback.clone().unwrap().on_failure.unwrap();

                let on_failure_children = on_failure.iter().map(|e| Node::from(e)).collect();
                let node = Node {
                    children: Some(on_failure_children),
                    value: Some("on_failure".to_owned()),
                    ..Node::default()
                };
                children.push(node);
            }
            if e.fallback.clone().unwrap().on_success.is_some() {
                let on_success = e.fallback.clone().unwrap().on_success.unwrap();
                let on_success_children = on_success.iter().map(|e| Node::from(e)).collect();
                let node = Node {
                    children: Some(on_success_children),
                    value: Some("on_success".to_owned()),
                    ..Node::default()
                };
                children.push(node);
            }
            if e.fallback.clone().unwrap().on_abortion.is_some() {
                let on_abortion = e.fallback.clone().unwrap().on_abortion.unwrap();
                let on_abortion_children = on_abortion.iter().map(|e| Node::from(e)).collect();
                let node = Node {
                    children: Some(on_abortion_children),
                    value: Some("on_abortion".to_owned()),
                    ..Node::default()
                };
                children.push(node);
            }
        }
        let node = Node {
            value: Some(head),
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
