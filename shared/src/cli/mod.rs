// Cli core
pub mod actions;
pub mod print;
pub mod types;
use clap::{Args, Parser, Subcommand, ValueEnum};
use log::info;
use std::error::Error;
use utils::logger::Logs;

pub fn get_args() -> Result<(), Box<dyn Error>> {
    let args = types::Cli::parse();
    let verbosity = args.verbose.log_level_filter();
    Logs::new().set(&verbosity);
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
                print::json()?;
            } else if logs.clear {
                Logs::clear()?
            } else {
                print::pretty()?;
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
