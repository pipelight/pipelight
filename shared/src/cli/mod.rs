//mods
pub mod print;
pub mod prompt;
pub mod types;

// Cli core
use crate::run;
use crate::stop;
use crate::trigger;
use clap::Parser;
use log::info;

// Logger
use utils::logger::logger;

use pipeline::types::{traits::getters::Getters, Config, Logs, Pipeline};
use std::error::Error;

/// Execute the Command Line Tool (cli)
/// Initialize Logger and program global vars
pub fn get_args() -> Result<(), Box<dyn Error>> {
    let args = types::Cli::parse();
    // Set verbosity
    let verbosity = args.verbose.log_level_filter();
    logger.lock().unwrap().level(&verbosity);

    Config::new(args.raw.clone())?;

    match args.commands {
        types::Commands::Ls(list) => {
            // info!("Listing piplines");
            if list.name.is_some() {
                let pipeline = Pipeline::get_by_name(&list.name.unwrap())?;
                print::inspect(&pipeline, list.json)?;
            } else {
                print::list();
            }
        }
        types::Commands::Inspect(list) => {
            // info!("Listing piplines");
            if list.name.is_some() {
                let pipeline = Pipeline::get_by_name(&list.name.unwrap())?;
                print::inspect(&pipeline, list.json)?;
            } else {
                prompt::inspect_prompt()?;
            }
        }
        types::Commands::Trigger(trigger) => {
            // info!("Triggering piplines");
            trigger::trigger_bin(trigger.attach, args.raw.clone())?;
        }
        types::Commands::Run(pipeline) => {
            // info!("Running pipline {:#?}", pipeline.name);
            if pipeline.name.is_some() {
                run::run_bin(pipeline.name.unwrap(), pipeline.attach, args.raw.clone())?;
            } else {
                prompt::run_prompt(args.raw)?;
            }
        }
        types::Commands::Stop(pipeline) => {
            // info!("Stopping pipline {:#?}", pipeline.name);
            if pipeline.name.is_some() {
                stop::stop(&pipeline.name.unwrap())?;
            }
        }
        types::Commands::Logs(logs) => match logs.commands {
            None => {
                let pipelines;
                if logs.display.name.is_some() {
                    pipelines = Logs::get_many_by_name(&logs.display.name.unwrap())?;
                } else {
                    pipelines = Logs::get()?;
                }
                if logs.display.json {
                    print::json(&pipelines)?;
                } else {
                    print::pretty(&pipelines)?;
                }
            }
            Some(logs_cmd) => match logs_cmd {
                types::LogsCommands::Rm(logs) => {
                    logger.lock().unwrap().clear()?;
                }
            },
        },
    }
    Ok(())
}
