use exec::Exec;
use log::{debug, error, info, trace, warn};
use pipeline::types::Config;
use std::error::Error;

pub fn run(pipeline_name: String) -> Result<(), Box<dyn Error>> {
    trace!("Create detached subprocess");
    let bin = "pipelight-run";
    let pipeline = Config::new()?.pipeline(&pipeline_name)?;
    let command = format!("cargo run --bin {} {}", bin, pipeline_name);
    // let command = format!("{} {}", bin, pipeline_name);
    Exec::new().detached(&command)?;
    Ok(())
}
