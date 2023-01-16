// Cli core
pub mod actions;
pub mod print;
pub mod types;
use crate::run;
use clap::Parser;
use log::info;
// use pipeline::types::Logs;
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
            // Pipeline::list()?;
        }
        types::Commands::Run(pipeline) => {
            info!("Triggering pipline {:#?}", pipeline.name);
            run::run_bin(pipeline.name)?;
        }
        types::Commands::Logs(logs) => {
            if logs.commands.is_some() {
                let logs_cmd = logs.commands.unwrap();
                match logs_cmd {
                    types::LogsCommands::Rm(logs) => {
                        logger.clear()?;
                    }
                }
            } else if logs.display.json {
                print::json()?;
            } else {
                print::pretty()?;
            }
        }
    }
    Ok(())
}
