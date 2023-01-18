// Struct for pipeline execution loggin.
// Pipeline is parsed as json into a log file

#![allow(dead_code)]

// Internal imports
mod traits;

// Standard libs
use log::info;
use serde::{Deserialize, Serialize};
use std::clone::Clone;
use std::error::Error;
use std::fs;
use std::path::Path;
use std::process;
use sysinfo::{PidExt, System, SystemExt};
use uuid::Uuid;

// External imports
use exec::types::StrOutput;
use exec::Exec;
use utils;
use utils::git::{Flag, Git, Hook};
use utils::logger::logger;

//Store
use once_cell::sync::Lazy;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Status {
    Started,
    Succeeded,
    Failed,
    Running,
    Aborted,
}

#[derive(Debug, Clone)]
pub struct Config {
    pub pipelines: Option<Vec<Pipeline>>,
    pub hooks: Option<Vec<Hook>>,
}

struct Store {
    pipeline: Pipeline,
}

static store: Lazy<Store> = Lazy::new(|| {
    let defaultPipeline = Pipeline::new();
    return Store {
        pipeline: defaultPipeline,
    };
});

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Pipeline {
    pub uuid: Uuid,
    pub event: Option<Event>,
    pub name: String,
    pub status: Option<Status>,
    pub triggers: Option<Vec<Trigger>>,
    pub steps: Vec<Step>,
}
impl Pipeline {
    pub fn log(&self) {
        logger.file(&self.uuid);
        let json = serde_json::to_string(&self).unwrap();
        info!(target: "pipeline_json","{}", json);
    }
    /// Compares if log_pid is in system pid list.
    /// If not, the program has been aborted
    pub fn is_aborted(&mut self) -> bool {
        if self.event.is_none() {
            return false;
        }
        if self.clone().event.unwrap().pid.is_none() {
            return false;
        }
        let mut sys = System::new_all();
        sys.refresh_all();
        return !sys
            .process(PidExt::from_u32(self.clone().event.unwrap().pid.unwrap()))
            .is_some();
    }
    /// If the log_pid exists it means the program runs
    /// (need to add process name comparision to harden func)
    pub fn is_running(&mut self) -> bool {
        let pipelines = Logs::get_by_name(&self.name).unwrap();
        let pipeline = pipelines.iter().next();
        // info!("{:?}", pipelines);

        if pipeline.is_none() {
            return false;
        }
        if pipeline.clone().unwrap().event.is_none() {
            return false;
        }
        let mut sys = System::new_all();
        sys.refresh_all();
        return sys
            .process(PidExt::from_u32(
                pipeline.unwrap().clone().event.unwrap().pid.unwrap(),
            ))
            .is_some();
    }
    pub fn run(&mut self) {
        if self.is_running() {
            return;
        }
        let pid = process::id();
        self.event.as_mut().unwrap().pid = Some(pid);

        // Store
        // store.pipeline = self.to_owned();

        let pipeline: &mut Pipeline = self;
        let pipeline_ptr: *mut Pipeline = pipeline;

        unsafe {
            pipeline_ptr.as_mut().unwrap().status(&Status::Running);
            pipeline_ptr.as_mut().unwrap().log();
        }
        for step in &mut self.steps {
            step.run(pipeline_ptr);
        }
        unsafe {
            pipeline_ptr.as_mut().unwrap().event.as_mut().unwrap().pid = None;
            pipeline_ptr.as_mut().unwrap().status(&Status::Succeeded);
            pipeline_ptr.as_mut().unwrap().log();
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
            pipeline_ptr.as_mut().unwrap().log();
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Trigger {
    pub action: Option<Flag>,
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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Event {
    pub trigger: Trigger,
    pub date: String,
    pub pid: Option<u32>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, PartialOrd, Eq, Ord)]
pub struct Logs;

impl Logs {
    /// Return pipelines from log files
    pub fn get() -> Result<Vec<Pipeline>, Box<dyn Error>> {
        let dir = &logger.directory;
        // Safe exit if no log folder
        if !Path::new(dir).exists() {
            let message = "No log can be displayed. Log folder is empty";
            return Err(Box::from(message));
        }
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
    pub fn get_by_name(name: &String) -> Result<Vec<Pipeline>, Box<dyn Error>> {
        let pipelines = Logs::get()?;
        let mut pipelines = pipelines
            .iter()
            .filter(|p| &p.name == name)
            .cloned()
            .collect::<Vec<Pipeline>>();
        // Sort by date descending
        pipelines.sort_by_key(|e| e.clone().event.unwrap().date);
        pipelines.reverse();
        Ok(pipelines)
    }
}
