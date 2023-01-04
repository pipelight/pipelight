use exec::Exec;
use log::{debug, error, info, trace, warn};
use pipeline::cast::{Config, Pipeline};
use pipeline::types;
use std::env;
use std::error::Error;

/// To be called from the cli
pub fn run(pipeline_name: String) -> Result<(), Box<dyn Error>> {
    trace!("Create detached subprocess");
    let bin = "pipelight-run";
    let pipeline = Config::new()?.pipeline(&pipeline_name)?;
    let command = format!("cargo run --bin {} {}", bin, pipeline_name);
    // let command = format!("{} {}", bin, pipeline_name);
    Exec::new().detached(&command)?;
    Ok(())
}

/// Launch attached subprocess
fn run_bin() -> Result<(), Box<dyn Error>> {
    // Collect Args
    let args = env::args().collect::<Vec<String>>();
    let pipeline_name: String = args[1].to_owned();

    let p: Pipeline = Config::new()?.pipeline(&pipeline_name)?;
    let mut pipeline = types::Pipeline::from(p);

    let is_running: bool = pipeline.is_running()?;
    if is_running {
        let message = "Pipeline already running";
        return Err(Box::from(message));
    } else {
        pipeline.run();
    }
    pipeline.run();
    Ok(())
}
