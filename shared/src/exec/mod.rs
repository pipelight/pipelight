pub mod subprocess;
use crate::logger::{debug, error, info, set_logger, trace, warn};
use crate::types::config::{Pipeline, Step};
use crate::types::logs::{PipelineLog, PipelineStatus, StepLog};
use std::env::current_dir;
use std::error::Error;

// pub fn run(pipeline: &Pipeline) -> Result<(), Box<dyn Error>> {
//     let mut log = PipelineLog::new(&pipeline.name).to_owned();
//     let run_result = run_pipeline(&pipeline, &mut log);
//     match run_result {
//         Ok(res) => {
//             //Logging to file
//             log.status(PipelineStatus::Succeeded);
//             let json = serde_json::to_string(&log).unwrap();
//             info!(target:"pipeline_json", "{}",json );
//             return Ok(());
//         }
//         Err(e) => {
//             //Logging to file
//             log.status(PipelineStatus::Failed);
//             let json = serde_json::to_string(&log).unwrap();
//             info!(target:"pipeline_json", "{}",json );
//             return Err(Box::from(e));
//         }
//     };
// }
//
// pub fn run_pipeline(pipeline: &Pipeline, log: &mut PipelineLog) -> Result<(), Box<dyn Error>> {
//     // Logging to file "Pipeline Started"
//     let json = serde_json::to_string(&log).unwrap();
//     info!(target:"pipeline_json", "{}",json );
//     // Loop through steps
//     for step in &pipeline.steps {
//         for command in &step.commands {
//             let res = shell(&command)?;
//             info!(target:"pipeline_raw", "{}", command);
//             let shell_result = shell(&command);
//             debug!(target:"pipeline_raw", "{}",&res);
//
//             // Format to json and logging to file
//             log.status(PipelineStatus::Running);
//             log.step(&step.name);
//             log.command(&command, &res);
//
//             let json = serde_json::to_string(&log).unwrap();
//             info!(target:"pipeline_json", "{}",json );
//         }
//     }
//     Ok(())
// }

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

/// Return the wqay the pipeline has been triggered
/// (manually or via git hook)
pub fn get_origin() -> Result<(), Box<dyn Error>> {
    let res = current_dir()?;
    Ok(())
}
