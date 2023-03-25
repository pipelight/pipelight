use exec::types::Status;
use exec::Exec;
use log::trace;
use pipeline::types::{traits::getters::Getters, Node, Pipeline};
use std::env;
use std::thread;
// Error Handling
use miette::{miette, Diagnostic, Error, IntoDiagnostic, NamedSource, Report, Result, SourceSpan};
use thiserror::Error;
// use std::error::Error;

/// To be called from the cli.
/// Either spawn a detached new process or spawn an attached thread
/// to run the pipeline
pub fn run_bin(pipeline_name: String, attach: bool) -> Result<()> {
    let bin = "pipelight-run";

    let pipeline = Pipeline::get_by_name(&pipeline_name)?;

    #[cfg(debug_assertions)]
    let command = format!("cargo run --bin {} {}", bin, pipeline_name);

    #[cfg(not(debug_assertions))]
    let command = format!("{} {}", bin, pipeline_name);

    match attach {
        true => {
            // Lauch in attach thread
            run_in_thread(&pipeline_name)?;
        }
        false => {
            // Lauch detached process
            // trace!("Create detached subprocess");
            Exec::new().detached(&command)?;
        }
    }
    Ok(())
}

/// Launch attached thread
pub fn run_in_thread(name: &str) -> Result<()> {
    let name = name.to_owned();
    let thread = thread::spawn(move || -> Result<()> {
        let mut pipeline = Pipeline::get_by_name(&name).unwrap();
        pipeline.run();
        // println!("{}", Node::from(&pipeline));
        match pipeline.status {
            Some(Status::Succeeded) => Ok(()),
            Some(Status::Failed) => {
                let message = "Pipeline status: Failed";
                Err(Error::msg(message))
            }
            _ => Ok(()),
        }
    });
    thread.join().unwrap()?;
    Ok(())
}
