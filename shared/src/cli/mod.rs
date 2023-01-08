// Cli core
pub mod actions;
pub mod print;
pub mod types;
use crate::run;
use clap::{Args, Parser, Subcommand, ValueEnum};
use log::info;
use pipeline::types::Pipelines;
use std::error::Error;
use utils::logger::Logs;

pub fn get_args() -> Result<(), Box<dyn Error>> {
    let args = types::Cli::parse();
    let verbosity = args.verbose.log_level_filter();
    Logs::new().set(&verbosity);
    match args.command {
        types::Commands::Ls(list) => {
            info!("Listing piplines");
            Pipelines::list()?;
        }
        types::Commands::Run(pipeline) => {
            info!("Triggering pipline {:#?}", pipeline.name);
            run::run_bin(pipeline.name)?;
        }
        types::Commands::Logs(logs) => {
            if logs.json {
                print::json()?;
            } else if logs.json {
                Logs::clear()?
            } else {
                print::pretty()?;
            }
        }
    }
    Ok(())
}
