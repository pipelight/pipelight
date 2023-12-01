use crate::types::{Cli, Commands, DisplayCommands, PostCommands};
use crate::types::{InternalVerbosity, Verbosity};
use crate::types::{Pipeline, Trigger};

impl Cli {
    pub fn new() -> Cli {
        Self::default()
    }
}

impl Default for Pipeline {
    fn default() -> Self {
        Pipeline {
            name: Some("default".to_owned()),
            trigger: Trigger {
                flag: Some("blank".to_owned()),
            },
        }
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
            verbose: None,
            // verbose: Some(Verbosity::new(0, 0)),
            internal_verbose: InternalVerbosity::new(0, 0),
            attach: Some(true),
        }
    }
}
