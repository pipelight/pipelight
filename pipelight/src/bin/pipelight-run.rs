#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_must_use)]
use log::LevelFilter::{Debug, Trace};
#[allow(dead_code)]
use shared::config::get_pipeline;
use shared::exec::run_pipeline;
use shared::logger::{debug, error, info, set_logger, trace, warn};
use shared::types::logs::{PipelineLog, PipelineStatus, StepLog};
use shared::types::{Pipeline, Step};
use std::env;
use std::error::Error;
use std::process::exit;

fn main() {
    handler().unwrap_or_else(|a| exit(1))
}
/// Launch attached subprocess
fn handler() -> Result<(), Box<dyn Error>> {
    set_logger(Trace);

    let args = env::args().collect::<Vec<String>>();
    let pipeline_name: String = args[1].to_owned();
    let pipeline = get_pipeline(pipeline_name)?.clone();

    let run_result = run_pipeline(pipeline.clone());
    let res = match run_result {
        Ok(res) => {
            //Logging to file
            let mut log = PipelineLog::new(&pipeline.name).to_owned();
            log.status(PipelineStatus::Succeeded);
            let json = serde_json::to_string(&log).unwrap();
            info!(target:"pipeline_json", "{}",json );

            return Ok(res);
        }
        Err(e) => {
            //Logging to file
            let mut log = PipelineLog::new(&pipeline.name).to_owned();
            log.status(PipelineStatus::Failed);
            let json = serde_json::to_string(&log).unwrap();
            info!(target:"pipeline_json", "{}",json );

            return Err(Box::from(e));
        }
    };
}
