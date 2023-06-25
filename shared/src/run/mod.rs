use exec::types::Status;
use exec::Exec;
use pipeline::types::{traits::getters::Getters, Node, Pipeline};
use std::thread;

// CLI
use super::cli::types::{Cli, Commands};
use clap::{Command, Parser};

// Logger
use log::{debug, error, info, trace, warn};

// Error Handling
use miette::{miette, Diagnostic, Error, IntoDiagnostic, NamedSource, Report, Result, SourceSpan};

// Globals
use super::cli::CLI;

/// To be called from the cli.
/// Either spawn a detached new process or spawn an attached thread
/// to run the pipeline
pub fn run_bin(pipeline_name: String, attach: bool) -> Result<()> {
    // Check if pipeline exists and give hints
    let pipeline = Pipeline::get_by_name(&pipeline_name.clone())?;
    if !pipeline.is_triggerable()? {
        let message = "Pipeline can not be triggered in this environment";
        let hint = "Either verify the triggers you set for this pipeline, \
        checkout branch, \
        or add actions like \"manual\" \n";

        info!(target:"nude", "{}", hint);
        return Err(Error::msg(message));
    }

    match attach {
        true => {
            // Lauch in attached thread
            trace!("Run pipeline in attached thread");
            run_in_thread(&pipeline)?;
        }
        false => {
            // Run a detached subprocess
            trace!("Create detached subprocess");
            let bin = "pipelight";
            let mut args;
            unsafe {
                args = (*CLI).clone();
            }
            args.attach = true;

            #[cfg(debug_assertions)]
            let command = format!("cargo run --bin {} {}", &bin, &args);

            #[cfg(not(debug_assertions))]
            let command = format!("{} {}", &bin, &args);

            println!("{}", command);

            Exec::new().detached(&command)?;
        }
    }
    Ok(())
}

/// Launch attached thread
pub fn run_in_thread(p: &Pipeline) -> Result<()> {
    let mut pipeline = p.to_owned();

    let thread = thread::spawn(move || {
        pipeline.run();
        println!("{}", Node::from(&pipeline));
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
