// Struct
use crate::services::types::{Action, Service};
use crate::types::{Commands, DetachableCommands, PostCommands};
use exec::Status;
use workflow::{Config, Getters, Node, Pipeline};
// Globals
use crate::globals::CLI;
use crate::verbosity::external::{level_value, Verbosity};
use log::LevelFilter;
use utils::globals::LOGGER;
// Traits
use crate::services::traits::FgBg;
// Error Handling
use miette::{Error, Result};

pub fn launch() -> Result<()> {
    let mut args = CLI.lock().unwrap().clone();

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
    let config = Config::get()?;

    // Guard
    pipeline.is_triggerable()?;
    if args.verbose.log_level_filter() == LevelFilter::Error {
            // Retrieve global options
        if config.has_loglevel_option().unwrap() {
            if let Some(level_filter) = config.get_default_loglevel().ok() {
                let level = level_filter.to_level();
                args.verbose = Verbosity::new(level_value(level).try_into().unwrap(), 0);
                LOGGER.lock().unwrap().set_level(&level_filter)?;
            }
        }
        // Retrieve per-pipeline options
        if pipeline.has_loglevel_option().unwrap() {
            if let Some(level_filter) = pipeline.get_default_loglevel().ok() {
                let level = level_filter.to_level();
                args.verbose = Verbosity::new(level_value(level).try_into().unwrap(), 0);
                LOGGER.lock().unwrap().set_level(&level_filter)?;
            }
        }
    }
    if args.attach.is_none() {
        // Retrieve global options
        if config.has_attach_option().unwrap() {
            args.attach = Some(!config.should_detach().unwrap());
        }
        // Retrieve per-pipeline options
        if pipeline.has_attach_option().unwrap() {
            args.attach = Some(!pipeline.should_detach().unwrap());
        }
    }

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
