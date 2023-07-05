use crate::interface::types::{Cli, Commands, DisplayCommands, LogsCommands};
use clap_verbosity_flag::Verbosity;

impl Cli {
    pub fn new() -> Cli {
        Self::default()
    }
}

impl Default for Cli {
    fn default() -> Self {
        Cli {
            commands: Commands::Ls(DisplayCommands {
                json: false,
                name: None,
                color: None,
            }),
            raw: None,
            config: None,
            verbose: Verbosity::new(0, 0),
            attach: true,
        }
    }
}
