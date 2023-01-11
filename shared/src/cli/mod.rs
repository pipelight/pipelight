// Cli core
pub mod actions;
pub mod print;
pub mod types;
use crate::run;
use clap::{Args, Parser, Subcommand, ValueEnum};
use log::info;
use pipeline::types::Pipeline;
use std::error::Error;
use utils::log::Logs;

/// Execute the Command Line Tool (cli)
pub fn get_args() -> Result<(), Box<dyn Error>> {
    let args = types::Cli::parse();
    // Set verbosity
    let verbosity = args.verbose.log_level_filter();
    // Set logs (deprecated)
    Logs::new().set(&verbosity);
    // Set config
    //
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
                        // Logs::remove()?;
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
