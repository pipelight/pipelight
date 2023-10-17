// Structs
use cli::services::{Action, FgBg, Service};
use cli::types::{
    Cli, ColoredOutput, Commands, DetachableCommands, LogsCommands, PostCommands, PreCommands,
};
use workflow::{Config, Getters, Logs, Pipeline, Trigger};
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
        set_early_globals()?;
        let mut args = CLI.lock().unwrap().clone();
        let args_binding = CLI.lock().unwrap().clone();
        match &mut args.commands {
            Commands::PreCommands(pre_commands) => {
                pre_commands.start()?;
            }
            Commands::PostCommands(post_commands) => {
                set_globals()?;
                match post_commands {
                    PostCommands::DetachableCommands(detachable_commands) => {
                        detachable_commands.start()?;
                    }
                    _ => {
                        Service::new(Action::Watch, Some(args_binding))?.should_detach()?;
                        post_commands.start()?;
                    }
                }
            }
        };
        Ok(())
    }
}
