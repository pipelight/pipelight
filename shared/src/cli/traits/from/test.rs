#[cfg(test)]
mod display {
    use crate::cli::types::{
        Cli, Commands, DisplayCommands, List, Logs, LogsCommands, Pipeline, Trigger,
    };
    use crate::cli::verbosity::Verbosity;
    // Convert cli into string then print
    #[test]
    fn run() {
        // Define a cli struct
        let cli = Cli {
            commands: Commands::Run(Pipeline {
                name: Some("test".to_owned()),
                trigger: Trigger {
                    attach: false,
                    flag: None,
                },
            }),
            raw: None,
            config: None,
            // Set verbosity to default level (Error)
            verbose: Verbosity::new(0, 0),
        };
        // print it
        let result = format!("{}", cli);
        assert_eq!(result, "run test");
    }
    #[test]
    fn logs() {
        // Define a cli struct
        let cli = Cli {
            commands: Commands::Logs(Logs {
                commands: Some(LogsCommands::Rm),
                display: DisplayCommands {
                    json: false,
                    name: None,
                },
            }),
            raw: None,
            config: None,
            verbose: Verbosity::new(0, 0),
        };
        // print it
        let result = format!("{}", cli);
        assert_eq!(result, "logs rm");
    }
    #[test]
    fn verbosity() {
        // Define a cli struct
        let cli = Cli {
            commands: Commands::Ls(DisplayCommands {
                json: false,
                name: None,
            }),
            raw: None,
            config: None,
            // fn new(verbose: u8, quiet: u8) -> Self
            verbose: Verbosity::new(2, 0),
        };
        // print it
        let result = format!("{}", cli);
        assert_eq!(result, "ls -vvv");
    }
}
