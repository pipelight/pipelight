// Struct
use crate::services::types::{Actions, Service};
use crate::types::{Cli, Commands, DetachableCommands, PostCommands};
use crate::types::{Pipeline, Trigger, Watch};
use exec::Status;
use utils::git::Flag;
// Process manipulation
use exec::SelfProcess;
// Error Handling
use log::trace;
use miette::Result;

pub trait Parser {
    /**
    Take the command line arguments and recycle every arguments that can be
    to launch the service with same args (verbosity, flags...) as the main process.
    */
    fn convert(&mut self) -> Result<()>;
}

impl Parser for Service {
    fn convert(&mut self) -> Result<()> {
        // Default arguments
        // Trigger options
        let mut flag = Some(String::from(&Flag::default()));
        // Run options
        let mut name = None;

        // Retrieve reusable arguments and mutate the defaults
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

        // Rewrite the arg according to action
        match self.cmd {
            Actions::Run => {
                if let Some(ref mut args) = self.args {
                    args.commands = Commands::PostCommands(PostCommands::DetachableCommands(
                        DetachableCommands::Run(Pipeline {
                            trigger: Trigger { flag },
                            name,
                        }),
                    ));
                }
            }
            Actions::Trigger => {
                if let Some(ref mut args) = self.args {
                    args.commands = Commands::PostCommands(PostCommands::DetachableCommands(
                        DetachableCommands::Trigger(Trigger { flag }),
                    ))
                }
            }
            Actions::Watch => {
                if let Some(ref mut args) = self.args {
                    args.commands = Commands::PostCommands(PostCommands::DetachableCommands(
                        DetachableCommands::Watch,
                    ))
                }
            }
        };
        Ok(())
    }
}
