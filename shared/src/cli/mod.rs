// Cli core
pub mod actions;
pub mod types;
use crate::logger;
use crate::types::Config;
use clap::{Args, Parser, Subcommand, ValueEnum};
use log::{debug, error, info, trace, warn};
use std::error::Error;

pub fn get_args() -> Result<(), Box<dyn Error>> {
    let args = types::Cli::parse();
    let verbosity = args.verbose.log_level_filter();
    logger::set_logger(verbosity)?;

    match args.command {
        types::Commands::Run(pipeline) => {
            debug!("Triggering pipline {:#?}", pipeline.name);
            actions::run(pipeline.name);
        }
        types::Commands::Stop(pipeline) => {
            debug!("Stopping pipline {:#?}", pipeline.name);
            actions::stop();
        }
        types::Commands::Logs(logs) => {
            debug!("Display logs");
            if logs.pretty {
                actions::pretty_logs()?
            }
            if logs.json {
                actions::json_logs()
            } else {
                actions::raw_logs()
            }
        }
        types::Commands::Ls(list) => {
            debug!("Listing piplines");
            actions::list();
        }
    }
    Ok(())
}
