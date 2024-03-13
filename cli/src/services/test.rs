#[cfg(test)]
mod service {
    // Struct
    use crate::services::types::{Action, Service};
    use crate::Pipeline;
    use crate::{Attach, Cli, Commands, DetachableCommands, PostCommands};
    // Traits
    use crate::services::traits::FgBg;
    // Error Handling
    use miette::{IntoDiagnostic, Result};

    use assert_cmd::prelude::*; // Add methods on commands
    use std::process::Command; // Run commnds

    /**
    Arguments to run an empty pipeline
    */
    fn make_dummy_args() -> Result<Cli> {
        let args = Cli {
            commands: Commands::PostCommands(PostCommands::DetachableCommands(
                DetachableCommands::Run(Pipeline {
                    name: Some("test".to_owned()),
                    ..Pipeline::default()
                }),
            )),
            ..Cli::default()
        };
        Ok(args)
    }

    /// Create a dummy service
    #[test]
    fn create_service() -> Result<()> {
        let mut args = make_dummy_args().ok();
        // println!("{:#?}", args);
        if let Some(ref mut args) = args {
            args.attach = Some(String::from(&Attach::True));
        }
        let service = Service::new(Action::RunStrict, args)?;
        println!("{:#?}", service);
        Ok(())
    }
    #[test]
    fn start_detached_service() -> Result<()> {
        let mut args = make_dummy_args().ok();
        // println!("{:#?}", args);
        if let Some(ref mut args) = args {
            args.attach = Some(String::from(&Attach::False));
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
            args.attach = Some(String::from(&Attach::True));
        }
        let service = Service::new(Action::Watch, args)?;
        service.detach()?;
        Ok(())
    }
}
