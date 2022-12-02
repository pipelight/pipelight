// Cli core

mod typings;
use crate::actions::*;
use clap::{Args, Parser, Subcommand, ValueEnum};
use log::{debug, error, info, trace, warn};

pub fn get_args() {
    let args = typings::Cli::parse();
    match args.command {
        typings::Commands::Run(pipeline) => {
            debug!("Triggering pipline {:#?}", pipeline.name);
        }
        typings::Commands::Stop(remote) => {
            debug!("Stopping pipeline {:#?}", remote);
        }
        typings::Commands::Logs(logs) => {
            println!("Display logs");
        }
        typings::Commands::Ls(list) => {
            // actions::list();
            println!("List pipelines");
        }
    }
}
