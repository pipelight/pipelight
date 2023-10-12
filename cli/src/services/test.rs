#[cfg(test)]
mod service {
    // Struct
    use crate::types::Service;
    use actions::types::Action;
    use crate::Pipeline;
    use crate::{Cli, Commands, DetachableCommands, PostCommands};
    // Error Handling
    use miette::{IntoDiagnostic, Result};

    use assert_cmd::prelude::*; // Add methods on commands
    use std::process::Command; // Run commnds

    /// Run pipeline but no config found
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
            args.attach = true;
        }
        let service = Service::new(Action::Run(None), args)?;
        println!("{:#?}", service);
        Ok(())
    }
    /// Run pipeline but no config found
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
            args.attach = true;
        }
        let service = Service::new(Action::Run(None), args)?;
        service.detach()?;
        Ok(())
    }
}
