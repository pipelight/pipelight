use crate::pipeline::Filters;
use crate::types::{
    Command, Config, ConfigOpts, Fallback, Mode, Parallel, Pipeline, PipelineOpts, Step, StepOpts,
    StepOrParallel,
};
use crate::types::{Trigger, TriggerBranch, TriggerTag};
use exec::Process;
use log::LevelFilter;

use convert_case::{Case, Casing};

use std::convert::From;
use std::process::exit;
use utils::git::Flag;
use uuid::Uuid;

// Logger
use log::error;

impl From<&cast::ConfigOpts> for ConfigOpts {
    fn from(e: &cast::ConfigOpts) -> Self {
        let mut options = ConfigOpts::default();
        if let Some(log_level) = &e.log_level {
            let level = serde_plain::from_str::<LevelFilter>(log_level).unwrap();
            options.log_level = Some(level);
        }
        if let Some(attach) = e.attach {
            options.attach = Some(attach);
        }
        options
    }
}
impl From<&cast::Config> for Config {
    fn from(e: &cast::Config) -> Self {
        let mut config = Config::default();
        if e.pipelines.is_some() {
            let mut pipelines = e
                .clone()
                .pipelines
                .unwrap()
                .iter()
                .map(Pipeline::from)
                .collect();
            pipelines = Filters::dedup(pipelines).unwrap();
            config.pipelines = Some(pipelines);
            // Remove duplicates
        }
        config
    }
}

impl From<&cast::PipelineOpts> for PipelineOpts {
    fn from(e: &cast::PipelineOpts) -> Self {
        let mut options = PipelineOpts::default();
        if let Some(log_level) = &e.log_level {
            let level = serde_plain::from_str::<LevelFilter>(log_level).unwrap();
            options.log_level = Some(level);
        }
        if let Some(attach) = e.attach {
            options.attach = Some(attach);
        }
        options
    }
}

impl From<&cast::Pipeline> for Pipeline {
    fn from(e: &cast::Pipeline) -> Self {
        let mut options = None;
        if let Some(cast_options) = &e.options {
            options = Some(PipelineOpts::from(cast_options));
        }
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
            options,
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

impl From<&cast::StepOpts> for StepOpts {
    fn from(e: &cast::StepOpts) -> Self {
        let mut options = StepOpts::default();
        if let Some(mode) = &e.mode {
            options.mode = Some(Mode::from(mode));
        }
        options
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

        // Convert options
        let mut options = None;
        if e.options.is_some() {
            options = Some(StepOpts::from(e.options.as_ref().unwrap()));
        }

        Step {
            name: e.clone().name,
            commands,
            status: None,
            fallback,
            options,
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
        let mut res = Parallel {
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
                        tuplelist.push(Trigger::TriggerBranch(TriggerBranch {
                            action: Some(Flag::from(&action)),
                            branch: None,
                            ..TriggerBranch::default()
                        }))
                    }
                }
                if res.actions.is_none() {
                    for branch in res.branches.clone().unwrap() {
                        tuplelist.push(Trigger::TriggerBranch(TriggerBranch {
                            action: None,
                            branch: Some(branch.to_owned()),
                            ..TriggerBranch::default()
                        }))
                    }
                }
                if res.branches.is_some() && res.actions.is_some() {
                    for branch in res.branches.clone().unwrap() {
                        for action in res.actions.clone().unwrap() {
                            tuplelist.push(Trigger::TriggerBranch(TriggerBranch {
                                action: Some(Flag::from(&action)),
                                branch: Some(branch.to_owned()),
                                ..TriggerBranch::default()
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
                            ..TriggerTag::default()
                        }))
                    }
                }
                if res.actions.is_none() {
                    for tag in res.tags.clone().unwrap() {
                        tuplelist.push(Trigger::TriggerTag(TriggerTag {
                            action: None,
                            tag: Some(tag.to_owned()),
                            ..TriggerTag::default()
                        }))
                    }
                }
                if res.tags.is_some() && res.actions.is_some() {
                    for tag in res.tags.clone().unwrap() {
                        for action in res.actions.clone().unwrap() {
                            tuplelist.push(Trigger::TriggerTag(TriggerTag {
                                action: Some(Flag::from(&action)),
                                tag: Some(tag.to_owned()),
                                ..TriggerTag::default()
                            }))
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
