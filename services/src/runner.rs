// Struct
use actions::types::Action;
use cli::types::{Cli, Commands, DetachableCommands, Pipeline, PostCommands, Trigger};
use exec::Status;
use utils::git::Flag;
// use workflow::{Getters, Node, Pipeline};
// Error Handling
use crate::traits::Parser;
use crate::types::Service;
use miette::{Error, Result};

impl Service {
    pub fn new(action: Action, args: Option<Cli>) -> Result<Self> {
        Ok(Service { action, args })
    }
    pub fn convert_args(&mut self) -> Result<()> {
        // Retrieve reusable arguments
        let mut flag = Some(String::from(&Flag::default()));
        let mut name = None;
        if let Some(args) = self.args.clone() {
            match args.commands {
                Commands::PostCommands(post_commands) => match post_commands {
                    PostCommands::DetachableCommands(detachable_commands) => {
                        match detachable_commands {
                            DetachableCommands::Trigger(trigger) => {
                                flag = trigger.flag;
                            }
                            DetachableCommands::Run(pipeline) => {
                                flag = pipeline.trigger.flag;
                                name = pipeline.name;
                            }
                            _ => {}
                        }
                    }
                    _ => {}
                },
                _ => {}
            };
        }
        // Detach
        match self.action {
            Action::Run(_) => {
                if let Some(ref mut args) = self.args {
                    args.commands = Commands::PostCommands(PostCommands::DetachableCommands(
                        DetachableCommands::Run(Pipeline {
                            trigger: Trigger { flag },
                            name,
                        }),
                    ));
                }
            }
            Action::Trigger(_) => {
                if let Some(ref mut args) = self.args {
                    args.commands = Commands::PostCommands(PostCommands::DetachableCommands(
                        DetachableCommands::Trigger(Trigger { flag }),
                    ))
                }
            }
            Action::Watch => {}
        };
        Ok(())
    }
    pub fn run_action(&mut self) -> Result<()> {
        self.convert_args()?;
        // Attach
        // match self.action {
        // Action::Run(_) => Action::Run(pipeline_name),
        // Action::Trigger(_) => Action::Trigger(flag),
        // Action::Watch => Action::Watch,
        // };
        Ok(())
    }
}
