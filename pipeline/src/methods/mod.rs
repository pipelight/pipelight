// Struct for pipeline execution loggin.
// Pipeline is parsed as json into a log file

// Internal imports
use super::traits::Getters;
use crate::{Config, Duration, Logs, Mode, Pipeline, StepOrParallel, Trigger};

// Error Handling
use miette::{Error, IntoDiagnostic, Result};

//Duration
use chrono::{DateTime, Local};

// Standard libs
use log::info;

//sys
use rustix::process::{kill_process_group, test_kill_process, Pid, Signal};

// Globbing
use glob::Pattern;

// External imports
use exec::{Statuable, Status};
use utils::git::{Flag, Git, Special};
use utils::logger::logger;

// Tests
mod test;

impl Config {
    pub fn has_watch_flag(&self) -> Result<()> {
        for pipeline in self.pipelines.clone().unwrap() {
            if pipeline.is_watchable().is_ok() {
                return Ok(());
            }
        }
        let message = "no watchable pipelines";
        Err(Error::msg(message))
    }
}

impl Pipeline {
    pub fn log(&self) {
        logger.lock().unwrap().file(&self.uuid);
        let json = serde_json::to_string(&self).unwrap();
        info!(target: "pipeline_json","{}", json);
    }
    // Add process stdout/stderr to runnnig pipeline logs.
    // Concurent std read/write while command is running
    pub fn hydrate(&mut self) {
        for step_or_parallel in &mut self.steps {
            match step_or_parallel {
                StepOrParallel::Step(step) => {
                    for command in &mut step.commands {
                        if command.get_status() == Some(Status::Running) {
                            command.process.read();
                        }
                    }
                }
                StepOrParallel::Parallel(parallel) => {
                    for step in &mut parallel.steps {
                        for command in &mut step.commands {
                            if command.get_status() == Some(Status::Running) {
                                command.process.read();
                            }
                        }
                    }
                }
            }
        }
    }
    /// Verify if pipeline can be triggered
    pub fn is_triggerable(&self) -> Result<bool> {
        let env = Trigger::flag(None)?;

        // If in git repo
        if Git::new().exists() {
            if self.triggers.is_some() {
                Ok(env.is_match(self.triggers.clone().unwrap()).is_ok())
            } else {
                Ok(true)
            }
        } else {
            Ok(true)
        }
    }
    /// Verify if pipeline can be watched
    pub fn is_watchable(&self) -> Result<()> {
        if self.triggers.is_some() {
            for trigger in self.triggers.clone().unwrap() {
                if trigger.action == Some(Flag::Special(Special::Watch)) {
                    return Ok(());
                }
            }
        }
        let message = "no watchable pipelines";
        Err(Error::msg(message))
    }
    /// Compares if log_pid is in system pid list.
    /// If not, the program has been aborted
    pub fn is_aborted(&mut self) -> bool {
        if self.event.is_some()
            && (self.status == Some(Status::Running) || self.status == Some(Status::Running))
        {
            unsafe {
                let pid = Pid::from_raw(self.event.clone().unwrap().pid.unwrap());
                test_kill_process(pid.unwrap()).is_err()
            }
        } else {
            false
        }
    }
    /// If the pid (extracted from logs) exists it means the pipeline is running
    /// (improvement: need to add process name comparision to harden func)
    pub fn is_running(&mut self) -> bool {
        if Logs::get().is_err() {
            false
        } else {
            let res = Logs::get_many_by_name(&self.name);
            match res {
                Ok(pipelines) => {
                    let mut p = pipelines;
                    p.reverse();
                    let pipeline = p.first();
                    if let Some(pipeline) = pipeline {
                        let event = &pipeline.event;
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
                Err(e) => return false,
            }
            false
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
}
impl Trigger {
    // Success if trigger has same action or None
    pub fn is_action_match(&self, trigger: Trigger) -> Result<()> {
        if trigger.action.is_none() {
            Ok(())
        } else if trigger.action == self.action {
            Ok(())
        } else {
            let message = "no action match";
            Err(Error::msg(message))
        }
    }
    pub fn is_branch_match(&self, trigger: Trigger) -> Result<()> {
        if trigger.branch.is_none() {
            return Ok(());
        } else {
            // Globbing pattern matching
            let glob = Pattern::new(&trigger.branch.unwrap()).into_diagnostic()?;
            let glob_match = glob.matches(&self.clone().branch.unwrap());
            if glob_match {
                return Ok(());
            } else {
                let message = "no branch match";
                return Err(Error::msg(message));
            }
        }
    }
    pub fn is_tag_match(&self, trigger: Trigger) -> Result<()> {
        if trigger.tag.is_none() || self.tag.is_none() {
            return Ok(());
        } else {
            // Globbing pattern matching
            let glob = Pattern::new(&trigger.tag.unwrap()).into_diagnostic()?;
            let glob_match = glob.matches(&self.clone().tag.unwrap());
            if glob_match {
                return Ok(());
            } else {
                let message = "no tag match";
                return Err(Error::msg(message));
            }
        }
    }
    pub fn is_match(&self, list: Vec<Trigger>) -> Result<()> {
        for trigger in list {
            let binding = trigger.clone();
            if self.is_action_match(binding.clone()).is_ok()
                && self.is_branch_match(binding.clone()).is_ok()
                && self.is_tag_match(binding).is_ok()
            {
                return Ok(());
            }
        }
        let message = "no match";
        return Err(Error::msg(message));
    }
}

pub fn std_duration_to_iso8601(duration: std::time::Duration) -> Result<String> {
    let chrono_duration = chrono::Duration::from_std(duration).ok();
    if let Some(chrono_duration) = chrono_duration {
        let duration_ISO_8601 = format!("{}", chrono_duration);
        Ok(duration_ISO_8601)
    } else {
        Err(Error::msg("Bad std::Duration instance"))
    }
}
pub fn iso8601_to_std_duration(duration: String) -> Result<std::time::Duration> {
    let duration = &duration.as_str();
    let chrono_duration: Option<iso8601_duration::Duration> = duration.parse().ok();
    if chrono_duration.is_some() {
        Ok(chrono_duration.unwrap().to_std().unwrap())
    } else {
        Err(Error::msg("Couldn't parse duration: Bad iso8601 duration"))
    }
}
pub fn compute_duration(duration: Duration) -> Result<std::time::Duration> {
    let computed_duration: Option<Duration> = None;
    let now = Local::now();

    let date = duration
        .started_at
        .clone()
        .unwrap()
        .parse::<DateTime<Local>>()
        .unwrap();

    let diff: chrono::Duration = now - date;
    let duration = diff.to_std().unwrap();

    Ok(duration)
}
