// Structs
use crate::types::{Pipeline, StepOrParallel};
// Traits
use exec::{Statuable, Status};
// Globals
use once_cell::sync::Lazy;
use std::io::Write;
use std::sync::{Arc, Mutex};
use pipelight_utils::globals::LOGGER;
// Fylesystem manipulation
use std::fs;
use std::fs::{create_dir_all, File};
use std::path::Path;
// Error Handling
use log::error;
use miette::{IntoDiagnostic, Result};

/**
Lazy global that contains the default output directory to be used.
*/
pub static OUTDIR: Lazy<Arc<Mutex<String>>> =
    Lazy::new(|| Arc::new(Mutex::new(".pipelight/logs".to_owned())));

impl Pipeline {
    /**
    Delete the pipeline log file.
    */
    pub fn clean(&self) -> Result<()> {
        //Ensure dir
        let dir = OUTDIR.lock().unwrap();
        fs::create_dir_all(dir.clone()).into_diagnostic()?;

        let stdout_path = format!("{}/{}.json", dir.clone(), self.uuid);

        let path = Path::new(&stdout_path);
        if path.exists() && path.is_file() {
            fs::remove_file(path).into_diagnostic()?;
        }
        // // Subprocess tmp files
        let processes = self.get_procs()?;
        for process in processes {
            process.io.clean()?;
        }
        Ok(())
    }
    /**
    Print the pipeline status as JSON inside a log file.
    */
    pub fn log(&self) -> Result<()> {
        //Ensure dir
        let dir = OUTDIR.lock().unwrap();
        fs::create_dir_all(dir.clone()).into_diagnostic()?;

        let json = serde_json::to_string(&self).unwrap() + "\n";

        let stdout_path = format!("{}/{}.json", dir.clone(), self.uuid);
        let mut f = File::options()
            .append(true)
            .write(true)
            .create(true)
            .open(stdout_path)
            .into_diagnostic()?;
        f.write_all(json.as_bytes()).into_diagnostic()?;
        Ok(())
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
