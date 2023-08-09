// Error Handling
use miette::{Error, IntoDiagnostic, Result};

// Logger
use log::info;
use utils::logger::logger;

// Colors
use pipeline::display::set_override;

// Pipeline types
use pipeline::{Config, Getters, Logs, Pipeline};

// Clap - command line lib
use clap::{
    builder::PossibleValue, Args, Command, CommandFactory, FromArgMatches, ValueEnum, ValueHint,
};
// use std::str::FromStr;

// Cli core types
use crate::interface::{Cli, ColoredOutput, Commands, LogsCommands, WatchCommands};
// Cli core functions
use crate::actions::print;
use crate::actions::prompt;
use crate::actions::run;
use crate::actions::stop;
use crate::actions::trigger;
use crate::actions::watch;

use clap_complete::{generate, shells::Shell};
use std::io;

// Global vars
use once_cell::sync::Lazy;

pub static mut CLI: Lazy<Cli> = Lazy::new(Cli::new);

pub struct Client;

impl Client {
    pub fn print_completion(shell: Shell) -> Result<()> {
        // Build client and generate autocompletion script
        let mut cmd = Client::build()?;
        let name = cmd.get_name().to_string();
        generate(shell, &mut cmd, name, &mut io::stdout());

        Ok(())
    }
    /// Build the cli
    pub fn build() -> Result<Command> {
        let mut cli = Command::new("pipelight");
        cli = Cli::augment_args(cli);
        cli = cli
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
            .mut_arg("config", |e| e.value_hint(ValueHint::FilePath))
            .mut_subcommand("logs", |a| {
                a.mut_arg("color", |e| e.default_missing_value("always"))
            });
        Ok(cli)
    }
    /// Build and Launch the cli
    pub fn launch() -> Result<()> {
        let cli = Client::build()?;
        let matches = cli.get_matches();
        let args = Cli::from_arg_matches(&matches)
            .map_err(|err| err.exit())
            .unwrap();
        unsafe { *CLI = args.clone() };

        // Set verbosity level
        let verbosity = args.verbose.log_level_filter();
        logger.lock().unwrap().level(&verbosity);

        match args.commands {
            Commands::Ls(list) => {
                // Set global config
                Config::new(args.config.clone(), args.raw.clone())?;
                // Launch watcher
                if Config::get()?.has_watch_flag().is_ok() {
                    watch::create_watcher()?;
                } else {
                    watch::destroy_watcher()?;
                }
                if list.name.is_some() {
                    let pipeline = Pipeline::get_by_name(&list.name.unwrap())?;
                    print::inspect(&pipeline, list.json)?;
                } else {
                    print::list()?;
                }
            }
            Commands::Inspect(list) => {
                // Set global config
                Config::new(args.config.clone(), args.raw.clone())?;
                // info!("Listing piplines");
                if list.name.is_some() {
                    let pipeline = Pipeline::get_by_name(&list.name.unwrap())?;
                    print::inspect(&pipeline, list.json)?;
                } else {
                    prompt::inspect_prompt()?;
                }
            }
            Commands::Watch(watch) => {
                // Set global config
                Config::new(args.config.clone(), args.raw.clone())?;
                match watch.commands {
                    None => {
                        info!("Watching for changes");
                        watch::launch(args.attach)?;
                    }
                    Some(watch_cmd) => match watch_cmd {
                        WatchCommands::Kill => {
                            watch::destroy_watcher()?;
                        }
                    },
                }
            }
            Commands::Trigger(trigger) => {
                // Set global config
                Config::new(args.config.clone(), args.raw.clone())?;
                info!("Triggering pipelines");
                trigger::launch(args.attach, trigger.flag)?;
            }
            Commands::Run(pipeline) => {
                // Set global config
                Config::new(args.config.clone(), args.raw.clone())?;
                if pipeline.name.is_some() {
                    info!("Running pipeline {:#?}", pipeline.name.clone().unwrap());
                    run::launch(pipeline.name.unwrap(), args.attach, pipeline.trigger.flag)?;
                } else {
                    prompt::run_prompt(args.attach, pipeline.trigger.flag)?;
                }
            }
            Commands::Stop(pipeline) => {
                // Set global config
                Config::new(args.config.clone(), args.raw.clone())?;
                info!(
                    "Stopping pipeline {:#?} with every attached and detached subprocess",
                    pipeline.name
                );
                if pipeline.name.is_some() {
                    stop::stop(&pipeline.name.unwrap())?;
                }
            }
            Commands::Completion(shell) => {
                let shell = Shell::from_str(&shell.name, true);
                if shell.is_ok() {
                    Client::print_completion(shell.unwrap())?;
                } else {
                    return Err(Error::msg("Couldn't determine shell"));
                }
            }
            Commands::Logs(logs) => {
                // Set colors
                if logs.display.color.is_some() {
                    match ColoredOutput::from(&logs.display.color.unwrap()) {
                        ColoredOutput::Always => set_override(true),
                        ColoredOutput::Never => set_override(false),
                    }
                }

                match logs.commands {
                    None => {
                        let mut pipelines;
                        if logs.display.name.is_some() {
                            pipelines = Logs::get_many_by_name(&logs.display.name.unwrap())?;
                        } else {
                            pipelines = Logs::get()?;
                        }
                        if logs.display.json {
                            print::json(&pipelines)?;
                        } else {
                            print::pretty(&mut pipelines)?;
                        }
                    }
                    Some(logs_cmd) => match logs_cmd {
                        LogsCommands::Rm => {
                            logger.lock().unwrap().clear()?;
                        }
                    },
                };
            }
        }
        Ok(())
    }
}
