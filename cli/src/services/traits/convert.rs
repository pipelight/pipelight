// Struct
use crate::services::types::{Action, Service};
use crate::types::{Commands, DetachableCommands, PostCommands};
use crate::types::{Pipeline, Trigger};
use utils::git::Flag;
// Error Handling
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
            if let Commands::PostCommands(PostCommands::DetachableCommands(detachable_commands)) =
                args.commands
            {
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
            };
        }

        // Rewrite the arg according to action
        match self.cmd {
            Action::RunStrict => {
                if let Some(ref mut args) = self.args {
                    args.commands = Commands::PostCommands(PostCommands::DetachableCommands(
                        DetachableCommands::Run(Pipeline {
                            trigger: Trigger { flag },
                            name,
                        }),
                    ));
                }
            }
            Action::RunLoose => {
                if let Some(ref mut args) = self.args {
                    args.commands = Commands::PostCommands(PostCommands::DetachableCommands(
                        DetachableCommands::Run(Pipeline {
                            trigger: Trigger { flag },
                            name,
                        }),
                    ));
                }
            }
            Action::Trigger => {
                if let Some(ref mut args) = self.args {
                    args.commands = Commands::PostCommands(PostCommands::DetachableCommands(
                        DetachableCommands::Trigger(Trigger { flag }),
                    ))
                }
            }
            Action::Watch => {
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
