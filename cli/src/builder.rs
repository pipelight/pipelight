// Struct
use crate::types::Cli;
// Clap - command line lib
use clap::FromArgMatches;
use clap::{builder::PossibleValue, Args, Command, ValueHint};
use clap_complete::{generate, shells};
// Standard I/O
use crate::globals::CLI;
use std::io;
// Error Handling
use crate::traits::from::string_to_command;
use log::trace;
use miette::Result;

impl Cli {
    /**
    The prefered way to build the cli.

    The Cli struct alone misses some commands that can't be created
    with the derived macros(#[clap()]...) on a simple struct.
    Building the Cli with the provided clap function
    allow for more fine grained customization.
    */
    pub fn build() -> Result<Command> {
        let mut cli = Command::new("pipelight");
        cli = Cli::augment_args(cli);
        cli = cli
            .mut_subcommand("logs", |a| {
                a.mut_arg("color", |e| {
                    e.num_args(0..=1)
                        .require_equals(true)
                        .default_missing_value("always")
                        .default_value("auto")
                })
            })
            .mut_arg("config", |e| e.value_hint(ValueHint::FilePath))
            .mut_subcommand("completion", |a| {
                a.mut_arg("name", |e| {
                    e.value_parser([
                        PossibleValue::new("bash"),
                        PossibleValue::new("zsh"),
                        PossibleValue::new("fish"),
                        PossibleValue::new("elvish"),
                    ])
                })
            });
        Ok(cli)
    }

    /**
    Prints the provided shell pipelight completion script
    on the standard stdout.
    */
    pub fn print_completion(shell: shells::Shell) -> Result<()> {
        // Build client and generate autocompletion script
        let mut cmd = Cli::build()?;
        let name = cmd.get_name().to_string();
        generate(shell, &mut cmd, name, &mut io::stdout());

        Ok(())
    }

    pub fn hydrate() -> Result<()> {
        let cli = Self::build()?;
        let matches = cli.get_matches();
        let args = Self::from_arg_matches(&matches)
            .map_err(|err| err.exit())
            .unwrap();

        // Hydrate global
        *CLI.lock().unwrap() = args;
        Ok(())
    }
}
