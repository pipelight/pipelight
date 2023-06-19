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
use super::cli::ARGS;

/// To be called from the cli.
/// Either spawn a detached new process or spawn an attached thread
/// to run the pipeline
pub fn run_bin(pipeline_name: String, attach: bool) -> Result<()> {
    let bin = "pipelight";
    let pipeline = Pipeline::get_by_name(&pipeline_name.clone())?;
    if !pipeline.is_triggerable()? {
        let message = "Pipeline can not be triggered in this environment";
        let hint = "Either verify the triggers you set for this pipeline, \
        checkout branch, \
        or add actions like \"manual\" \n";
        warn!(target:"nude", "{}", hint);
        return Err(Error::msg(message));
    }

    let mut args: String;
    let parsed;
    unsafe {
        parsed = Cli::try_parse_from((*ARGS).clone()).into_diagnostic()?;
        let mut args_vec = (*ARGS).clone();
        args_vec.remove(0);
        args = args_vec.join(" ").to_owned();
    }

    match parsed.commands {
        Commands::Run(pipeline) => {
            if pipeline.name.is_none() {
                args = args.replace("run", &format!("run {}", &pipeline_name));
            }
        }
        _ => {
            let message = "Missunderstood commandline argument";
            return Err(Error::msg(message));
        }
    }

    #[cfg(debug_assertions)]
    let command = format!("cargo run --bin {} {} --attach", &bin, &args);

    #[cfg(not(debug_assertions))]
    let command = format!("{} {} --attach", &bin, &args);

    println!("{}", command);

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
    let thread = thread::spawn(move || {
        let mut pipeline = Pipeline::get_by_name(&name).unwrap();
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
