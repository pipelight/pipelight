// Struct for pipeline execution loggin.
// PipelineLog is parsed as json into a log file
#![allow(dead_code)]
use crate::exec::subprocess::exec;
use chrono::{DateTime, Local, NaiveDateTime, Offset, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::{Result, Value};
use std::clone::Clone;
use std::cmp::PartialEq;
use std::convert::From;
use std::error::Error;
use std::marker::Copy;
use std::ops::{Deref, DerefMut};
use std::process::{ExitStatus, Output};
use uuid::{uuid, Uuid};

use crate::types::config::{Pipeline, Step, Trigger};

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum PipelineStatus {
    Started,
    Succeeded,
    Failed,
    Running,
    Aborted,
    Never,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PipelineLog {
    pub uuid: Uuid,
    pub name: String,
    pub date: Option<String>,
    pub status: PipelineStatus,
    pub trigger: Option<Trigger>,
    pub steps: Vec<StepLog>,
}
impl PipelineLog {
    pub fn run(&mut self) {
        let mut x = DerefMutPipeline {
            value: self.clone(),
        };
        for step in &mut self.steps {
            step.run();
        }
        *x = self.clone();
    }
}

impl From<Pipeline> for PipelineLog {
    fn from(e: Pipeline) -> Self {
        let steps = e
            .steps
            .iter()
            .map(|e| StepLog::from(e))
            .collect::<Vec<StepLog>>();
        return PipelineLog {
            uuid: Uuid::new_v4(),
            date: Some(Utc::now().to_string()),
            name: e.name,
            steps: steps,
            status: PipelineStatus::Started,
            trigger: None,
        };
    }
}
struct DerefMutPipeline<T> {
    value: T,
}
impl<T> Deref for DerefMutPipeline<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}
impl<T: std::fmt::Debug> DerefMut for DerefMutPipeline<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        println!("{:?}", self.value);
        return &mut self.value;
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StepLog {
    pub name: String,
    pub commands: Vec<CommandLog>,
    pub non_blocking: Option<bool>,
    pub on_failure: Option<Vec<String>>,
}
impl StepLog {
    fn run(&mut self) {
        for command in &mut self.commands {
            command.run();
        }
        // println!("{:?}", self.commands);
    }
}
impl From<&Step> for StepLog {
    fn from(e: &Step) -> Self {
        let commands = e
            .commands
            .iter()
            .map(|e| CommandLog::from(e))
            .collect::<Vec<CommandLog>>();
        StepLog {
            name: e.clone().name,
            commands: commands,
            non_blocking: e.non_blocking,
            on_failure: e.clone().on_failure,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct CommandLog {
    pub stdin: String,
    output: Option<StrOutput>,
}
impl CommandLog {
    fn new() -> Self {
        return CommandLog {
            stdin: "".to_owned(),
            output: None,
        };
    }
    fn run(&mut self) {
        let mut output = exec(&self.stdin).ok();
        output = Some(StrOutput::from(output.unwrap()));
        self.output = output;
        // println!("{:?}", self.output);
    }
}
impl From<&String> for CommandLog {
    fn from(s: &String) -> Self {
        CommandLog {
            stdin: s.to_owned(),
            output: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StrOutput {
    pub status: ExitStatus,
    pub stdout: Option<String>,
    pub stderr: Option<String>,
}
impl From<&Output> for StrOutput {
    fn from(s: &Output) -> Self {
        let stdout = String::from_utf8(s.clone().stdout).unwrap().to_owned();
        let stderr = String::from_utf8(s.clone().stderr).unwrap().to_owned();
        return StrOutput {
            status: s.status,
            stdout: Some(stdout),
            stderr: Some(stderr),
        };
    }
}
