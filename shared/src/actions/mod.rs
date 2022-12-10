// Actions: Functions called by cli
mod types;
use crate::config::get_config;
use crate::exec::{exec_attach, exec_detach};
use crate::types::Config;
use log::{debug, error, info, trace, warn};
use std::error::Error;

pub fn run(pipeline_name: String) -> Result<(), Box<dyn Error>> {
    let config = get_config()?;
    trace!("Running pipeline {} in the background", pipeline_name);

    Ok(())
    // Check duplicate

    // for command in pipeline.
    // exec_attah(pi)
}

pub fn stop() {
    println!("config");
}

pub fn list() -> Result<(), Box<dyn Error>> {
    let config = get_config()?;
    // Print headers
    // String litteral might not be a variable (c injections issues)
    // let col = "{0: <10} {1: <20} {2: <10} {3}";
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

pub fn logs() {
    println!("logs");
}
