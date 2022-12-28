// Cli core
pub mod actions;
pub mod types;
use crate::logger::Logs;
use clap::{Args, Parser, Subcommand, ValueEnum};
use log::info;
use std::error::Error;

pub fn get_args() -> Result<(), Box<dyn Error>> {
    let args = types::Cli::parse();
    let verbosity = args.verbose.log_level_filter();
    Logs::new().set()?;

    match args.command {
        types::Commands::Run(pipeline) => {
            info!("Triggering pipline {:#?}", pipeline.name);
            actions::run(pipeline.name)?;
        }
        types::Commands::Stop(pipeline) => {
            info!("Stopping pipline {:#?}", pipeline.name);
            actions::stop();
        }
        types::Commands::Logs(logs) => {
            if logs.json {
                Logs::json()?
            } else if logs.clear {
                Logs::clear()?
            } else {
                Logs::pretty()?
            }
        }
        types::Commands::Ls(list) => {
            info!("Listing piplines");
            actions::list()?;
        }
        types::Commands::Lint(lint) => {
            actions::lint()?;
        }
        types::Commands::Init(init) => {
            info!("Ensure working tree");
            actions::init()?;
        }
    }
    Ok(())
}
