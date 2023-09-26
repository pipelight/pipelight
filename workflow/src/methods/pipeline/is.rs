// Types
use crate::types::{Logs, Pipeline, Trigger};
use utils::git::{Flag, Git, Special};

// Traits

use exec::{Status};

//sys
use rustix::process::{test_kill_process};
use sysinfo::{PidExt, ProcessExt, System, SystemExt};

// Error Handling


use miette::{Error, IntoDiagnostic, Result};

impl Pipeline {
    /// If the pid (extracted from logs) exists it means the pipeline is running
    pub fn has_homologous_already_running(&self) -> Result<()> {
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
                test_kill_process(rustix_pid.unwrap()).into_diagnostic()?;

                // Guard: check if it is a pipelight process
                let mut sys = System::new_all();
                sys.refresh_all();
                if sys
                    .process(sysinfo_pid)
                    .unwrap()
                    .name()
                    .contains("pipelight")
                {
                    let name = sys.process(sysinfo_pid).unwrap().name();
                    return Ok(());
                }
            }
        }
        Err(Error::msg("Pipeline is not running"))
    }
    /// If the pid (extracted from logs) exists it means the pipeline is running
    pub fn is_running(&self) -> Result<()> {
        let event = self.event.clone();
        if event.is_some() {
            let raw_pid = &event.clone().unwrap().pid;
            let rustix_pid;
            unsafe { rustix_pid = rustix::process::Pid::from_raw(raw_pid.unwrap()) };
            let sysinfo_pid = sysinfo::Pid::from_u32(raw_pid.unwrap());

            // Guard: check if pid exists
            test_kill_process(rustix_pid.unwrap()).into_diagnostic()?;

            // Guard: check if it is a pipelight process
            let mut sys = System::new_all();
            sys.refresh_all();
            if sys
                .process(sysinfo_pid)
                .unwrap()
                .name()
                .contains("pipelight")
            {
                let name = sys.process(sysinfo_pid).unwrap().name();
                return Ok(());
            }
        }
        Err(Error::msg("Pipeline is not running"))
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
                test_kill_process(pid.unwrap()).is_err()
            }
        } else {
            false
        }
    }
}
