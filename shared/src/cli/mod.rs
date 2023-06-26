//modules
pub mod print;
pub mod prompt;
pub mod traits;
pub mod types;

use pipeline::{Config, Getters, Logs, Pipeline};

// Clap - command line lib
use clap::Parser;
use types::{Cli, Commands};

// Cli core functions
use crate::run;
use crate::stop;
// use crate::trigger;
// use crate::watch;

// Error Handling
use miette::Result;

// Logger
use log::info;
use utils::logger::logger;

// Global vars
use once_cell::sync::Lazy;

pub static mut CLI: Lazy<Cli> = Lazy::new(|| Cli::new());

/// Launch the cli
pub fn get_args() -> Result<()> {
    // Parse args from command line
    // and hydrate global var
    let args = Cli::parse();
    unsafe { *CLI = args.clone() };

    // Set verbosity level
    let verbosity = args.verbose.log_level_filter();
    logger.lock().unwrap().level(&verbosity);

    // Set global config
    Config::new(args.config.clone(), args.raw.clone())?;

    match args.commands {
        Commands::Ls(list) => {
            // info!("Listing piplines");
            if list.name.is_some() {
                let pipeline = Pipeline::get_by_name(&list.name.unwrap())?;
                print::inspect(&pipeline, list.json)?;
            } else {
                print::list();
            }
        }
        Commands::Inspect(list) => {
            // info!("Listing piplines");
            if list.name.is_some() {
                let pipeline = Pipeline::get_by_name(&list.name.unwrap())?;
                print::inspect(&pipeline, list.json)?;
            } else {
                prompt::inspect_prompt()?;
            }
        }
        Commands::Watch => {
            info!("Watching for changes");
            // watch::watch_bin()?;
        }
        Commands::Trigger(trigger) => {
            info!("Triggering pipelines");
            // trigger::trigger_bin(trigger.flag)?;
        }
        Commands::Run(pipeline) => {
            if pipeline.name.is_some() {
                info!("Running pipeline {:#?}", pipeline.name.clone().unwrap());
                run::run_bin(pipeline.name.unwrap(), args.attach)?;
            } else {
                prompt::run_prompt(args.attach)?;
            }
        }
        Commands::Stop(pipeline) => {
            info!(
                "Stopping pipeline {:#?} with every attached and detached subprocess",
                pipeline.name
            );
            if pipeline.name.is_some() {
                stop::stop(&pipeline.name.unwrap())?;
            }
        }
        Commands::Logs(logs) => match logs.commands {
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
                types::LogsCommands::Rm => {
                    logger.lock().unwrap().clear()?;
                }
            },
        },
    }
    Ok(())
}
