use super::detach;
use exec::Status;
use pipeline::{Getters, Node, Pipeline, Trigger};
use std::thread;
use utils::git::Flag;

// Logger
use log::{info, trace};

// Error Handling
use miette::{Error, Result};

/// To be called from the cli.
/// Either spawn a detached new process or spawn an attached thread
/// to run the pipeline
pub fn launch(pipeline_name: String, attach: bool, flag: Option<String>) -> Result<()> {
    if flag.is_some() {
        Trigger::flag(Flag::from(&flag.unwrap()));
    }
    // Ensure
    // Check if pipeline exists and give hints
    let pipeline = Pipeline::get_by_name(&pipeline_name)?;
    if !pipeline.is_triggerable()? {
        let message = "Pipeline can not be triggered in this environment";
        let hint = "Either verify the triggers you set for this pipeline, \
        checkout branch, \
        or add actions like \"manual\" \n";

        info!(target:"nude", "{}", hint);
        return Err(Error::msg(message));
    }
    // Run or Fork
    match attach {
        true => {
            // Lauch in attached thread
            trace!("Run pipeline in attached thread");
            run_in_thread(&pipeline)?;
        }
        false => detach()?,
    }
    Ok(())
}

/// Launch attached thread
pub fn run_in_thread(p: &Pipeline) -> Result<()> {
    let mut pipeline = p.to_owned();

    let thread = thread::spawn(move || {
        // Action
        pipeline.run();
        // Return status
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
