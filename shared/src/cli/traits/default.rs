use crate::cli::types::{Cli, Commands, DisplayCommands, LogsCommands};
use clap_verbosity_flag::Verbosity;

impl Cli {
    pub fn new() -> Cli {
        Self::default()
    }
}

impl Default for Cli {
    fn default() -> Self {
        let cli = Cli {
            commands: Commands::Ls(DisplayCommands {
                json: false,
                name: None,
            }),
            raw: None,
            config: None,
            verbose: Verbosity::new(0, 0),
            attach: true,
        };
        return cli;
    }
}
