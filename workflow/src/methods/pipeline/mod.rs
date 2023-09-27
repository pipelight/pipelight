// Types
use crate::types::{Pipeline, StepOrParallel};
// Traits
use exec::{Statuable, Status};
// Error Handling
use log::error;
use miette::{IntoDiagnostic, Result};
//sys
use rustix::process::{kill_process_group, Signal};

// Globals
use crate::globals::LOGGER;

pub mod filters;
pub mod getters;
pub mod is;
pub mod run;

// Modifiers
impl Pipeline {
    /** Print the pipeline status as JSON inside a log file. */
    pub fn log(&self) {
        LOGGER.lock().unwrap().to_file();
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
                            let _ = command.process.io.read()?;
                        }
                    }
                }
                StepOrParallel::Parallel(parallel) => {
                    for step in &mut parallel.steps {
                        for command in &mut step.commands {
                            if command.get_status() == Some(Status::Running) {
                                let _ = command.process.io.read()?;
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }
    /**
    Abort process execution
    Kil the process group
    */
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
