use crate::exec::shell;
use crate::logger::{debug, error, info, set_logger, trace, warn};
use crate::types::logs::{PipelineLog, PipelineStatus, StepLog};
use crate::types::{Pipeline, Step};
use std::error::Error;

pub fn run_pipeline(pipeline: Pipeline) -> Result<(), Box<dyn Error>> {
    // Logging to file "Pipeline Started"
    let log = PipelineLog::new(&pipeline.name).to_owned();
    let json = serde_json::to_string(&log).unwrap();
    info!(target:"pipeline_json", "{}",json );
    // Loop through steps
    for step in pipeline.clone().steps {
        for command in step.clone().commands {
            run_command(&pipeline, &step, &command);
        }
    }
    Ok(())
}
fn run_command(pipeline: &Pipeline, step: &Step, command: &str) -> Result<(), Box<dyn Error>> {
    let res = shell(command.to_owned())?;
    // Raw logging to file
    info!(target:"pipeline_raw", "{}",&command);
    debug!(target:"pipeline_raw", "{}",&res);

    // Format to json and logging to file
    let mut log = PipelineLog::new(&pipeline.name).to_owned();
    log.status(PipelineStatus::Running);
    log.step(&step.name);
    log.command(&command, &res);

    let json = serde_json::to_string(&log).unwrap();
    info!(target:"pipeline_json", "{}",json );
    Ok(())
}
