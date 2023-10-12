use crate::types::Cli;
use crate::types::{ColoredOutput, LogsCommands};
use crate::types::{Commands, DetachableCommands, PostCommands, PreCommands};
use actions::{print, prompt, run, stop, trigger, watch};
use utils::git::Hook;
// Clap
use clap::ValueEnum;
use clap_complete::shells::Shell;
// Template
use templates::Template;
// Colors
use colored::control::set_override;
use colored::{ColoredString, Colorize};
// Error Handling
use miette::{Error, Result};

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
                // create file
                Template::new(e.template.clone(), e.file.clone())?.create()?;
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
        Ok(())
    }
}

impl DetachableCommands {
    pub fn start(&self) -> Result<()> {
        match self {
            DetachableCommands::Run(e) => {
                if let Some(name) = e.name.clone() {
                    run::launch(&name)?;
                } else {
                    // Select prompt
                    let name = prompt::pipeline()?;
                    run::launch(&name)?;
                }
            }
            DetachableCommands::Trigger(e) => {
                if let Some(e) = e.flag.clone() {
                    // Set global args flag ??
                }
                trigger::launch()?;
            }
            DetachableCommands::Watch(e) => {
                watch::Watcher::start()?;
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
                    let name = prompt::pipeline()?;
                    stop::launch(&name)?;
                }
            }
            PostCommands::Logs(e) => {
                // Set colors
                if e.display.color.is_some() {
                    match ColoredOutput::from(&e.display.color.clone().unwrap()) {
                        ColoredOutput::Always => set_override(true),
                        ColoredOutput::Never => set_override(false),
                        ColoredOutput::Auto => {}
                    }
                }
                if e.display.json {
                    actions::print::logs::json(e.display.name.clone())?;
                } else {
                    actions::print::logs::pretty(e.display.name.clone())?;
                }
            }
            PostCommands::Ls(e) => {
                if e.name.is_some() {
                    if e.json {
                        actions::print::pipeline::json(e.name.clone())?;
                    } else {
                        actions::print::pipeline::pretty(e.name.clone())?;
                    }
                } else {
                    actions::print::pipeline::default()?;
                }
            }
            PostCommands::Inspect(e) => {
                if let Some(name) = e.name.clone() {
                    print::pipeline::inspect(&name, e.json)?;
                } else {
                    // Select prompt
                    let name = prompt::pipeline()?;
                    print::pipeline::inspect(&name, e.json)?;
                }
            }
            PostCommands::DetachableCommands(e) => {
                e.start()?;
            }
        };
        Ok(())
    }
}
