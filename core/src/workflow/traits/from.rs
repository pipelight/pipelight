use crate::workflow::types::{
    Command, Config, Fallback, Mode, Parallel, Pipeline, Step, StepOrParallel, Trigger,
};
use exec::Process;

use convert_case::{Case, Casing};

use std::convert::From;
use std::process::exit;
use utils::git::Flag;
use uuid::Uuid;

// Logger
use log::error;

impl From<&cast::Config> for Config {
    fn from(e: &cast::Config) -> Self {
        let mut config = Config::default();
        if e.pipelines.is_some() {
            let pipelines = e
                .clone()
                .pipelines
                .unwrap()
                .iter()
                .map(Pipeline::from)
                .collect();
            config.pipelines = Some(pipelines);
            // Remove duplicates
            config.dedup_pipelines();
        }
        config
    }
}

impl From<&cast::Pipeline> for Pipeline {
    fn from(e: &cast::Pipeline) -> Self {
        // Convert steps
        let steps = &e
            .steps
            .iter()
            .map(StepOrParallel::from)
            .collect::<Vec<StepOrParallel>>();

        // Convert fallback
        let mut fallback = None;
        if e.fallback.is_some() {
            fallback = Some(Fallback::from(e.fallback.as_ref().unwrap()));
        }
        // Flatten triggers
        let triggers: Option<Vec<Trigger>> = if e.triggers.is_none() {
            None
        } else {
            Some(
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
        };
        Pipeline {
            uuid: Uuid::new_v4(),
            name: e.name.to_owned(),
            duration: None,
            event: None,
            status: None,
            triggers,
            steps: steps.to_owned(),
            fallback,
        }
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
            .map(Command::from)
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
            mode,
            commands,
            status: None,
            fallback,
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
            mode,
            fallback,
            steps: vec![],
            ..Parallel::new()
        };

        for step in &e.parallel {
            res.steps.push(Step::from(step));
        }
        res
    }
}

impl From<&String> for Command {
    fn from(s: &String) -> Self {
        Command {
            process: Process::new(s),
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
                    .map(StepOrParallel::from)
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
                    .map(StepOrParallel::from)
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
                    .map(StepOrParallel::from)
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
                    .map(StepOrParallel::from)
                    .collect::<Vec<StepOrParallel>>(),
            );
        }
        Fallback {
            on_started,
            on_failure,
            on_success,
            on_abortion,
        }
    }
}
impl Trigger {
    pub fn flatten(e: &cast::Trigger) -> Vec<Trigger> {
        let mut tuplelist: Vec<Trigger> = vec![];
        match &e {
            cast::Trigger::TriggerBranch(res) => {
                if res.branches.is_none() {
                    for action in res.actions.clone().unwrap() {
                        tuplelist.push(Trigger {
                            action: Some(Flag::from(&action)),
                            branch: None,
                            tag: None,
                        })
                    }
                }
                if res.actions.is_none() {
                    for branch in res.branches.clone().unwrap() {
                        tuplelist.push(Trigger {
                            action: None,
                            branch: Some(branch.to_owned()),
                            tag: None,
                        })
                    }
                }
                if res.branches.is_some() && res.actions.is_some() {
                    for branch in res.branches.clone().unwrap() {
                        for action in res.actions.clone().unwrap() {
                            tuplelist.push(Trigger {
                                action: Some(Flag::from(&action)),
                                branch: Some(branch.to_owned()),
                                tag: None,
                            })
                        }
                    }
                }
            }
            cast::Trigger::TriggerTag(res) => {
                if res.tags.is_none() {
                    for action in res.actions.clone().unwrap() {
                        tuplelist.push(Trigger {
                            action: Some(Flag::from(&action)),
                            tag: None,
                            branch: None,
                        })
                    }
                }
                if res.actions.is_none() {
                    for tag in res.tags.clone().unwrap() {
                        tuplelist.push(Trigger {
                            action: None,
                            tag: Some(tag.to_owned()),
                            branch: None,
                        })
                    }
                }
                if res.tags.is_some() && res.actions.is_some() {
                    for tag in res.tags.clone().unwrap() {
                        for action in res.actions.clone().unwrap() {
                            tuplelist.push(Trigger {
                                action: Some(Flag::from(&action)),
                                tag: Some(tag.to_owned()),
                                branch: None,
                            })
                        }
                    }
                }
            }
        }
        tuplelist
    }
}
impl From<&String> for Mode {
    fn from(mode: &String) -> Mode {
        let cased: &str = &mode.to_case(Case::Snake);
        // let cased: &str = &mode.to_case(Case::Kebab);
        match cased {
            "stop" => Mode::StopOnFailure,
            "jump_next" => Mode::JumpNextOnFailure,
            "continue" => Mode::ContinueOnFailure,
            _ => {
                let message = format!("The step execution mode {} is not known", cased);
                error!("{}", message);
                exit(1);
            }
        }
    }
}
impl From<&Mode> for String {
    fn from(mode: &Mode) -> String {
        match mode {
            Mode::StopOnFailure => "stop".to_owned(),
            Mode::JumpNextOnFailure => "jump_next".to_owned(),
            Mode::ContinueOnFailure => "continue".to_owned(),
        }
    }
}
