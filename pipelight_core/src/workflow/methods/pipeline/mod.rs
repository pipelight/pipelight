// Types
use crate::workflow::types::{Duration, Logs, Mode, Pipeline, StepOrParallel, Trigger};
use utils::git::{Flag, Git, Special};

// Traits
use crate::workflow::traits::Getters;
use exec::{Statuable, Status};

// Error Handling
use crate::globals::LOGGER;
use log::{error, info, warn};
use miette::{Error, IntoDiagnostic, Result};

//sys
use rustix::process::{kill_process_group, test_kill_process_group, Signal};
use sysinfo::{get_current_pid, Pid, PidExt, ProcessExt, System, SystemExt};

impl Pipeline {
    /** Print the pipeline status as JSON inside a log file. */
    pub fn log(&self) {
        LOGGER.lock().unwrap().set_file(&self.uuid);
        let json = serde_json::to_string(&self).unwrap();
        error!(target: "pipelines_to_file","{}", json);
    }
    /** On demand,
    Add the current process stdout/stderr to a runnnig pipeline log.
    Beware: Concurent std read/write */
    pub fn hydrate(&mut self) -> Result<()> {
        for step_or_parallel in &mut self.steps {
            match step_or_parallel {
                StepOrParallel::Step(step) => {
                    for command in &mut step.commands {
                        if command.get_status() == Some(Status::Running) {
                            let _ = command.process.read()?;
                        }
                    }
                }
                StepOrParallel::Parallel(parallel) => {
                    for step in &mut parallel.steps {
                        for command in &mut step.commands {
                            if command.get_status() == Some(Status::Running) {
                                let _ = command.process.read()?;
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }
    /** Check if pipeline can be triggered in the actual environment */
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
    /** Check if pipeline can be watched */
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
                let pid = rustix::process::Pid::from_raw(self.event.clone().unwrap().pid.unwrap());
                test_kill_process_group(pid.unwrap()).is_err()
            }
        } else {
            false
        }
    }
    /// If the pid (extracted from logs) exists it means the pipeline is running
    /// (improvement: need to add process name comparision to harden func)
    pub fn is_running(&self) -> Result<()> {
        let mut is: bool = false;
        let mut pipelines = Logs::get_many_by_name(&self.name)?;
        pipelines.reverse();
        let pipeline = pipelines.first();
        if let Some(pipeline) = pipeline {
            let event = &pipeline.event;
            if event.is_some() {
                let raw_pid = &event.clone().unwrap().pid;
                let rustix_pid;
                unsafe { rustix_pid = rustix::process::Pid::from_raw(raw_pid.unwrap()) };
                let sysinfo_pid = sysinfo::Pid::from_u32(raw_pid.unwrap());

                // Guard: check if pid exists
                test_kill_process_group(rustix_pid.unwrap()).into_diagnostic()?;

                // Guard: check if it is a pipelight process
                let mut sys = System::new_all();
                sys.refresh_all();
                if sys
                    .process(sysinfo_pid)
                    .unwrap()
                    .name()
                    .contains("pipelight")
                {
                    return Ok(());
                }
            }
        }
        Err(Error::msg("Pipeline is not running"))
    }
    /// Abort process execution
    pub fn stop(&mut self) -> Result<()> {
        if self.event.is_some() && self.status == Some(Status::Running) {
            let _pid = self.clone().event.unwrap().pid.unwrap();
            unsafe {
                let pgid_raw = self.event.clone().unwrap().pgid.unwrap();
                let pgid = rustix::process::Pid::from_raw(pgid_raw).unwrap();
                kill_process_group(pgid, Signal::Term).into_diagnostic()?
            }
            self.status = Some(Status::Aborted);
            self.log();
        }
        Ok(())
    }
}
