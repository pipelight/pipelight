// Struct
use crate::services::types::Service;
use crate::types::{Cli, Commands, DetachableCommands, PostCommands};
use crate::types::{Pipeline, Trigger, Watch};
use exec::Status;
use utils::git::Flag;
// Process manipulation
use exec::SelfProcess;
// Error Handling
use log::trace;
use miette::Result;

pub trait FgBg {
    /**
    Fork action/process end send to background
    */
    fn detach(&mut self) -> Result<()>;
    /**
    Fork action/process end keep in foreground
    */
    fn attach(&mut self) -> Result<()>;
    /**
    Inspect the parsed command line arguments (CLI global, attach flag)
    and determine whether to detach the subprocess or not.
    */
    fn should_detach(&mut self) -> Result<()>;
}

pub trait Parser {
    /**
    Take the command line arguments and recycle every arguments that can be
    to launch the service with same args (verbosity, flags...) as the main process.
    */
    fn convert(&mut self) -> Result<()>;
}

impl FgBg for Service {
    fn attach(&mut self) -> Result<()> {
        if let Some(args) = self.args.clone() {
            SelfProcess::run_fg_with_cmd(&String::from(&args))?;
        }
        Ok(())
    }
    fn detach(&mut self) -> Result<()> {
        if let Some(args) = self.args.clone() {
            SelfProcess::run_bg_with_cmd(&String::from(&args))?;
        }
        Ok(())
    }
    fn should_detach(&mut self) -> Result<()> {
        if let Some(mut args) = self.args.clone() {
            match args.attach.clone() {
                true => {
                    trace!("pipelight process is attached");
                    self.attach()?;
                }
                false => {
                    trace!("detach pipelight process");
                    // Exit the detach loop
                    args.attach = true;
                    self.detach()?;
                }
            };
        }
        Ok(())
    }
}
impl Parser for Service {
    fn convert(&mut self) -> Result<()> {
        // Default arguments
        // Trigger options
        let mut flag = Some(String::from(&Flag::default()));
        // Run options
        let mut name = None;
        // Watch options
        let mut toggle = None;

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
                            DetachableCommands::Watch(watch) => {
                                toggle = watch.toggle;
                            }
                        }
                    }
                    _ => {}
                },
                _ => {}
            };
        }

        // Rewrite the arg according to action
        match self.cmd {
            Commands::PostCommands(PostCommands::DetachableCommands(DetachableCommands::Run(
                _,
            ))) => {
                if let Some(ref mut args) = self.args {
                    args.commands = Commands::PostCommands(PostCommands::DetachableCommands(
                        DetachableCommands::Run(Pipeline {
                            trigger: Trigger { flag },
                            name,
                        }),
                    ));
                }
            }
            Commands::PostCommands(PostCommands::DetachableCommands(
                DetachableCommands::Trigger(_),
            )) => {
                if let Some(ref mut args) = self.args {
                    args.commands = Commands::PostCommands(PostCommands::DetachableCommands(
                        DetachableCommands::Trigger(Trigger { flag }),
                    ))
                }
            }
            Commands::PostCommands(PostCommands::DetachableCommands(
                DetachableCommands::Watch(_),
            )) => {
                if let Some(ref mut args) = self.args {
                    args.commands = Commands::PostCommands(PostCommands::DetachableCommands(
                        DetachableCommands::Watch(Watch { toggle }),
                    ))
                }
            }
            _ => {}
        };
        Ok(())
    }
}
