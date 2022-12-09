// Actions: Functions called by cli
mod types;
use crate::shell::{exec_attach, exec_detach, load_config};
use crate::types::Config;
use log::{debug, error, info, trace, warn};
use std::error::Error;

pub fn run(pipeline_name: String) {
    trace!("Running pipeline {} in the background", pipeline_name);
}
pub fn stop() {
    println!("config");
}
pub fn list() -> Result<(), Box<dyn Error>> {
    let config = load_config()?;
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
    println!("config");
}
