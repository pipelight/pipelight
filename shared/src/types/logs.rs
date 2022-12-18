// Struct for pipeline execution loggin.
// PipelineLog is parsed as json into a log file

#![allow(dead_code)]
use chrono::{DateTime, Local, NaiveDateTime, Offset, TimeZone, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::{Result, Value};
use std::clone::Clone;
use uuid::{uuid, Uuid};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum PipelineStatus {
    Started,
    Succeeded,
    Failed,
    Running,
    Aborted,
    Never,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PipelineLog {
    pub status: PipelineStatus,
    pub date: String,
    pub uuid: Uuid,
    pub name: String,
    pub step: Option<StepLog>,
}
impl PipelineLog {
    pub fn new<'a>(name: &'a str) -> Self {
        PipelineLog {
            name: name.to_owned(),
            uuid: Uuid::new_v4(),
            date: Utc::now().to_string(),
            step: Default::default(),
            status: PipelineStatus::Started,
        }
    }
    pub fn status(&mut self, status: PipelineStatus) -> &Self {
        self.status = status;
        return self;
    }
    pub fn uuid(&mut self, uuid: Uuid) -> &Self {
        self.uuid = uuid;
        return self;
    }
    pub fn step<'a>(&mut self, step: &'a str) -> &Self {
        self.step = Some(StepLog::new(step).to_owned());
        return self;
    }
    pub fn command<'a>(&mut self, cmd: &'a str, stdout: &'a str) -> &Self {
        self.step.as_mut().unwrap().command = CommandLog::new().to_owned();
        self.step.as_mut().unwrap().command.stdin = cmd.to_owned();
        self.step.as_mut().unwrap().command.stdout = stdout.to_owned();
        return self;
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StepLog {
    pub name: String,
    pub command: CommandLog,
}
impl StepLog {
    pub fn new<'a>(name: &'a str) -> Self {
        StepLog {
            name: name.to_owned(),
            command: CommandLog::new(),
        }
    }
    pub fn name<'a>(&mut self, name: &'a str) -> &Self {
        self.name = name.to_owned();
        return self;
    }
    pub fn command<'a>(&mut self, command: &'a str) -> &mut Self {
        self.command = CommandLog::new();
        return self;
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CommandLog {
    pub stdin: String,
    pub stdout: String,
    pub stderr: String,
}
impl CommandLog {
    fn new() -> Self {
        CommandLog {
            stdin: "".to_owned(),
            stdout: "".to_owned(),
            stderr: "".to_owned(),
        }
    }
    pub fn stdin(&mut self, cmd: String) -> &mut Self {
        self.stdin = cmd;
        return self;
    }
}
