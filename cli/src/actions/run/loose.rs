// Struct
use crate::services::types::{Action, Service};
use crate::types::{Commands, DetachableCommands, PostCommands};
use exec::Status;
use workflow::{Getters, Node, Pipeline};
// Globals
use crate::globals::CLI;
// Traits
use crate::services::traits::FgBg;
// Error Handling
use miette::{Error, Result};

pub fn launch() -> Result<()> {
    let args = CLI.lock().unwrap().clone();

    // Retrieve command line args
    let name: String;
    match args.commands {
        Commands::PostCommands(PostCommands::DetachableCommands(DetachableCommands::Run(e))) => {
            name = e.name.unwrap();
        }
        _ => {
            let message = "Couldn't retrieve pipeline name";
            return Err(Error::msg(message));
        }
    };

    let mut pipeline = Pipeline::get_by_name(&name)?;

    // Guard
    pipeline.is_triggerable()?;
    // Action
    pipeline.run()?;
    // Return pipeline log
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
