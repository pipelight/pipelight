mod typings;
// use crate::cli::typings::{Cli, Command};
use clap::{Args, Parser, Subcommand, ValueEnum};

pub fn get_args() {
    let args = typings::Cli::parse();
    match args.command {
        typings::Commands::Pipe(remote) => {
            println!("Cloning {:#?}", remote);
        }
        typings::Commands::Logs(remote) => {
            println!("Cloning {:#?}", remote);
        }
    }
}
