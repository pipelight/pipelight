// Struct for pipeline execution loggin.
// PipelineLog is parsed as json into a log file
#![allow(dead_code)]
use crate::exec::shell;
use chrono::{DateTime, Local, NaiveDateTime, Offset, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::{Result, Value};
use std::clone::Clone;
use std::cmp::PartialEq;
use std::convert::From;
use std::error::Error;
use std::marker::Copy;
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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct PipelineLog {
    pub uuid: Uuid,
    pub name: String,
    pub date: Option<String>,
    pub status: PipelineStatus,
    pub trigger: Option<Trigger>,
    pub steps: Vec<StepLog>,
}
impl PipelineLog {
    pub fn run(&self) -> Result<()> {
        for mut step in self.steps.clone() {
            step.run()?;
        }
        Ok(())
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
            date: None,
            name: e.name,
            steps: steps,
            status: PipelineStatus::Started,
            trigger: None,
        };
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct StepLog {
    pub name: String,
    pub commands: Vec<CommandLog>,
    pub non_blocking: Option<bool>,
    pub on_failure: Option<Vec<String>>,
}
impl StepLog {
    fn run(&mut self) -> Result<Self> {
        for mut command in self.commands.clone() {
            command.run();
        }
        Ok(self.to_owned())
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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct CommandLog {
    pub stdin: String,
    pub stdout: Option<String>,
    pub stderr: Option<String>,
}
impl CommandLog {
    fn new() -> Self {
        return CommandLog {
            stdin: "".to_owned(),
            stdout: Some("".to_owned()),
            stderr: Some("".to_owned()),
        };
    }
    fn run(&mut self) -> Result<Self> {
        let output = shell(&self.stdin);
        Ok(self.to_owned())
    }
}
impl From<&String> for CommandLog {
    fn from(s: &String) -> Self {
        CommandLog {
            stdin: s.to_owned(),
            stdout: None,
            stderr: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct StrOutput {
    pub status: ExitStatus,
    pub stdout: String,
    pub stderr: String,
}
impl From<&Output> for StrOutput {
    fn from(s: &Output) -> Self {
        let stdout = String::from_utf8(s.clone().stdout).unwrap().to_owned();
        let stderr = String::from_utf8(s.clone().stderr).unwrap().to_owned();
        return StrOutput {
            status: s.status,
            stdout: stdout,
            stderr: stderr,
        };
    }
}
