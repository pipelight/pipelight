// Struct for pipeline execution loggin.
// Pipeline is parsed as json into a log file

#![allow(dead_code)]

// pub mod list;
mod traits;

use exec::types::StrOutput;
use exec::Exec;
use log::{debug, error, info, trace, warn, LevelFilter};
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use std::clone::Clone;
// use std::cmp::PartialEq;
use std::error::Error;
use std::fs;
use std::process;
use std::sync::Mutex;
use utils;
use utils::git::{Git, Hook};
use utils::log::Logs;
use uuid::Uuid;

const STORE: Lazy<Mutex<Config>> = Lazy::new(|| Mutex::new(Config::new()));

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Status {
    Started,
    Succeeded,
    Failed,
    Running,
    Aborted,
    Never,
}

#[derive(Debug, Clone)]
pub struct Config {
    pipelines: Option<Vec<Pipeline>>,
    logs: Option<Logs>,
    hooks: Option<Vec<Hook>>,
}
impl Config {
    pub fn logs(&mut self) {
        self.logs = Some(Logs::new());
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Pipeline {
    pub uuid: Uuid,
    pub pid: Option<u32>,
    pub name: String,
    pub date: Option<String>,
    pub status: Option<Status>,
    pub triggers: Option<Vec<Trigger>>,
    pub steps: Vec<Step>,
}
impl Pipeline {
    pub fn from_log() -> Result<Vec<Pipeline>, Box<dyn Error>> {
        let dir = Logs::new().path;
        let paths = fs::read_dir(dir).unwrap();
        let mut pipelines = vec![];
        for res in paths {
            let dir_entry = res?;
            let json = utils::read_last_line(&dir_entry.path())?;
            let pipeline = serde_json::from_str::<Pipeline>(&json)?;
            pipelines.push(pipeline);
        }
        Ok(pipelines)
    }
    pub fn to_log(&self) {
        let json = serde_json::to_string(&self).unwrap();
        info!(target: "pipeline_json","{}", json);
    }
    pub fn is_running(&mut self) -> bool {
        let pipelines = Pipeline::from_log().unwrap();
        let pipeline = pipelines
            .iter()
            .filter(|p| p.name == self.name)
            .cloned()
            .next();
        let is = pipeline.unwrap().pid.is_some();
        return is;
    }
    pub fn run(&mut self) {
        if self.is_running() {
            return;
        }
        let pid = process::id();
        self.pid = Some(pid);
        let pipeline: &mut Pipeline = self;
        let pipeline_ptr: *mut Pipeline = pipeline;
        Logs::new().set_file(&LevelFilter::Trace, pipeline.uuid);

        unsafe {
            pipeline_ptr.as_mut().unwrap().to_log();
            pipeline_ptr.as_mut().unwrap().status(&Status::Running);
        }
        for step in &mut self.steps {
            step.run(pipeline_ptr);
        }
        unsafe {
            pipeline_ptr.as_mut().unwrap().pid = None;
            pipeline_ptr.as_mut().unwrap().status(&Status::Succeeded);
            pipeline_ptr.as_mut().unwrap().to_log();
        }
    }
    pub fn status(&mut self, status: &Status) {
        self.status = Some(status.to_owned());
    }
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Step {
    pub name: String,
    pub commands: Vec<Command>,
    pub non_blocking: Option<bool>,
    pub on_failure: Option<Vec<String>>,
}
impl Step {
    fn run(&mut self, pipeline_ptr: *mut Pipeline) {
        for command in &mut self.commands {
            command.run(pipeline_ptr);
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Command {
    pub stdin: String,
    output: Option<StrOutput>,
}
impl Command {
    fn new() -> Command {
        return Command {
            stdin: "".to_owned(),
            output: None,
        };
    }
    fn run(&mut self, pipeline_ptr: *mut Pipeline) {
        let output_res = Exec::new().simple(&self.stdin.clone());
        match output_res {
            Ok(output) => {
                self.output = Some(output);
                Ok(())
            }
            Err(e) => {
                unsafe {
                    pipeline_ptr.as_mut().unwrap().status(&Status::Failed);
                }
                Err(e)
            }
        };
        unsafe {
            pipeline_ptr.as_mut().unwrap().to_log();
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Trigger {
    pub action: Option<Hook>,
    pub branch: Option<String>,
}
impl Trigger {
    /// Return actual triggering env
    pub fn env() -> Result<Trigger, Box<dyn Error>> {
        let branch = Git::new().get_branch()?;
        let action = Hook::origin()?;
        Ok(Trigger {
            branch: Some(branch),
            action: Some(action),
        })
    }
}
