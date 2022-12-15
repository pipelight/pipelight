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

#[derive(Debug, Serialize, Deserialize)]
pub enum State {
    Started,
    Succeeded,
    Failed,
    Running,
    Aborted,
    Never,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PipelineLog<'a> {
    pub state: State,
    pub name: &'a str,
    pub step: StepLog<'a>,
}
impl<'a> PipelineLog<'a> {
    pub fn new(name: &'a str, step: &'a str, command: &'a str) -> Self {
        PipelineLog {
            name,
            step: StepLog::new(step, command),
            state: State::Running,
        }
    }
    pub fn state(&mut self, state: &'a State) -> &Self {
        self.state = state;
        self
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StepLog<'a> {
    pub name: &'a str,
    pub command: CommandLog<'a>,
}
impl<'a> StepLog<'a> {
    fn new(name: &'a str, command: &'a str) -> Self {
        StepLog {
            name,
            command: CommandLog::new(command),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CommandLog<'a> {
    pub stdin: &'a str,
    pub stdout: &'a str,
    pub stderr: &'a str,
}
impl<'a> CommandLog<'a> {
    fn new(stdin: &'a str) -> Self {
        CommandLog {
            stdin,
            stdout: "",
            stderr: "",
        }
    }
}
