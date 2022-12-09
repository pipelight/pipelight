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
            // actions::run();
            debug!("Triggering pipline {:#?}", pipeline.name);
            println!("Triggering pipline {:#?}", pipeline.name);
        }
        types::Commands::Stop(remote) => actions::stop(),
        types::Commands::Logs(logs) => {
            actions::logs();
        }
        types::Commands::Ls(list) => {
            actions::list();
        }
    }
}
