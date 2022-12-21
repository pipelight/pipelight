// Cli core
pub mod actions;
pub mod types;
use crate::logger;
use crate::types::config::Config;
use clap::{Args, Parser, Subcommand, ValueEnum};
use log::info;
use std::error::Error;

pub fn get_args() -> Result<(), Box<dyn Error>> {
    let args = types::Cli::parse();
    let verbosity = args.verbose.log_level_filter();
    logger::set_logger(verbosity)?;

    match args.command {
        types::Commands::Run(pipeline) => {
            info!("Triggering pipline {:#?}", pipeline.name);
            actions::run(pipeline.name);
        }
        types::Commands::Stop(pipeline) => {
            info!("Stopping pipline {:#?}", pipeline.name);
            actions::stop();
        }
        types::Commands::Logs(logs) => {
            info!("Display logs");
            if logs.pretty {
                actions::pretty_logs()?
            } else if logs.json {
                actions::json_logs()
            } else if logs.clear {
                actions::clear_logs()?
            } else {
                actions::raw_logs()
            }
        }
        types::Commands::Ls(list) => {
            info!("Listing piplines");
            actions::list();
        }
        types::Commands::Lint(lint) => {
            actions::lint();
        }
        types::Commands::Init(init) => {
            info!("Ensure working tree");
            actions::init();
        }
    }
    Ok(())
}
