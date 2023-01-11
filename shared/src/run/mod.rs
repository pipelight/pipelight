use exec::Exec;
use log::{debug, error, info, trace, warn};
use pipeline::types;
use pipeline::types::{Config, Pipeline};
use std::env;
use std::error::Error;

/// To be called from the cli
pub fn run_bin(pipeline_name: String) -> Result<(), Box<dyn Error>> {
    trace!("Create detached subprocess");
    let bin = "pipelight-run";

    let pipeline = Pipeline::name(&pipeline_name)?;
    let command = format!("cargo run --bin {} {}", bin, pipeline_name);
    // let command = format!("{} {}", bin, pipeline_name);
    Exec::new().detached(&command)?;
    Ok(())
}

/// Launch attached subprocess
pub fn run() -> Result<(), Box<dyn Error>> {
    // Collect Args
    let args = env::args().collect::<Vec<String>>();
    let pipeline_name: String = args[1].to_owned();

    let mut pipeline = Pipeline::name(&pipeline_name)?;
    pipeline.run();
    Ok(())
}
