// Git Hooks
use utils::git::Hook;
// Colors
use workflow::traits::display::set_override;
// Structs
use cli::types::{
    Cli, ColoredOutput, Commands, DetachableCommands, LogsCommands, PostCommands, PreCommands,
};
use utils::git::Flag;
use workflow::{Config, Getters, Logs, Pipeline, Trigger};
// Actions
use actions::Action;
use clap::ValueEnum;
use clap_complete::shells::Shell;
// Template
use templates::Template;
// Error Handling
use log::info;
use miette::{Error, IntoDiagnostic, Result};
// Global vars
use crate::globals::{set_early_globals, set_globals};
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
                                // Set global config
                                match watch.toggle.clone() {
                                    None => {
                                        info!("Watching for changes");
                                        watcher::launch()?;
                                    }
                                    Some(toggle) => {
                                        if toggle.enable {
                                            watcher::launch()?;
                                        }
                                        if toggle.disable {
                                            watcher::kill()?;
                                        }
                                    }
                                }
                            }
                            DetachableCommands::Trigger(trigger) => {
                                info!("Triggering pipelines");
                                // trigger::launch(args.attach, trigger.flag)?;
                            }
                            DetachableCommands::Run(pipeline) => {
                                    info!("Running pipeline {:#?}", pipeline.name.clone().unwrap());
                                    Action::Run(pipeline)?;
                                }
                            }
                        }
                    }
                    PostCommands::Ls(list) => {
                        action::print::list(list);
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
                    PostCommands::Stop(pipeline) => Action::stop(pipeline),
                    PostCommands::Logs(logs) => {
                        // Set colors
                        if logs.display.color.is_some() {
                            match ColoredOutput::from(&logs.display.color.clone().unwrap()) {
                                ColoredOutput::Always => set_override(true),
                                ColoredOutput::Never => set_override(false),
                                ColoredOutput::Auto => {}
                            }
                        }

                        let mut _logs = Logs::new().hydrate()?;
                        match logs.commands.clone() {
                            None => {
                                let mut pipelines;
                                if logs.display.name.is_some() {
                                    pipelines = _logs
                                        .get_many_by_name(&logs.display.name.clone().unwrap())?;
                                } else {
                                    pipelines = _logs.get()?;
                                }
                                if logs.display.json {
                                    print::json(&pipelines)?;
                                } else {
                                    print::pretty(&mut pipelines)?;
                                }
                            }
                            Some(logs_cmd) => match logs_cmd {
                                LogsCommands::Rm => _logs.clean()?,
                            },
                        };
                    }
                }
            }
        }
        Ok(())
    }
}
