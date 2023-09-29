// Test
mod test;
// Structs
use crate::types::{Logs, Pipeline, Trigger};
use utils::git::{Flag, Git, Special};
// Traits
use exec::Status;
// Unix process management
use rustix::process::test_kill_process;
use sysinfo::{PidExt, ProcessExt, System, SystemExt};
// Error Handling
use miette::{Error, IntoDiagnostic, Result};

/**
The following methods returns informations about pipeline states.
They question and sanitize the logs according to the unix kernel answers.

Reasons:
To avoid duplicates,
pipelight use its autogenerated logs as a lock file to keep the state of the executing pipelines.
But this method is much more error prone than a lock file as logs are frequently manipulated.

That is why pipelight chose to distruss the generated log files and concider the unix kernel
a much older piece of software as the uniq source of truth.

Those methods are to be used everytime logs are loaded.
*/

impl Pipeline {
    /**
    Check if a triggered pipeline has an already running instance.
    Should be combined with .is_ok() and .is_err() to generate a boolean.

    It cascade checks the following conditions:
    - if running homologous(same name) in logs exists.
    - if homologous pid exists on the unix process registry.
    - if corresponding program is a "pipelight" instance.

    If those conditions are met we assume the pipeline has an already running instance.
    */
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
                let process_name = sys.process(sysinfo_pid).unwrap().name();
                if process_name.contains("pipelight") {
                    return Ok(());
                }
            }
        }
        Err(Error::msg("Pipeline is not running"))
    }
    /**
    Check if the pipeline instance(loaded from logs) is running.

    It cascade checks the following conditions:
    - if pipeline pid exists on the unix process registry.
    - if corresponding program is a "pipelight" instance.

    If those conditions are met we assume the pipeline is running.
    */
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
            let process_name = sys.process(sysinfo_pid).unwrap().name();
            if process_name.contains("pipelight") {
                return Ok(());
            }
        }
        Err(Error::msg("Pipeline is not running"))
    }
    /**
    Check if the pipeline can be triggered in the actual environment
    */
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
    /**
    Check if the pipeline has a trigger that contains a "watch" flag
    */
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
    /**
    Tells if the pipeline execution has been aborted.

    Compares if log_pid is in system pid list.
    If not, the program has been aborted
    */
    pub fn is_aborted(&mut self) -> bool {
        if self.event.is_some() {
            if self.status == Some(Status::Aborted) {
                return true;
            }
            if self.status == Some(Status::Running) {
                unsafe {
                    let pid =
                        rustix::process::Pid::from_raw(self.event.clone().unwrap().pid.unwrap());
                    test_kill_process(pid.unwrap()).is_err()
                }
            } else {
                false
            }
        } else {
            false
        }
    }
}