// Struct for pipeline execution loggin.
// PipelineLog is parsed as json into a log file
#![allow(dead_code)]
use crate::exec::Exec;
use crate::logger;
use chrono::Utc;
mod display;
pub use log::Level::{Debug, Trace};
pub use log::{debug, error, info, trace, warn, LevelFilter, SetLoggerError};
use log4rs::Handle;
use serde::{Deserialize, Serialize};
use std::clone::Clone;
use std::cmp::PartialEq;
use std::convert::From;
use std::marker::Copy;
use std::process;
use std::process::Output;
use uuid::Uuid;

use crate::types::{Pipeline, Step, Trigger};

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
    pub pid: Option<u32>,
    pub name: String,
    pub date: Option<String>,
    pub status: PipelineStatus,
    pub trigger: Option<Trigger>,
    pub steps: Vec<StepLog>,
}
impl PipelineLog {
    pub fn run(&mut self, handle: Handle) {
        let pid = process::id();
        self.pid = Some(pid);
        let pipeline: &mut PipelineLog = self;
        let pipeline_ptr: *mut PipelineLog = pipeline;
        handle
            .set_config(logger::config::set_with_file(LevelFilter::Trace, pipeline.uuid).unwrap());

        unsafe {
            pipeline_ptr.as_mut().unwrap().log();
            pipeline_ptr
                .as_mut()
                .unwrap()
                .status(&PipelineStatus::Running);
        }
        for step in &mut self.steps {
            step.run(pipeline_ptr);
        }
        unsafe {
            pipeline_ptr.as_mut().unwrap().pid = None;
            pipeline_ptr
                .as_mut()
                .unwrap()
                .status(&PipelineStatus::Succeeded);
            pipeline_ptr.as_mut().unwrap().log();
        }
    }
    pub fn log(&self) {
        let json = serde_json::to_string(&self).unwrap();
        info!(target: "pipeline_json","{}", json);
    }
    pub fn status(&mut self, status: &PipelineStatus) {
        self.status = status.to_owned();
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
            pid: None,
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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct StepLog {
    pub name: String,
    pub commands: Vec<CommandLog>,
    pub non_blocking: Option<bool>,
    pub on_failure: Option<Vec<String>>,
}
impl StepLog {
    fn run(&mut self, pipeline_ptr: *mut PipelineLog) {
        for command in &mut self.commands {
            command.run(pipeline_ptr);
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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
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
    fn run(&mut self, pipeline_ptr: *mut PipelineLog) {
        let output_res = Exec::new().simple(&self.stdin.clone());
        match output_res {
            Ok(output) => {
                self.output = Some(output);
                Ok(())
            }
            Err(e) => {
                unsafe {
                    pipeline_ptr
                        .as_mut()
                        .unwrap()
                        .status(&PipelineStatus::Failed);
                }
                Err(e)
            }
        };
        unsafe {
            pipeline_ptr.as_mut().unwrap().log();
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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct StrOutput {
    pub status: bool,
    pub stdout: Option<String>,
    pub stderr: Option<String>,
}
impl From<&Output> for StrOutput {
    fn from(s: &Output) -> Self {
        let stdout = String::from_utf8(s.clone().stdout).unwrap().to_owned();
        let stderr = String::from_utf8(s.clone().stderr).unwrap().to_owned();
        return StrOutput {
            status: s.status.success(),
            stdout: Some(stdout),
            stderr: Some(stderr),
        };
    }
}
