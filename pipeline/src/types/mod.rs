// Struct for pipeline execution loggin.
// Pipeline is parsed as json into a log file

#![allow(dead_code)]

// Internal imports
mod traits;

// Standard libs
use log::{info, warn};
use serde::{Deserialize, Serialize};
use std::clone::Clone;
use std::error::Error;
use std::fs;
use std::path::Path;
use std::process;
use sysinfo::{Pid, PidExt, ProcessExt, System, SystemExt};
use uuid::Uuid;

// External imports
use exec::types::{Status, StrOutput};
use exec::Exec;
use utils;
use utils::git::{Flag, Git, Hook};
use utils::logger::logger;

// Duraion
use std::time::{Duration, Instant};

#[derive(Debug, Clone)]
pub struct Config {
    pub pipelines: Option<Vec<Pipeline>>,
    pub hooks: Option<Vec<Hook>>,
}

struct Store {
    pipeline: Pipeline,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Pipeline {
    pub uuid: Uuid,
    pub event: Option<Event>,
    pub duration: Option<Duration>,
    pub name: String,
    pub status: Option<Status>,
    pub triggers: Option<Vec<Trigger>>,
    pub steps: Vec<Step>,
}
impl Pipeline {
    pub fn log(&self) {
        logger.load().file(&self.uuid);
        let json = serde_json::to_string(&self).unwrap();
        info!(target: "pipeline_json","{}", json);
    }
    /// Compares if log_pid is in system pid list.
    /// If not, the program has been aborted
    pub fn is_aborted(&mut self) -> bool {
        if self.event.is_none() {
            return false;
        }
        // if self.clone().event.unwrap().pid.is_none() {
        if self.clone().status.is_none() {
            return false;
        }
        let mut sys = System::new_all();
        sys.refresh_all();
        return !sys
            .process(PidExt::from_u32(self.clone().event.unwrap().pid.unwrap()))
            .is_some();
    }
    /// If the pid (extracted from logs) exists it means the pipeline is running
    /// (improvement: need to add process name comparision to harden func)
    pub fn is_running(&mut self) -> bool {
        if Logs::get().is_err() {
            return false;
        }
        let pipelines = Logs::get_by_name(&self.name).unwrap();
        let pipeline = pipelines.iter().next();
        if pipeline.is_some() {
            let event = &pipeline.clone().unwrap().event;
            if event.is_some() {
                let pid = &event.clone().unwrap().pid;
                if pid.is_some() {
                    let mut sys = System::new_all();
                    sys.refresh_all();
                    return sys.process(PidExt::from_u32(pid.unwrap())).is_some();
                }
            }
        }
        return false;
    }
    /// Abort process execution
    pub fn stop(&mut self) {
        if self.event.is_some() {
            if self.event.clone().unwrap().pid.is_some() {
                let pid = self.clone().event.unwrap().pid.unwrap();
                let mut sys = System::new_all();
                sys.refresh_all();
                let process = sys.process(PidExt::from_u32(pid));
                if process.clone().is_some() {
                    process.unwrap().kill();
                    self.status = Some(Status::Aborted);
                    self.log();
                }
            }
        }
    }
    /// Execute the pipeline
    pub fn run(&mut self) {
        // Duration
        let start = Instant::now();
        let duration = start.elapsed();

        // Globals
        let pipeline: &mut Pipeline = self;
        let pipeline_ptr: *mut Pipeline = pipeline;

        if pipeline.is_running() {
            return;
        }

        // Set Pid and Status and Duration
        pipeline.event = Some(Event::new());
        pipeline.status(&Status::Running);
        pipeline.duration = Some(duration);
        pipeline.log();

        for step in &mut pipeline.steps {
            step.run(pipeline_ptr);
        }

        // Set pipeline status to last Step status
        let last_step = pipeline.steps.last().unwrap();
        if last_step.status.is_some() {
            pipeline.status(&last_step.status.clone().unwrap())
        } else {
            pipeline.status(&Status::Failed)
        }
        pipeline.log();
    }
    pub fn status(&mut self, status: &Status) {
        self.status = Some(status.to_owned());
    }
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Step {
    pub status: Option<Status>,
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

        let optional_output = &self.commands.last().unwrap().output;
        if optional_output.is_some() {
            self.status(&optional_output.clone().unwrap().status)
        } else {
            self.status(&Status::Failed)
        }
    }
    pub fn status(&mut self, status: &Status) {
        self.status = Some(status.to_owned());
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Command {
    pub stdin: String,
    pub output: Option<StrOutput>,
}
impl Command {
    fn new() -> Command {
        return Command {
            stdin: "".to_owned(),
            output: None,
        };
    }
    fn run(&mut self, pipeline_ptr: *mut Pipeline) {
        // Duration
        let start = Instant::now();
        let mut duration = start.elapsed();
        // unsafe {
        //     pipeline_ptr.as_mut().unwrap().duration.unwrap() + duration;
        // }

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
        duration = start.elapsed();
        unsafe {
            pipeline_ptr.as_mut().unwrap().duration.unwrap() + duration;
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
        let mut branch = None;
        if Git::new().exists() {
            branch = Some(Git::new().get_branch()?);
        }
        let action = Some(Hook::origin()?);
        Ok(Trigger {
            branch: branch,
            action: action,
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
    fn sanitize(pipelines: Vec<Pipeline>) -> Result<Vec<Pipeline>, Box<dyn Error>> {
        let message = "Sanitizing log files";
        info!("{}", message);
        for mut pipeline in pipelines.clone() {
            if pipeline.is_aborted() {
                pipeline.status = Some(Status::Aborted);
                pipeline.log();
            }
        }
        Ok(pipelines.to_owned())
    }
    /// Return pipelines from log files
    pub fn get() -> Result<Vec<Pipeline>, Box<dyn Error>> {
        let dir = &logger.load().directory;
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
        // pipelines = Logs::sanitize(pipelines)?;
        Ok(pipelines)
    }
    pub fn get_by_name(name: &String) -> Result<Vec<Pipeline>, Box<dyn Error>> {
        let pipelines = Logs::get()?;
        let mut pipelines = pipelines
            .iter()
            .filter(|p| &p.name == name)
            .cloned()
            .collect::<Vec<Pipeline>>();
        pipelines.sort_by_key(|e| e.clone().event.unwrap().date);

        if pipelines.is_empty() {
            warn!("Couldn't find a pipeline named {:?}", name);
        }
        Ok(pipelines)
    }
}
