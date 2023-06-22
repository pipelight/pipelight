//mods
pub mod print;
pub mod prompt;
pub mod traits;
pub mod types;
pub mod verbosity;

// Use types
use types::Cli;

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
/// Hydrate global var ARGS
pub fn hydrate(mut args: Lazy<Vec<String>>) -> Result<()> {
    use std::env;
    let raw_args = env::args().collect::<Vec<String>>();
    *args = raw_args;
    Ok(())
}

// Command line args conversion
pub fn args_to_cli() -> Result<Option<Cli>> {
    let mut args_vec: Vec<String>;
    unsafe {
        args_vec = (*ARGS).clone();
        args_vec.remove(0);
    }
    let raw = args_vec.join(" ").to_owned();
    let res = serde_json::from_str::<Cli>(&raw);
    Ok(res.into_diagnostic().ok())
}

pub fn cli_to_args() -> Result<String> {
    let parsed;
    unsafe {
        parsed = Cli::try_parse_from((*ARGS).clone()).into_diagnostic()?;
    }
    let json = serde_json::to_string::<Cli>(&parsed).into_diagnostic()?;
    Ok(json)
}

/// Launch the cli
// Initialize Logger and program global vars (Config, Args)
pub fn get_args(some_args: Option<Cli>) -> Result<()> {
    let args: Cli;
    if some_args.is_none() {
        args = Cli::parse();
    } else {
        args = some_args.unwrap();
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
                run::run_bin(pipeline.name.unwrap(), pipeline.trigger.attach)?;
            } else {
                prompt::run_prompt(pipeline.trigger.attach)?;
            }
        }
        types::Commands::Stop(pipeline) => {
            // info!("Stopping pipline {:#?}", pipeline.name);
            if pipeline.name.is_some() {
                stop::stop(&pipeline.name.unwrap())?;
            }
        }
        types::Commands::Raw(raw) => {
            // info!("Stopping pipline {:#?}", pipeline.name);
            let cli = serde_json::from_str::<types::Cli>(&raw.string).into_diagnostic()?;
            let bin = "pipelight raw";

            #[cfg(debug_assertions)]
            // let command = format!("cargo run --bin {} {}", &bin, &args.clone().unwrap());
            #[cfg(not(debug_assertions))]
            // let command = format!("{} {}", &bin, &args.unwrap());

            // Exec::new().detached(&command)?;
            get_args(Some(cli));
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
                types::LogsCommands::Rm => {
                    logger.lock().unwrap().clear()?;
                }
            },
        },
    }
    Ok(())
}
