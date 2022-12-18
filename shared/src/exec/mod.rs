// mod subprocess;
pub mod subprocess;
use crate::logger::{debug, error, info, set_logger, trace, warn};
use crate::types::logs::{PipelineLog, PipelineStatus, StepLog};
use crate::types::{Pipeline, Step};
use std::error::Error;

pub fn run_pipeline(pipeline: Pipeline) -> Result<(), Box<dyn Error>> {
    // Logging to file "Pipeline Started"
    let mut log = PipelineLog::new(&pipeline.name).to_owned();
    let json = serde_json::to_string(&log).unwrap();
    info!(target:"pipeline_json", "{}",json );
    // Loop through steps
    for step in pipeline.steps {
        for command in step.commands {
            info!(target:"pipeline_raw", "{}", command);
            let res = shell(&command)?;
            debug!(target:"pipeline_raw", "{}",&res);

            // Format to json and logging to file
            log.status(PipelineStatus::Running);
            log.step(&step.name);
            log.command(&command, &res);

            let json = serde_json::to_string(&log).unwrap();
            info!(target:"pipeline_json", "{}",json );
        }
    }
    Ok(())
}

pub fn shell<'a>(command: &str) -> Result<String, String> {
    let user_shell = subprocess::get_shell();
    let output = subprocess::subprocess_attached(&user_shell, command);
    match output {
        Ok(output) => {
            return Ok(output);
        }
        Err(e) => {
            return Err(e);
        }
    };
}
