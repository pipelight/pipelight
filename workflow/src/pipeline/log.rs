// Structs
use crate::types::{Pipeline, StepOrParallel};
// Traits
use exec::{Statuable, Status};
// Globals
use utils::globals::LOGGER;
// Fylesystem manipulation
use std::fs;
use std::path::Path;
// Error Handling
use log::error;
use miette::{IntoDiagnostic, Result};

impl Pipeline {
    /**
    Delete the pipeline log file.
    */
    pub fn clean(&self) -> Result<()> {
        LOGGER.lock().unwrap().to_file();
        LOGGER.lock().unwrap().set_file(&self.uuid);
        let logger = LOGGER.lock().unwrap().clone();

        // Pipeline main file
        let file = logger.pipelines.file_info;
        if let Some(file) = file {
            let path = format!("{}/{}.json", file.directory, file.name);
            let path = Path::new(&path);
            if path.exists() && path.is_file() {
                fs::remove_file(path).into_diagnostic()?;
            }
        }
        // Subprocess tmp files
        let processes = self.get_procs()?;
        for process in processes {
            process.io.clean()?;
        }
        Ok(())
    }
    /**
    Print the pipeline status as JSON inside a log file.
    */
    pub fn log(&self) {
        LOGGER.lock().unwrap().to_file();
        LOGGER.lock().unwrap().set_file(&self.uuid);
        let json = serde_json::to_string(&self).unwrap();
        error!(target: "pipelines_to_file","{}", json);
    }
    /**
    On demand,
    Add the current process stdout/stderr to a runnnig pipeline log.
    Beware: Concurent std read/write
    */
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
