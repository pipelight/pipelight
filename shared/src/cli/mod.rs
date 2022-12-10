// Cli core

mod types;
use crate::actions;
use crate::types::Config;
use clap::{Args, Parser, Subcommand, ValueEnum};
use log::{debug, error, info, trace, warn};

pub fn get_args() {
    let args = types::Cli::parse();
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
            trace!("Display logs");
            actions::logs();
        }
        types::Commands::Ls(list) => {
            trace!("Listing piplines");
            actions::list();
        }
    }
}
