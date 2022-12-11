// Actions: Functions called by cli
mod types;
use crate::config::get_config;
use crate::exec::{exec_attach, exec_detach};
use crate::types::{Config, Pipeline};
use log::{debug, error, info, trace, warn};
use std::error::Error;
use std::io::{Read, Write};
use std::process::{Command, Stdio};

pub fn run(pipeline_name: String) -> Result<(), Box<dyn Error>> {
    let bin = "pipelight_run";

    let config = get_config()?;
    let pipeline_result = config
        .pipelines
        .iter()
        .filter(|p| p.name == pipeline_name)
        .cloned()
        .next();

    if pipeline_result.is_some() {
        let pipeline = pipeline_result.unwrap();
        trace!("Running pipeline {} in the background", pipeline.name);
        let child = Command::new("cargo")
            .arg(format!("run"))
            .arg(format!("--bin"))
            .arg(format!("{}", bin))
            .arg(format!("{}", pipeline_name))
            .stdout(Stdio::piped())
            .spawn()
            .expect("failed to execute child");

        let output = child.wait_with_output().expect("failed to wait on child");
        println!("{:?}", output);
        Ok(())
    } else {
        let message = "Pipeline doesn't exist";
        error!("{}", message);
        Err(Box::from(message))
    }
}

pub fn stop() {
    println!("stop");
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
