// Struct for pipeline execution loggin.
// Pipeline is parsed as json into a log file

#![allow(dead_code)]

// Internal imports
pub mod characters;
mod logs;
mod run;
pub mod traits;
mod tree;
use crate::types::traits::getters::Getters;

// Standard libs
use log::LevelFilter;
use log::{info, warn};
use serde::{Deserialize, Serialize};
use std::clone::Clone;
use std::error::Error;
use std::fs;
use std::path::Path;
use std::process;
use std::time::{Duration, Instant};
use sysinfo::{Pid, PidExt, ProcessExt, System, SystemExt};
use uuid::Uuid;

// External imports
use exec::types::{Status, StrOutput};
use exec::Exec;
use utils;
use utils::git::{Flag, Git, Hook};
use utils::logger::logger;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Fallback {
    pub on_failure: Option<Vec<StepOrParallel>>,
    pub on_success: Option<Vec<StepOrParallel>>,
    pub on_abortion: Option<Vec<StepOrParallel>>,
}
#[derive(Debug, Clone)]
pub struct Config {
    pub pipelines: Option<Vec<Pipeline>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    value: Option<String>,
    duration: Option<Duration>,
    status: Option<Status>,
    children: Option<Vec<Node>>,
    level: LevelFilter,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Pipeline {
    pub uuid: Uuid,
    pub name: String,
    pub event: Option<Event>,
    pub status: Option<Status>,
    pub duration: Option<Duration>,
    pub triggers: Option<Vec<Trigger>>,
    pub fallback: Option<Fallback>,
    pub steps: Vec<StepOrParallel>,
}

impl Pipeline {
    pub fn log(&self) {
        logger.lock().unwrap().file(&self.uuid);
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
        } else {
            let res = Logs::get_many_by_name(&self.name);
            match res {
                Ok(pipelines) => {
                    let mut p = pipelines.clone();
                    p.reverse();
                    let pipeline = p.iter().next();
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
                }
                Err(e) => {
                    return false;
                }
            }
            return false;
        }
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
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum StepOrParallel {
    Step(Step),
    Parallel(Parallel),
}
impl StepOrParallel {
    pub fn non_blocking(&self) -> Option<bool> {
        match self {
            StepOrParallel::Step(res) => res.non_blocking,
            StepOrParallel::Parallel(res) => res.non_blocking,
        }
    }
    pub fn duration(&self) -> Option<Duration> {
        match self {
            StepOrParallel::Step(res) => res.duration,
            StepOrParallel::Parallel(res) => res.duration,
        }
    }
}
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Parallel {
    pub status: Option<Status>,
    pub duration: Option<Duration>,
    pub steps: Vec<Step>,
    pub non_blocking: Option<bool>,
    pub fallback: Option<Fallback>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Step {
    pub name: String,
    pub status: Option<Status>,
    pub duration: Option<Duration>,
    pub commands: Vec<Command>,
    pub non_blocking: Option<bool>,
    pub fallback: Option<Fallback>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Command {
    pub status: Option<Status>,
    pub duration: Option<Duration>,
    pub stdin: String,
    pub output: Option<StrOutput>,
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
