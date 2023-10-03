// Git Hooks
use utils::git::Hook;
// Colors
use workflow::traits::display::set_override;
// Structs
use cli::types::{
    Cli, ColoredOutput, Commands, DetachableCommands, LogsCommands, PostCommands, PreCommands,
    WatchCommands,
};
use utils::git::Flag;
use workflow::{Config, Getters, Logs, Pipeline, Trigger};
// Actions
use crate::globals::{set_early_globals, set_globals};
use actions::*;
use clap::ValueEnum;
use clap_complete::shells::Shell;
// Template
use templates::Template;
// Error Handling
use log::info;
use miette::{Error, IntoDiagnostic, Result};
// Global vars
use cli::globals::CLI;

pub struct Switch;
impl Switch {
    /// Build and Launch the cli
    pub fn case() -> Result<()> {
        // Doesn't read config file
        set_early_globals()?;
        // let args;
        let mut args = CLI.lock().unwrap().clone();
        match &mut args.commands {
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
                        Template::new(init.template.clone(), init.file.clone())?.create()?;
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
                    PostCommands::DetachableCommands(detachable_commands) => {
                        match detachable_commands {
                            DetachableCommands::Watch(watch) => {
                                match watch.commands.clone() {
                                    // Set global config
                                    None => {
                                        info!("Watching for changes");
                                        // watch::launch(args.attach)?;
                                    }
                                    Some(watch_cmd) => match watch_cmd {
                                        WatchCommands::Kill => {
                                            // watch::destroy_watcher()?;
                                        }
                                    },
                                }
                            }
                            DetachableCommands::Trigger(trigger) => {
                                // Set global config
                                info!("Triggering pipelines");
                                // trigger::launch(args.attach, trigger.flag)?;
                            }
                            DetachableCommands::Run(pipeline) => {
                                if pipeline.name.is_some() {
                                    info!("Running pipeline {:#?}", pipeline.name.clone().unwrap());
                                    run::launch(&pipeline.name.as_ref().unwrap())?;
                                } else {
                                    prompt::run_prompt()?;
                                }
                            }
                        }
                    }
                    PostCommands::Ls(list) => {
                        // Set global config
                        // Launch watcher
                        // if Config::get()?.has_watch_flag().is_ok() {
                        // watch::create_watcher()?;
                        // } else {
                        // watch::destroy_watcher()?;
                        // }
                        if list.name.is_some() {
                            let pipeline = Pipeline::get_by_name(&list.name.clone().unwrap())?;
                            print::inspect(&pipeline, list.json)?;
                        } else {
                            print::list()?;
                        }
                    }
                    PostCommands::Inspect(list) => {
                        // Set global config
                        // info!("Listing piplines");
                        if list.name.is_some() {
                            let pipeline = Pipeline::get_by_name(&list.name.clone().unwrap())?;
                            print::inspect(&pipeline, list.json)?;
                        } else {
                            prompt::inspect_prompt()?;
                        }
                    }
                    PostCommands::Stop(pipeline) => {
                        // Set global config
                        if pipeline.name.is_some() {
                            info!(
                        "Stopping pipeline {:#?} with every attached and detached subprocess",
                        pipeline.name
                    );
                            stop::stop(&pipeline.name.clone().unwrap())?;
                        } else {
                            prompt::stop_prompt()?;
                        }
                    }
                    PostCommands::Logs(logs) => {
                        // Set colors
                        if logs.display.color.is_some() {
                            match ColoredOutput::from(&logs.display.color.clone().unwrap()) {
                                ColoredOutput::Always => set_override(true),
                                ColoredOutput::Never => set_override(false),
                                ColoredOutput::Auto => {}
                            }
                        }

                        match logs.commands.clone() {
                            None => {
                                let mut pipelines;
                                if logs.display.name.is_some() {
                                    pipelines = Logs::get_many_by_name(
                                        &logs.display.name.clone().unwrap(),
                                    )?;
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
