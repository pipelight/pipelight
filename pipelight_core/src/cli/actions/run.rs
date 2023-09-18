use crate::cli::utils::detach;

use crate::workflow::{Getters, Node, Pipeline, Trigger};
use exec::Status;
use utils::git::Flag;

use crate::cli::interface::types;

// Logger
use log::{info, trace};

// Global
use crate::globals::CLI;

// Error Handling
use miette::{Error, Result};

/// To be called from the cli.
/// Either spawn a detached new process or spawn an attached thread
/// to run the pipeline
pub fn launch(pipeline_name: String, attach: bool, flag: Option<String>) -> Result<()> {
    // Set triggering env action
    if flag.is_some() {
        Trigger::flag(Some(Flag::from(&flag.unwrap())))?;
    }

    let args;
    unsafe {
        args = (*CLI).clone();
    }

    // Set the detach command subcommand in case of pipeline name is prompted
    let subcommand = types::Commands::Run(types::Pipeline {
        name: Some(pipeline_name.clone()),
        trigger: match args.commands {
            types::Commands::Run(res) => res.trigger,
            types::Commands::Trigger(res) => res,
            _ => types::Trigger {
                flag: Some("manual".to_owned()),
            },
        },
    });

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
            run(&pipeline)?;
        }
        false => detach(Some(subcommand))?,
    }
    Ok(())
}

/// Launch attached thread
pub fn run(p: &Pipeline) -> Result<()> {
    let mut pipeline = p.to_owned();

    // Action
    pipeline.run()?;
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
}
