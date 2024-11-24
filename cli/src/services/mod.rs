//Traits
mod convert;
mod detach;
mod execute;

pub use convert::Parser;
pub use detach::FgBg;
pub use execute::Exec;

// Struct
use crate::types::Cli;

use log::LevelFilter;
use std::fmt;

// Error Handling
use miette::Result;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Action {
    RunStrict,
    RunLoose,
    Trigger,
    Watch,
}

/**
If you want to create a detached or attached running process fork.
If you want to execute actions in the same process,
use Actions directly.
*/
#[derive(Debug, Clone)]
pub struct Service {
    pub args: Option<Cli>,
    pub cmd: Action,
}

impl Service {
    pub fn new(cmd: Action, args: Option<Cli>) -> Result<Self> {
        let mut service = Service { cmd, args };
        service.convert()?;
        Ok(service)
    }
}

impl fmt::Display for Service {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // global vars
        let bin = "pipelight";

        // Dev env or production env
        #[cfg(debug_assertions)]
        let command = format!("cargo run --bin {} {}", &bin, &self.args.clone().unwrap());
        #[cfg(not(debug_assertions))]
        let command = format!("{} {}", &bin, &cmd_args);

        write!(f, "{}", command)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::Pipeline;
    use crate::{Attach, Cli, Commands, DetachableCommands, PostCommands};

    use assert_cmd::prelude::*; // Add methods on commands
    use std::process::Command; // Run commnds

    /**
    Arguments to run an empty pipeline
    */
    fn make_dummy_service() -> Result<Service> {
        let args = Cli {
            commands: Commands::PostCommands(PostCommands::DetachableCommands(
                DetachableCommands::Run(Pipeline {
                    name: Some("test".to_owned()),
                    ..Pipeline::default()
                }),
            )),
            attach: Some(String::from(&Attach::True)),
            ..Cli::default()
        };
        let service = Service::new(Action::RunStrict, Some(args))?;
        Ok(service)
    }

    /*
     * Create default service
     */
    #[test]
    fn create_service() -> Result<()> {
        let service = make_dummy_service();
        assert!(service.is_ok());
        Ok(())
    }
    /*
     * Print the service's resulting command
     */
    #[test]
    fn print_command() -> Result<()> {
        let service = make_dummy_service()?;
        println!("{:#?}", service);
        println!("{}", service);
        Ok(())
    }

    #[test]
    fn start_detached_service() -> Result<()> {
        let mut service = make_dummy_service()?;
        if let Some(ref mut args) = service.args {
            args.attach = Some(String::from(&Attach::False));
        }
        service.detach()?;
        Ok(())
    }

    #[test]
    fn start_detached_watcher() -> Result<()> {
        let args = Cli {
            commands: Commands::PostCommands(PostCommands::DetachableCommands(
                DetachableCommands::Watch,
            )),
            ..Cli::default()
        };
        let service = Service::new(Action::Watch, Some(args))?;
        service.detach()?;
        Ok(())
    }
}
