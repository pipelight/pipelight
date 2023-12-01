#[cfg(test)]
mod service {
    // Struct
    use crate::services::types::{Action, Service};
    use crate::Pipeline;
    use crate::{Cli, Commands, DetachableCommands, PostCommands};
    // Traits
    use crate::services::traits::FgBg;
    // Error Handling
    use miette::{IntoDiagnostic, Result};

    use assert_cmd::prelude::*; // Add methods on commands
    use std::process::Command; // Run commnds

    #[test]
    fn create_service() -> Result<()> {
        let mut args = Some(Cli {
            commands: Commands::PostCommands(PostCommands::DetachableCommands(
                DetachableCommands::Run(Pipeline {
                    name: Some("test".to_owned()),
                    ..Pipeline::default()
                }),
            )),
            ..Cli::default()
        });
        // println!("{:#?}", args);
        if let Some(ref mut args) = args {
            args.attach = Some(true);
        }
        let service = Service::new(Action::RunStrict, args)?;
        println!("{:#?}", service);
        Ok(())
    }
    #[test]
    fn start_detached_service() -> Result<()> {
        let mut args = Some(Cli {
            commands: Commands::PostCommands(PostCommands::DetachableCommands(
                DetachableCommands::Run(Pipeline {
                    name: Some("test".to_owned()),
                    ..Pipeline::default()
                }),
            )),
            ..Cli::default()
        });
        // println!("{:#?}", args);
        if let Some(ref mut args) = args {
            args.attach = Some(true);
        }
        let service = Service::new(Action::RunStrict, args)?;
        service.detach()?;
        Ok(())
    }
    /// Run watcher
    #[test]
    fn start_detached_watcher() -> Result<()> {
        let mut args = Some(Cli {
            commands: Commands::PostCommands(PostCommands::DetachableCommands(
                DetachableCommands::Watch,
            )),
            ..Cli::default()
        });
        // println!("{:#?}", args);
        if let Some(ref mut args) = args {
            args.attach = Some(true);
        }
        let service = Service::new(Action::Watch, args)?;
        service.detach()?;
        Ok(())
    }
}
