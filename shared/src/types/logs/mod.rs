// Struct for pipeline execution loggin.
// PipelineLog is parsed as json into a log file
#![allow(dead_code)]
use crate::exec::subprocess::exec;
use crate::logger;
use chrono;
use chrono::{format, DateTime, Local, NaiveDateTime, Offset, TimeZone, Utc};
use colored::Colorize;
pub use log::Level::{Debug, Trace};
pub use log::{debug, error, info, trace, warn, LevelFilter, SetLoggerError};
use log4rs::Handle;
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::{Result, Value};
use std::clone::Clone;
use std::cmp::PartialEq;
use std::convert::From;
use std::error::Error;
use std::fmt;
use std::marker::Copy;
use std::process;
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
impl fmt::Display for PipelineStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let icon = "●";
        match *self {
            PipelineStatus::Started => write!(f, "{} Started", icon),
            PipelineStatus::Succeeded => write!(f, "{} Succeeded", icon.blue()),
            PipelineStatus::Failed => write!(f, "{} Failed", icon.red()),
            PipelineStatus::Running => write!(f, "{} Running", icon.green()),
            PipelineStatus::Aborted => write!(f, "{} Aborted", icon.yellow()),
            PipelineStatus::Never => write!(f, "{} Never", icon),
        };
        Ok(())
    }
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
impl fmt::Display for PipelineLog {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} - ", &self.status);
        let str_date = &self.date.as_ref().unwrap();
        let date = str_date.parse::<DateTime<Local>>().unwrap();
        // let date: &str = &binding.as_ref();
        write!(f, "{}\n", date.to_rfc2822());
        write!(f, "   pipeline: {}\n", self.name);
        if self.pid.is_some() {
            write!(f, "   pid: {}\n", &self.pid.unwrap());
        }
        for step in &self.steps {
            info!(target :"nude","\tstep: {}\n", step.name);
            for command in &step.commands {
                let stdout = command.output.as_ref().unwrap().stdout.as_ref().unwrap();
                let stderr = command.output.as_ref().unwrap().stderr.as_ref().unwrap();
                let status = command.output.as_ref().unwrap().status;
                if status {
                    info!(target: "nude", "\t\t{}\n", &command.stdin.green());
                    debug!(target: "nude", "{}\n", stdout)
                } else {
                    info!(target: "nude", "\t\t{}\n", &command.stdin.red());
                    debug!(target: "nude", "\r{}\n", stderr);
                }
            }
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
        let output_res = exec(&self.stdin.clone());
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
