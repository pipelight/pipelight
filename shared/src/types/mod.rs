#![allow(dead_code)]
use serde::{Deserialize, Serialize};
use serde_json::json;
use serde_json::{Result, Value};
use std::clone::Clone;
use std::fmt;
use std::marker::Copy;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub pipelines: Vec<Pipeline>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Pipeline {
    pub name: String,
    pub steps: Vec<Step>,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Step {
    pub name: String,
    pub commands: Vec<String>,
    pub on_failure: Option<Vec<String>>,
}
pub fn type_of<T>(_: &T) -> String {
    let res = format!("{}", std::any::type_name::<T>());
    return res;
}

pub struct Path {
    pub folder: String,
    pub file: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum PipelineState {
    Started,
    Succeeded,
    Failed,
    Running,
    Aborted,
    Never,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PipelineLog<'a> {
    pub state: PipelineState,
    pub name: &'a str,
    pub step: Option<StepLog<'a>>,
}
impl<'a> PipelineLog<'a> {
    pub fn new(name: &'a str) -> Self {
        PipelineLog {
            name,
            step: Default::default(),
            state: PipelineState::Started,
        }
    }
    pub fn state(&mut self, state: PipelineState) -> &Self {
        self.state = state;
        return self;
    }
    pub fn step(&mut self, step: &'a str) -> &Self {
        self.step = Some(StepLog::new(step).to_owned());
        return self;
    }
    pub fn command(&mut self, cmd: &'a str, stdout: &'a str) -> &Self {
        self.step.as_mut().unwrap().command = CommandLog::new().to_owned();
        self.step.as_mut().unwrap().command.stdin = cmd;
        self.step.as_mut().unwrap().command.stdout = stdout;
        return self;
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StepLog<'a> {
    pub name: &'a str,
    pub command: CommandLog<'a>,
}
impl<'a> StepLog<'a> {
    pub fn new(name: &'a str) -> Self {
        StepLog {
            name,
            command: CommandLog::new(),
        }
    }
    pub fn name(&mut self, name: &'a str) -> &Self {
        self.name = name;
        return self;
    }
    pub fn command(&mut self, command: &'a str) -> &mut Self {
        self.command = CommandLog::new();
        return self;
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CommandLog<'a> {
    pub stdin: &'a str,
    pub stdout: &'a str,
    pub stderr: &'a str,
}
impl<'a> CommandLog<'a> {
    fn new() -> Self {
        CommandLog {
            stdin: "",
            stdout: "",
            stderr: "",
        }
    }
    pub fn stdin(&mut self, cmd: &'a str) -> &mut Self {
        self.stdin = cmd;
        return self;
    }
}
