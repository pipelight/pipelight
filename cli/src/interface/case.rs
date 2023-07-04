use super::print;
use super::prompt;
use super::types::{LogsCommands, WatchCommands};
use super::CLI;

// Error Handling
use miette::Result;

// Logger
use log::info;
use utils::logger::logger;

// Pipeline types
use pipeline::{Config, Getters, Logs, Pipeline};

// Clap - command line lib
use super::types::{Cli, Commands};
use clap::Parser;

// Cli core functions
use crate::actions::run;
use crate::actions::stop;
use crate::actions::trigger;
use crate::actions::watch;

/// Launch the cli
pub fn get_args() -> Result<()> {
    // Autocompletion
    // traits::autocomplete::make_completion()?;

    // Parse args from command line and hydrate globals
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
            // Launch watcher
            if Config::get()?.has_watch_flag().is_ok() {
                watch::create_watcher()?;
            } else {
                watch::destroy_watcher()?;
            }

            if list.name.is_some() {
                let pipeline = Pipeline::get_by_name(&list.name.unwrap())?;
                print::inspect(&pipeline, list.json)?;
            } else {
                print::list()?;
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
        Commands::Watch(watch) => match watch.commands {
            None => {
                info!("Watching for changes");
                watch::launch(args.attach)?;
            }
            Some(watch_cmd) => match watch_cmd {
                WatchCommands::Kill => {
                    watch::destroy_watcher();
                }
            },
        },
        Commands::Trigger(trigger) => {
            info!("Triggering pipelines");
            trigger::launch(args.attach, trigger.flag)?;
        }
        Commands::Run(pipeline) => {
            if pipeline.name.is_some() {
                info!("Running pipeline {:#?}", pipeline.name.clone().unwrap());
                run::launch(pipeline.name.unwrap(), args.attach, pipeline.trigger.flag)?;
            } else {
                prompt::run_prompt(args.attach, pipeline.trigger.flag)?;
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
                let mut pipelines;
                if logs.display.name.is_some() {
                    pipelines = Logs::get_many_by_name(&logs.display.name.unwrap())?;
                } else {
                    pipelines = Logs::get()?;
                }
                if logs.display.json {
                    print::json(&pipelines)?;
                } else {
                    print::pretty(&mut pipelines)?;
                }
            }
            Some(logs_cmd) => match logs_cmd {
                LogsCommands::Rm => {
                    logger.lock().unwrap().clear()?;
                }
            },
        },
    }
    Ok(())
}
