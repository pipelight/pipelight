#![allow(unused_variables)]
#![allow(unused_must_use)]
use exec::types::Status;
#[allow(dead_code)]
use log::error;
use pipeline::types::{traits::getters::Getters, Pipeline};
use std::env;

// Error Handling
use miette::{Error, Result};
use std::process::exit;
// use std::error::Error;

fn main() {
    handler().unwrap_or_else(|e| {
        error!("{}", e);
        exit(1)
    })
}

/// Launch detached subprocess
fn handler() -> Result<()> {
    run()?;
    Ok(())
}

// / Get command line args and run pipeline
pub fn run() -> Result<()> {
    // Collect Args
    let args = env::args().collect::<Vec<String>>();
    let pipeline_name: String = args[1].to_owned();

    let mut pipeline = Pipeline::get_by_name(&pipeline_name)?;
    pipeline.run();

    match pipeline.status {
        Some(Status::Succeeded) => {
            return Ok(());
        }
        Some(Status::Failed) => {
            let message = "Pipeline status: Failed";
            return Err(Error::msg(message));
        }
        Some(Status::Aborted) => {
            let message = "Pipeline status: Aborted";
            return Err(Error::msg(message));
        }
        _ => return Ok(()),
    }
}
