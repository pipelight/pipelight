// Struct
use crate::actions::{logs, pipeline, prompt, run, stop, trigger, watch};
use crate::services::types::{Action, Service};
use crate::types::Cli;
use crate::types::{ColoredOutput, LogsCommands, ToggleCommands};
use crate::types::{Commands, DetachableCommands, PostCommands, PreCommands};
use workflow::{Getters, Pipeline};

use utils::git::Hook;
// Clap
use clap::ValueEnum;
use clap_complete::shells::Shell;
// Template
use templates::Template;
// Colors
use colored::control::set_override;
// Traits
use crate::services::traits::FgBg;
// Error Handling
use miette::{Error, Result};
// Globals
use crate::globals::CLI;

impl Commands {
    pub fn start(&self) -> Result<()> {
        match self {
            Commands::PreCommands(e) => {
                e.start()?;
            }
            Commands::PostCommands(e) => {
                e.start()?;
            }
        }
        Ok(())
    }
}

impl PreCommands {
    pub fn start(&self) -> Result<()> {
        match self {
            PreCommands::Completion(shell) => {
                let shell = Shell::from_str(&shell.name, true);
                if shell.is_ok() {
                    Cli::print_completion(shell.unwrap())?;
                } else {
                    return Err(Error::msg("Couldn't determine shell"));
                }
            }
            PreCommands::Init(e) => {
                // create template pipeline file
                let template = Template::new(e.template.clone(), e.file.clone())?;
                template.create()?;
                // Create gitignore file
                template.create_ignore()?;
            }
            PreCommands::Enable(e) => {
                if let Some(commands) = e.commands.clone() {
                    let args = CLI.lock().unwrap().clone();
                    match commands {
                        ToggleCommands::GitHooks => Hook::enable()?,
                        ToggleCommands::Watcher => {
                            let mut service = Service::new(Action::Watch, Some(args))?;
                            service.should_detach()?;
                        }
                    }
                }
            }
            PreCommands::Disable(e) => {
                if let Some(commands) = e.commands.clone() {
                    match commands {
                        ToggleCommands::GitHooks => Hook::disable()?,
                        ToggleCommands::Watcher => watch::Watcher::kill()?,
                    }
                }
            }
        }
        Ok(())
    }
}

impl DetachableCommands {
    pub fn start(&mut self) -> Result<()> {
        let mut args = CLI.lock().unwrap().clone();
        match self {
            DetachableCommands::Run(e) => {
                if e.name.is_none() {
                    e.name = Some(prompt::pipeline()?);
                    args.commands = Commands::PostCommands(PostCommands::DetachableCommands(
                        DetachableCommands::Run(e.to_owned()),
                    ))
                }
                if let Some(name) = e.name.clone() {
                    // Usefull SafeGuard that early returns fancy error
                    // if pipeline name not found
                    // on detach mode
                    let pipeline = Pipeline::get_by_name(&name)?;
                    pipeline.is_triggerable()?;

                    Service::new(Action::RunLoose, Some(args))?.attach()?;
                }
            }
            DetachableCommands::Watch => {
                Service::new(Action::Watch, Some(args))?.should_detach()?;
            }
            DetachableCommands::Trigger(..) => {
                Service::new(Action::Trigger, Some(args))?.should_detach()?
            }
        }
        Ok(())
    }
}

impl PostCommands {
    pub fn start(&self) -> Result<()> {
        match self {
            PostCommands::Stop(e) => {
                if let Some(name) = e.name.clone() {
                    stop::launch(&name)?;
                } else {
                    // Select prompt
                    let name = prompt::running_pipeline()?;
                    stop::launch(&name)?;
                }
            }
            PostCommands::Logs(e) => {
                if let Some(commands) = e.commands.clone() {
                    match commands {
                        LogsCommands::Rm => {
                            logs::clean()?;
                            return Ok(());
                        }
                    };
                }
                // Set colors
                if e.display.color.is_some() {
                    match ColoredOutput::from(&e.display.color.clone().unwrap()) {
                        ColoredOutput::Always => set_override(true),
                        ColoredOutput::Never => set_override(false),
                        ColoredOutput::Auto => {}
                    }
                }
                if e.display.json {
                    logs::json(e.display.name.clone())?;
                } else {
                    logs::pretty(e.display.name.clone())?;
                }
            }
            PostCommands::Ls(e) => {
                if e.name.is_some() {
                    if e.json {
                        pipeline::json(e.name.clone())?;
                    } else {
                        pipeline::pretty(e.name.clone())?;
                    }
                } else {
                    pipeline::default()?;
                }
            }
            PostCommands::Inspect(e) => {
                if let Some(name) = e.name.clone() {
                    pipeline::inspect(&name, e.json)?;
                } else {
                    // Select prompt
                    let name = prompt::pipeline()?;
                    pipeline::inspect(&name, e.json)?;
                }
            }
            PostCommands::DetachableCommands(e) => {
                e.clone().start()?;
            }
        };
        Ok(())
    }
}
