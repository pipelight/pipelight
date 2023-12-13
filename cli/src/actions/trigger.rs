// Struct
use crate::services::types::{Action, Service};
use crate::types::{Commands, DetachableCommands, Pipeline, PostCommands, Trigger};
use crate::verbosity::external::{level_value, Verbosity};
use workflow;
// Traits
use crate::services::traits::FgBg;
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
            let message = "Couldn.t retrieve pipeline name";
            return Err(Error::msg(message));
        }
    };

    let mut pipelines = workflow::Pipeline::get()?;
    let config = workflow::Config::get()?;

    pipelines.par_iter_mut().for_each(|pipeline| {
        // Guard
        if pipeline.is_triggerable_strict().unwrap() {
            let mut args = CLI.lock().unwrap().clone();

            // Retrieve global options
            if config.has_attach_option().unwrap() {
                args.attach = Some(!config.should_detach().unwrap());
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
                args.attach = Some(!pipeline.should_detach().unwrap());
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
        }
    });
    Ok(())
}
