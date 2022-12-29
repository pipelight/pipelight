// Actions: Functions called by cli
use crate::exec::Exec;
use crate::hooks::Hooks;
use crate::types::Config;
use log::{debug, error, info, trace, warn};
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

pub fn init() -> Result<(), Box<dyn Error>> {
    Hooks::ensure()?;
    Ok(())
}

pub fn lint() -> Result<(), Box<dyn Error>> {
    Config::new()?.lint()?;
    Ok(())
}

pub fn stop() -> Result<(), Box<dyn Error>> {
    println!("stop");
    Ok(())
}

pub fn list() -> Result<(), Box<dyn Error>> {
    let config = Config::new()?;
    println!(
        "{0: <10} {1: <20} {2: <10} {3}",
        "status", "last_run_date", "hook", "name"
    );
    for pipeline in config.pipelines {
        println!(
            "{0: <10} {1: <20} {2: <10} {3}",
            "status", "last_run date", "hook", pipeline.name
        )
    }
    Ok(())
}
