// Struct for pipeline execution loggin.
// PipelineLog is parsed as json into a log file
#![allow(dead_code)]
use crate::exec::subprocess::exec;
use crate::logger::set_logger_config_pipeline;
use chrono::{DateTime, Local, NaiveDateTime, Offset, TimeZone, Utc};
pub use log::Level::{Debug, Trace};
pub use log::{debug, error, info, trace, warn, LevelFilter, SetLoggerError};
use log4rs::Handle;
use once_cell::sync::Lazy;
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

#[derive(Debug, PartialEq)]
struct Observer {
    pipeline: Option<PipelineLog>,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum PipelineStatus {
    Started,
    Succeeded,
    Failed,
    Running,
    Aborted,
    Never,
}

#[derive(Debug, Serialize, Deserialize Clone, PartialEq)]
pub struct PipelineLog {
    pub uuid: Uuid,
    pub name: String,
    pub date: Option<String>,
    pub status: PipelineStatus,
    pub trigger: Option<Trigger>,
    pub steps: Vec<StepLog>,
}
impl PipelineLog {
    pub fn run(&mut self, handle: Handle) {
        let pipeline: &mut PipelineLog = self;
        let pipeline_ptr: *mut PipelineLog = pipeline;
        handle.set_config(set_logger_config_pipeline(LevelFilter::Trace, pipeline.uuid).unwrap());
        for step in &mut self.steps {
            step.run(pipeline_ptr);
        }
    }
    pub fn log(&self) {
        let json = serde_json::to_string(&self).unwrap();
        info!(target: "pipeline_json","{:?}", json);
    }
}

impl From<Pipeline> for PipelineLog {
    fn from(e: Pipeline) -> Self {
        let steps = e
            .steps
            .iter()
            .map(|e| StepLog::from(e))
            .collect::<Vec<StepLog>>();
        let p = PipelineLog {
            uuid: Uuid::new_v4(),
            date: Some(Utc::now().to_string()),
            name: e.name,
            steps: steps,
            status: PipelineStatus::Started,
            trigger: None,
        };
        return p;
    }
}

#[derive(Debug, Serialize, Deserialize Clone, PartialEq)]
pub struct StepLog {
    pub name: String,
    pub commands: Vec<CommandLog>,
    pub non_blocking: Option<bool>,
    pub on_failure: Option<Vec<String>>,
}
impl StepLog {
    fn run(&mut self, pipeline: *mut PipelineLog) {
        for command in &mut self.commands {
            command.run(pipeline);
        }
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

#[derive(Debug, Serialize, Deserialize Clone, PartialEq)]
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
    fn run(&mut self, pipeline: *mut PipelineLog) {
        let output = exec(&self.stdin.clone()).ok();
        self.output = output;
        unsafe {
            pipeline.as_ref().unwrap().log();
        }
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

#[derive(Debug, Serialize, Deserialize Clone, PartialEq)]
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
