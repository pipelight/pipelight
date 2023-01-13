// Cli core
pub mod actions;
pub mod print;
pub mod types;
use crate::run;
use clap::Parser;
use log::info;
use std::error::Error;
use utils::logger::Logger;

/// Execute the Command Line Tool (cli)
pub fn get_args() -> Result<(), Box<dyn Error>> {
    let args = types::Cli::parse();
    // Set verbosity
    let verbosity = args.verbose.log_level_filter();
    // Set logs (deprecated)
    //
    Logger::level(&verbosity);
    // Logger::new();
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
