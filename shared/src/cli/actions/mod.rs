// Actions: Functions called by cli
use crate::config::{get_config, get_pipeline, lint_config};
use crate::exec::subprocess::{exec_detached, subprocess_detached};
use crate::hook::ensure_folders;
pub use crate::logger::clear_logs;
pub use crate::reader::{json_logs, pretty_logs, raw_logs};
use colored::Colorize;
use log::{debug, error, info, trace, warn};
use std::error::Error;
use std::process::{Command, Output, Stdio};

pub fn run(pipeline_name: String) -> Result<(), Box<dyn Error>> {
    trace!("Create detached subprocess");
    let bin = "pipelight-run";
    let pipeline = get_pipeline(&pipeline_name)?;
    let command = format!("cargo run --bin {} {}", bin, pipeline_name);
    // let command = format!("{} {}", bin, pipeline_name);
    exec_detached(&command)?;
    Ok(())
}

pub fn init() -> Result<(), Box<dyn Error>> {
    ensure_folders()?;
    Ok(())
}

pub fn lint() -> Result<(), Box<dyn Error>> {
    lint_config()?;
    Ok(())
}

pub fn stop() -> Result<(), Box<dyn Error>> {
    println!("stop");
    Ok(())
}

pub fn list() -> Result<(), Box<dyn Error>> {
    let config = get_config()?;
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
