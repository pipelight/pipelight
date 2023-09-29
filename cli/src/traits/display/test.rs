#[cfg(test)]
mod display {
    use crate::interface::{
        Cli, Commands, DisplayCommands, Logs, LogsCommands, Pipeline, PostCommands, Trigger,
    };
    use crate::interface::{InternalVerbosity, Verbosity};

    // Test Cli struct to bash string convertion.
    #[test]
    fn pipeline_args() {
        // Define a cli struct
        let cli = Cli {
            commands: Commands::PostCommands(PostCommands::Run(Pipeline {
                name: Some("test".to_owned()),
                trigger: Trigger {
                    flag: Some("pre-push".to_owned()),
                },
            })),
            attach: false,
            raw: None,
            config: None,
            // Set verbosity to default level (Error)
            internal_verbose: InternalVerbosity::new(0, 0),
            verbose: Verbosity::new(0, 0),
        };
        // print it
        let result = format!("{}", cli);
        assert_eq!(result, "run test --flag pre-push");
    }
    #[test]
    fn logs_args() {
        // Define a cli struct
        let cli = Cli {
            commands: Commands::PostCommands(PostCommands::Logs(Logs {
                commands: Some(LogsCommands::Rm),
                display: DisplayCommands {
                    json: false,
                    name: None,
                    color: None,
                },
            })),
            attach: false,
            raw: None,
            config: None,
            internal_verbose: InternalVerbosity::new(0, 0),
            verbose: Verbosity::new(0, 0),
        };
        // print it
        let result = format!("{}", cli);
        assert_eq!(result, "logs rm");
    }
    #[test]
    fn internal_verbosity() {
        // Define a cli struct
        let cli = Cli {
            commands: Commands::PostCommands(PostCommands::Ls(DisplayCommands {
                json: false,
                name: None,
                color: None,
            })),
            attach: false,
            raw: None,
            config: None,
            // fn new(verbose: u8, quiet: u8) -> Self
            internal_verbose: InternalVerbosity::new(2, 0),
            verbose: Verbosity::new(0, 0),
        };
        // print it
        let result = format!("{}", cli);
        assert_eq!(result, "ls -uu");
    }
    #[test]
    fn verbosity() {
        // Define a cli struct
        let cli = Cli {
            commands: Commands::PostCommands(PostCommands::Ls(DisplayCommands {
                json: false,
                name: None,
                color: None,
            })),
            attach: false,
            raw: None,
            config: None,
            // fn new(verbose: u8, quiet: u8) -> Self
            internal_verbose: InternalVerbosity::new(0, 0),
            verbose: Verbosity::new(2, 0),
        };
        // print it
        let result = format!("{}", cli);
        assert_eq!(result, "ls -vv");
    }
    #[test]
    fn config_file() {
        // Define a cli struct
        let cli = Cli {
            commands: Commands::PostCommands(PostCommands::Ls(DisplayCommands {
                json: false,
                name: None,
                color: None,
            })),
            attach: false,
            raw: None,
            config: Some("test.pipelight.ts".to_owned()),
            // fn new(verbose: u8, quiet: u8) -> Self
            internal_verbose: InternalVerbosity::new(0, 0),
            verbose: Verbosity::new(0, 0),
        };
        // print it
        let result = format!("{}", cli);
        assert_eq!(result, "ls --config test.pipelight.ts");
    }
    #[test]
    fn deno_args() {
        // Define a cli struct
        let cli = Cli {
            commands: Commands::PostCommands(PostCommands::Ls(DisplayCommands {
                json: false,
                name: None,
                color: None,
            })),
            attach: false,
            raw: Some(vec!["--host".to_owned(), "linode".to_owned()]),
            config: None,
            // fn new(verbose: u8, quiet: u8) -> Self
            internal_verbose: InternalVerbosity::new(0, 0),
            verbose: Verbosity::new(0, 0),
        };
        // print it
        let result = format!("{}", cli);
        assert_eq!(result, "ls -- --host linode");
    }
}
