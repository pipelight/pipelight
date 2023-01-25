// Cli core
pub mod print;
pub mod types;
use crate::run;
use crate::stop;
use crate::trigger;
use clap::Parser;
use log::info;
use pipeline::types::{Config, Logs, Pipeline, Status};
use std::error::Error;
use utils::logger::logger;

/// Execute the Command Line Tool (cli)
/// Initialize Logger and program global vars
pub fn get_args() -> Result<(), Box<dyn Error>> {
    let args = types::Cli::parse();
    // Set verbosity
    let verbosity = args.verbose.log_level_filter();
    logger.level(&verbosity);

    match args.commands {
        types::Commands::Ls(list) => {
            info!("Listing piplines");
            print::list()?;
        }
        types::Commands::Trigger(trigger) => {
            info!("Triggering piplines");
            trigger::trigger_bin(trigger.attach)?;
        }
        types::Commands::Run(pipeline) => {
            info!("Running pipline {:#?}", pipeline.name);
            run::run_bin(pipeline.name, pipeline.attach)?;
        }
        types::Commands::Stop(pipeline) => {
            info!("Stopping pipline {:#?}", pipeline.name);
            stop::stop(&pipeline.name)?;
        }
        types::Commands::Logs(logs) => match logs.commands {
            None => {
                let pipelines;
                if logs.display.name.is_some() {
                    pipelines = Logs::get_by_name(&logs.display.name.unwrap())?;
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
                    logger.clear()?;
                }
            },
        },
    }
    Ok(())
}
