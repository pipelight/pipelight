use crate::types::{Cli, Commands, DisplayCommands, InternalVerbosity, PostCommands, Verbosity};

impl Cli {
    pub fn new() -> Cli {
        Self::default()
    }
}

impl Default for Cli {
    fn default() -> Self {
        Cli {
            commands: Commands::PostCommands(PostCommands::Ls(DisplayCommands {
                json: false,
                name: None,
                color: None,
            })),
            raw: None,
            config: None,
            verbose: Verbosity::new(0, 0),
            internal_verbose: InternalVerbosity::new(0, 0),
            attach: true,
        }
    }
}
