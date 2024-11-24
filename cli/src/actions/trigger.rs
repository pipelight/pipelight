// Struct
use crate::services::{Action, FgBg, Service};

use crate::types::{Attach, Commands, DetachableCommands, Pipeline, PostCommands, Trigger};
use crate::verbosity::external::{level_value, Verbosity};
use workflow;
// Traits
use workflow::Getters;
// IterMut
use rayon::prelude::*;
// Globals
use crate::globals::CLI;
// Error Handling
use miette::{Error, Result};

pub fn launch() -> Result<()> {
    let trigger: Trigger;
    let args = CLI.lock().unwrap().clone();
    // Retrieve command line args
    match args.commands {
        Commands::PostCommands(PostCommands::DetachableCommands(DetachableCommands::Trigger(
            e,
        ))) => {
            trigger = e;
        }
        _ => {
            let message = "Couldn't retrieve pipeline name";
            return Err(Error::msg(message));
        }
    };

    let mut pipelines = workflow::Pipeline::get()?;
    let config = workflow::Config::get()?;

    pipelines.par_iter_mut().for_each(|pipeline| {
        // Guard
        if pipeline.is_triggerable_strict().is_err() {
            return;
        }

        let mut args = CLI.lock().unwrap().clone();

        // Retrieve global options
        if config.has_attach_option().unwrap() {
            args.attach = match config.should_attach().ok() {
                Some(false) => Some(String::from(&Attach::False)),
                Some(true) | None => Some(String::from(&Attach::True)),
            };
        }

        if config.has_loglevel_option().unwrap() {
            let mut level = None;
            if let Some(level_filter) = config.get_default_loglevel().ok() {
                level = level_filter.to_level()
            }
            args.verbose = Verbosity::new(level_value(level).try_into().unwrap(), 0);
            // LOGGER.lock().unwrap().set_level(&args.verbose)?;
        }

        // Retrieve per-pipeline options
        if pipeline.has_attach_option().unwrap() {
            args.attach = match pipeline.should_detach().ok() {
                Some(false) => Some(String::from(&Attach::True).to_owned()),
                Some(true) | None => Some(String::from(&Attach::False).to_owned()),
            }
        }
        if pipeline.has_loglevel_option().unwrap() {
            let mut level = None;
            if let Some(level_filter) = pipeline.get_default_loglevel().ok() {
                level = level_filter.to_level()
            }
            args.verbose = Verbosity::new(level_value(level).try_into().unwrap(), 0);
            // LOGGER.lock().unwrap().set_level(&args.verbose)?;
        }
        args.commands = Commands::PostCommands(PostCommands::DetachableCommands(
            DetachableCommands::Run(Pipeline {
                trigger: trigger.to_owned(),
                name: Some(pipeline.name.clone()),
            }),
        ));
        Service::new(Action::RunLoose, Some(args))
            .unwrap()
            .should_detach()
            .unwrap();
    });
    Ok(())
}
