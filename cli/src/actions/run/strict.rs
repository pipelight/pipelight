// Struct
use crate::services::types::{Action, Service};
use exec::Status;
use workflow::{Getters, Node, Pipeline};
// Globals
use crate::globals::CLI;
use crate::verbosity::external::{level_value, Verbosity};
use log::LevelFilter;
use utils::globals::LOGGER;
// Traits
use crate::services::traits::FgBg;
// Error Handling
use miette::{Error, Result};
use workflow::error::IsError;

pub fn launch(name: &str) -> Result<()> {
    let mut pipeline = Pipeline::get_by_name(name)?;
    let config = workflow::Config::get()?;

    println!("{:#?}", LOGGER.lock().unwrap().pipelines);

    // Guard
    if pipeline.is_triggerable()? {
        let mut args = CLI.lock().unwrap().clone();

        if args.verbose.log_level_filter() == LevelFilter::Error {
            if config.has_loglevel_option().unwrap() {
                if let Some(level_filter) = config.get_default_loglevel().ok() {
                    let level = level_filter.to_level();
                    args.verbose = Verbosity::new(level_value(level).try_into().unwrap(), 0);
                    LOGGER.lock().unwrap().set_level(&level_filter)?;
                }
            }
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
                args.attach = Some(!config.should_detach()?);
            }

            // Retrieve per-pipeline options
            if pipeline.has_attach_option().unwrap() {
                args.attach = Some(!pipeline.should_detach()?);
            }
        }

        match args.attach {
            None => {
                Service::new(Action::RunStrict, Some(args))?.should_detach()?;
                Ok(())
            }
            Some(false) => {
                Service::new(Action::RunStrict, Some(args))?.should_detach()?;
                Ok(())
            }
            Some(true) => {
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
        }
    } else {
        let mut string = "".to_owned();
        if let Some(triggers) = pipeline.triggers {
            // let actions = triggers.iter().map(|e| e.get_action());
            for trigger in triggers {
                string += &format!("{}\n", trigger);
            }
        }
        let mut hint = "".to_owned();
        hint += "Checkout to an authorize git branch or use an authorize action:\n";
        hint += &string;

        let message = "Can not trigger the pipeline in this environment";
        Err(IsError::new(message, &hint)?.into())
    }
}
