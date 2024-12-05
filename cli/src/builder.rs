// Struct
use crate::types::{Cli, Commands};
// Clap - command line lib
use clap::FromArgMatches;
use clap::{builder::PossibleValue, Args, Command, ValueHint};
use clap_complete::{generate, shells};
// Standard I/O
use crate::globals::CLI;
use std::io;
// Error Handling
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
            .mut_arg("attach", |e| {
                e.num_args(0..=1)
                    .require_equals(true)
                    .default_missing_value("true")
            })
            .mut_subcommand("completion", |a| {
                a.mut_arg("name", |e| {
                    e.value_parser([
                        PossibleValue::new("bash"),
                        PossibleValue::new("zsh"),
                        PossibleValue::new("fish"),
                        PossibleValue::new("elvish"),
                    ])
                })
            })
            .mut_subcommand("init", |a| {
                a.mut_arg("template", |e| {
                    e.value_parser([
                        PossibleValue::new("toml"),
                        PossibleValue::new("yaml"),
                        PossibleValue::new("hcl"),
                        PossibleValue::new("js"),
                        PossibleValue::new("ts"),
                        PossibleValue::new("ts_helpers"),
                        PossibleValue::new("json"),
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

    pub fn string_to_command(e: &str) -> Result<Commands> {
        let os_str: Vec<&str> = e.split(' ').collect();
        let cli = Cli::build()?;
        let matches = cli.get_matches_from(os_str);
        let args = Cli::from_arg_matches(&matches)
            .map_err(|err| err.exit())
            .unwrap();
        let command: Commands = args.commands;
        Ok(command)
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
