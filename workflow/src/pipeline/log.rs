// Structs
use crate::types::{Pipeline, StepOrParallel};
// Traits
use exec::{Statuable, Status};
// Globals
use crate::globals::LOGGER;
// Error Handling
use log::error;
use miette::Result;

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
                            command.process.io.read()?;
                        }
                    }
                }
                StepOrParallel::Parallel(parallel) => {
                    for step in &mut parallel.steps {
                        for command in &mut step.commands {
                            if command.get_status() == Some(Status::Running) {
                                command.process.io.read()?;
                            }
                        }
                    }
                }
            }
        }
        Ok(())
    }
}
