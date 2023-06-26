// Struct for pipeline execution loggin.
// Pipeline is parsed as json into a log file

// Internal imports
use super::traits::Getters;

use super::types::{Logs, Mode, Pipeline, StepOrParallel, Trigger, TriggerBranch, TriggerTag};
// Error Handling
use miette::{IntoDiagnostic, Result};

// Standard libs
use log::info;
use std::time::Duration;

//sys
use rustix::process::{kill_process_group, test_kill_process, Pid, Signal};

// External imports
use exec::Status;
use utils;
use utils::git::{Flag, Git, Hook};
use utils::logger::logger;

impl Pipeline {
    pub fn log(&self) {
        logger.lock().unwrap().file(&self.uuid);
        let json = serde_json::to_string(&self).unwrap();
        info!(target: "pipeline_json","{}", json);
    }
    /// Compares if log_pid is in system pid list.
    /// If not, the program has been aborted
    pub fn is_aborted(&mut self) -> bool {
        if self.event.is_some()
            && (self.status == Some(Status::Running) || self.status == Some(Status::Running))
        {
            unsafe {
                let pid = Pid::from_raw(self.event.clone().unwrap().pid.unwrap());
                match test_kill_process(pid.unwrap()) {
                    Ok(_) => return false,
                    Err(_) => return true,
                }
            }
        } else {
            return false;
        }
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
                            unsafe {
                                let pid = Pid::from_raw(pid.unwrap());
                                match test_kill_process(pid.unwrap()) {
                                    Ok(_) => return true,
                                    Err(_) => return false,
                                }
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
        if self.event.is_some() && self.status == Some(Status::Running) {
            let pid = self.clone().event.unwrap().pid.unwrap();
            unsafe {
                let pgid_raw = self.event.clone().unwrap().pgid.unwrap();
                let pgid = Pid::from_raw(pgid_raw).unwrap();
                kill_process_group(pgid, Signal::Term)
                    .into_diagnostic()
                    .unwrap();
            }
            self.status = Some(Status::Aborted);
            self.log();
        }
    }
}

impl StepOrParallel {
    pub fn mode(&self) -> Option<Mode> {
        match self {
            StepOrParallel::Step(res) => res.mode.clone(),
            StepOrParallel::Parallel(res) => res.mode.clone(),
        }
    }
    pub fn duration(&self) -> Option<Duration> {
        match self {
            StepOrParallel::Step(res) => res.duration,
            StepOrParallel::Parallel(res) => res.duration,
        }
    }
}
impl Trigger {
    /// Return actual triggering env
    pub fn env() -> Result<Trigger> {
        let mut branch = None;
        let mut tag = None;
        let res;

        let action = Some(Hook::origin()?);

        if Git::new().exists() {
            branch = Git::new().get_branch()?;
            tag = Git::new().get_tag()?;
        }
        if tag.is_some() {
            res = Trigger::TriggerTag(TriggerTag {
                action: action.clone(),
                tag: tag,
            });
            Ok(res)
        } else if branch.is_some() {
            res = Trigger::TriggerBranch(TriggerBranch {
                action: action.clone(),
                branch: branch,
            });
            Ok(res)
        } else {
            res = Trigger::TriggerBranch(TriggerBranch {
                action: action.clone(),
                branch: None,
            });
            Ok(res)
            // let message = "Couldn't get pipeline triggering environment";
            // return Err(Error::msg(message));
        }
    }
    pub fn set_action(&mut self, flag: Option<String>) -> Self {
        let flag = Some(Flag::from(&flag.unwrap()));
        match self {
            Trigger::TriggerBranch(res) => res.action = flag,
            Trigger::TriggerTag(res) => res.action = flag,
        }
        return self.to_owned();
    }
    pub fn action(&self) -> Option<Flag> {
        match self {
            Trigger::TriggerBranch(res) => res.action.clone(),
            Trigger::TriggerTag(res) => res.action.clone(),
        }
    }
    pub fn branch(&self) -> Option<String> {
        match self {
            Trigger::TriggerBranch(res) => res.branch.clone(),
            Trigger::TriggerTag(..) => None,
        }
    }
    pub fn tag(&self) -> Option<String> {
        match self {
            Trigger::TriggerTag(res) => res.tag.clone(),
            Trigger::TriggerBranch(..) => None,
        }
    }
}
