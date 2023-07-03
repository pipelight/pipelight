#[cfg(test)]
mod display {
    use crate::{Cli, Commands, DisplayCommands, Logs, LogsCommands, Pipeline, Trigger};
    use clap_verbosity_flag::Verbosity;

    // Test Cli struct to bash string reversion
    //
    #[test]
    fn pipeline_args() {
        // Define a cli struct
        let cli = Cli {
            commands: Commands::Run(Pipeline {
                name: Some("test".to_owned()),
                trigger: Trigger {
                    flag: Some("pre-push".to_owned()),
                },
            }),
            attach: false,
            raw: None,
            config: None,
            // Set verbosity to default level (Error)
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
            commands: Commands::Logs(Logs {
                commands: Some(LogsCommands::Rm),
                display: DisplayCommands {
                    json: false,
                    name: None,
                },
            }),
            attach: false,
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
            attach: false,
            raw: None,
            config: None,
            // fn new(verbose: u8, quiet: u8) -> Self
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
            commands: Commands::Ls(DisplayCommands {
                json: false,
                name: None,
            }),
            attach: false,
            raw: None,
            config: Some("test.pipelight.ts".to_owned()),
            // fn new(verbose: u8, quiet: u8) -> Self
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
            commands: Commands::Ls(DisplayCommands {
                json: false,
                name: None,
            }),
            attach: false,
            raw: Some(vec!["--host".to_owned(), "linode".to_owned()]),
            config: None,
            // fn new(verbose: u8, quiet: u8) -> Self
            verbose: Verbosity::new(0, 0),
        };
        // print it
        let result = format!("{}", cli);
        assert_eq!(result, "ls -- --host linode");
    }
}
