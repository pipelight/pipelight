// Actions: Functions called by cli
use crate::config::{get_config, get_pipeline};
use crate::exec::exec_detached;
pub use crate::logger::read::{json_logs, pretty_logs, raw_logs};
use crate::types::{Config, Pipeline};
use colored::Colorize;
use log::{debug, error, info, trace, warn};
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::{self, BufRead, Read, Write};
use std::path::Path;
use std::process::{Command, Stdio};

pub fn run(pipeline_name: String) -> Result<(), Box<dyn Error>> {
    let bin = "pipelight_run";
    let pipeline = get_pipeline(pipeline_name.clone())?;
    trace!("Create detached subprocess");
    exec_detached(format!("cargo run --bin {} {}", bin, pipeline_name))?;
    Ok(())
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
