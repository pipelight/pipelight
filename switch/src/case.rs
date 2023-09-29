// Git Hooks
use utils::git::Hook;

// Colors
use workflow::traits::display::set_override;

// Pipeline types
use workflow::{Config, Getters, Logs, Pipeline};

// Clap - command line lib
use clap::ValueEnum;
// use std::str::FromStr;

// Cli core types
use cli::types::{
    Cli, ColoredOutput, Commands, LogsCommands, PostCommands, PreCommands, WatchCommands,
};

// Cli core functions
use actions::*;

use crate::globals::{set_early_globals, set_globals, CLI};

use clap_complete::shells::Shell;

// Template
use templates::Template;
// Error Handling
use log::info;
use miette::{Error, Result};

pub struct Switch;

impl Switch {
    // Hydrate cli
    pub fn hydrate_global() -> Result<()> {
        let cli = Cli::build()?;
        let matches = cli.get_matches();
        let args = Cli::from_arg_matches(&matches)
            .map_err(|err| err.exit())
            .unwrap();
        unsafe { *CLI = args.clone() };
        Ok(())
    }
}

impl Switch {
    /// Build and Launch the cli
    pub fn launch() -> Result<()> {
        // Doesn't read config file
        set_early_globals()?;
        let args;
        unsafe {
            args = (*CLI).clone();
        };
        match args.commands {
            Commands::PreCommands(pre_commands) => {
                match pre_commands {
                    PreCommands::Completion(shell) => {
                        let shell = Shell::from_str(&shell.name, true);
                        if shell.is_ok() {
                            Cli::print_completion(shell.unwrap())?;
                        } else {
                            return Err(Error::msg("Couldn't determine shell"));
                        }
                    }
                    PreCommands::Init(init) => {
                        // create file
                        Template::new(init.template, init.file)?.create()?;
                    }
                    PreCommands::Hooks(toggle) => {
                        if toggle.enable {
                            Hook::enable()?;
                        }
                        if toggle.disable {
                            Hook::disable()?;
                        }
                    }
                }
            }
            Commands::PostCommands(post_commands) => {
                // Read config file
                set_globals()?;
                match post_commands {
                    PostCommands::Ls(list) => {
                        // Set global config
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
                    PostCommands::Inspect(list) => {
                        // Set global config
                        // info!("Listing piplines");
                        if list.name.is_some() {
                            let pipeline = Pipeline::get_by_name(&list.name.unwrap())?;
                            print::inspect(&pipeline, list.json)?;
                        } else {
                            prompt::inspect_prompt()?;
                        }
                    }
                    PostCommands::Watch(watch) => {
                        match watch.commands {
                            // Set global config
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
                    PostCommands::Trigger(trigger) => {
                        // Set global config
                        info!("Triggering pipelines");
                        trigger::launch(args.attach, trigger.flag)?;
                    }
                    PostCommands::Run(pipeline) => {
                        // Set global config
                        if pipeline.name.is_some() {
                            info!("Running pipeline {:#?}", pipeline.name.clone().unwrap());
                            run::launch(
                                pipeline.name.unwrap(),
                                args.attach,
                                pipeline.trigger.flag,
                            )?;
                        } else {
                            prompt::run_prompt(args.attach, pipeline.trigger.flag)?;
                        }
                    }
                    PostCommands::Stop(pipeline) => {
                        // Set global config
                        if pipeline.name.is_some() {
                            info!(
                        "Stopping pipeline {:#?} with every attached and detached subprocess",
                        pipeline.name
                    );
                            stop::stop(&pipeline.name.unwrap())?;
                        } else {
                            prompt::stop_prompt()?;
                        }
                    }
                    PostCommands::Logs(logs) => {
                        // Set colors
                        if logs.display.color.is_some() {
                            match ColoredOutput::from(&logs.display.color.unwrap()) {
                                ColoredOutput::Always => set_override(true),
                                ColoredOutput::Never => set_override(false),
                                ColoredOutput::Auto => {}
                            }
                        }

                        match logs.commands {
                            None => {
                                let mut pipelines;
                                if logs.display.name.is_some() {
                                    pipelines =
                                        Logs::get_many_by_name(&logs.display.name.unwrap())?;
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
                                LogsCommands::Rm => Logs::clean()?,
                            },
                        };
                    }
                }
            }
        }
        Ok(())
    }
}
