//mods
pub mod print;
pub mod prompt;
pub mod types;

// Cli core
use crate::run;
use crate::stop;
use crate::trigger;
use crate::watch;
use clap::Parser;
use log::info;

// Error Handling
use miette::{miette, Diagnostic, Error, IntoDiagnostic, NamedSource, Report, Result, SourceSpan};
// use std::error::Error;

// Logger
use utils::logger::logger;

// Global var
use once_cell::sync::Lazy;
use std::ops::{Deref, DerefMut};
use std::sync::{Arc, Mutex, RwLock};

use pipeline::types::{traits::getters::Getters, Config, Logs, Pipeline};

// Global var
pub static mut ARGS: Lazy<Vec<String>> = Lazy::new(|| vec![]);

/// Launch the cli
// Initialize Logger and program global vars (Config, Args)
pub fn get_args(raw_args: Vec<String>) -> Result<()> {
    let args = types::Cli::parse();

    // Set globals args
    unsafe {
        *ARGS = raw_args;
    }

    // Set verbosity level
    let verbosity = args.verbose.log_level_filter();
    logger.lock().unwrap().level(&verbosity);

    // Set global config
    Config::new(args.config.clone(), args.raw.clone())?;

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
        types::Commands::Watch(watch) => {
            // info!("Triggering piplines");
            watch::watch_bin(watch.attach)?;
        }
        types::Commands::Trigger(trigger) => {
            // info!("Triggering piplines");
            trigger::trigger_bin(trigger.attach, trigger.flag)?;
        }
        types::Commands::Run(pipeline) => {
            // info!("Running pipline {:#?}", pipeline.name);
            if pipeline.name.is_some() {
                run::run_bin(pipeline.name.unwrap(), pipeline.attach)?;
            } else {
                prompt::run_prompt(pipeline.attach)?;
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
