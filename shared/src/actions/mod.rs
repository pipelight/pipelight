// Actions: Functions called by cli
use crate::config::{get_config, get_pipeline};
use crate::exec::exec;
use crate::types::logs::PipelineLog;
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
    trace!("Create subprocess");
    exec(format!("cargo run --bin {} {}", bin, pipeline_name))?;
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

pub fn json_logs() {
    let file_path = ".pipelight/logs/pipelines.json.log";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    println!("{}", contents);
}
pub fn raw_logs() {
    let file_path = ".pipelight/logs/pipelines.raw.log";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    println!("{}", contents);
}

pub fn pretty_logs() -> Result<(), Box<dyn Error>> {
    let file_path = ".pipelight/logs/pipelines.json.log";
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines(file_path) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(json) = line {
                let log = serde_json::from_str::<PipelineLog>(&json)?;

                println!("{:?}", log);
            }
        }
    }
    Ok(())
}
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
