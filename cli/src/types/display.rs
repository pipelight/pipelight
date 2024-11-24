// Structs
use crate::types::{
    Attach, Cli, DisplayCommands, Init, Logs, LogsCommands, Pipeline, Shell, Toggle,
    ToggleCommands, Trigger,
};
use crate::types::{Commands, DetachableCommands, PostCommands, PreCommands};
use crate::types::{InternalVerbosity, Verbosity};

use log::LevelFilter;
use std::fmt;

impl fmt::Display for Cli {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut string = format!("{}", self.commands);

        if self.config.is_some() {
            string += " ";
            string += &format!("--config {}", self.config.clone().unwrap());
        }
        if self.verbose.log_level_filter() != LevelFilter::Error {
            string += &from_verbosity_to_string(self.verbose.clone());
        }
        string += &from_internal_verbosity_to_string(self.internal_verbose.clone());

        if let Some(attach) = self.attach.clone() {
            match Attach::from(&attach) {
                Attach::True => {
                    string += " ";
                    string += "--attach";
                }
                _ => {}
            };
        }
        // Keep last arg
        if self.raw.is_some() {
            string += " ";
            string += &format!("-- {}", self.raw.clone().unwrap().join(" "));
        }
        write!(f, "{}", string)
    }
}

impl fmt::Display for Commands {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string = match self {
            Commands::PreCommands(pre_commands) => match pre_commands {
                PreCommands::Init(_) => "init".to_owned(),
                PreCommands::Completion(shell) => format!("completion{}", shell),
                PreCommands::Enable(toggle) => format!("enable{}", toggle),
                PreCommands::Disable(toggle) => format!("disable{}", toggle),
            },
            Commands::PostCommands(post_commands) => match post_commands {
                PostCommands::DetachableCommands(detachable_command) => match detachable_command {
                    DetachableCommands::Run(pipeline) => format!("run{}", pipeline),
                    DetachableCommands::Trigger(trigger) => format!("trigger{}", trigger),
                    DetachableCommands::Watch => "watch".to_owned(),
                },
                PostCommands::Stop(pipeline) => format!("stop{}", pipeline),
                PostCommands::Logs(logs) => format!("logs{}", logs),
                PostCommands::Inspect(pipeline) => format!("inspect{}", pipeline),
                PostCommands::Ls(list) => format!("ls{}", list),
            },
        };
        write!(f, "{}", string)
    }
}

impl fmt::Display for Pipeline {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut string = "".to_owned();

        if self.name.is_some() {
            #[cfg(not(target_os = "macos"))]
            fn escape(name: &str) -> String {
                return format!("{:?}", name);
            }
            #[cfg(target_os = "macos")]
            fn escape(name: &str) -> String {
                return format!("{}", name);
            }
            string += " ";
            string += &escape(&self.name.clone().unwrap());
        }
        if self.trigger.flag.is_some() {
            string += " ";
            string += "--flag";
            string += " ";
            string += &self.trigger.flag.clone().unwrap();
        }
        write!(f, "{}", string)
    }
}
impl fmt::Display for DisplayCommands {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut string = "".to_owned();
        if self.name.is_some() {
            string += " ";
            string += &self.name.clone().unwrap();
        }
        if self.json {
            string += " ";
            string += "--json";
        }
        write!(f, "{}", string)
    }
}
impl fmt::Display for Shell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut string = "".to_owned();
        string += " ";
        string += &self.name;
        write!(f, "{}", string)
    }
}
impl fmt::Display for Init {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string = "".to_owned();
        write!(f, "{}", string)
    }
}
impl fmt::Display for Logs {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut string = "".to_owned();
        if self.commands.is_some() {
            match self.commands.clone().unwrap() {
                LogsCommands::Rm => {
                    string += " ";
                    string += "rm";
                }
            }
            string += &format!("{}", &self.display);
        }
        write!(f, "{}", string)
    }
}

impl fmt::Display for Toggle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut string = "".to_owned();
        if let Some(commands) = self.commands.clone() {
            match commands {
                ToggleCommands::GitHooks => {
                    string += " ";
                    string += "git-hooks";
                }
                ToggleCommands::Watcher => {
                    string += " ";
                    string += "watcher";
                }
            }
        }
        write!(f, "{}", string)
    }
}

impl fmt::Display for Trigger {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut string = "".to_owned();
        if self.flag.is_some() {
            string += " ";
            string += "--flag";
            string += " ";
            string += &self.flag.clone().unwrap();
        }
        write!(f, "{}", string)
    }
}

fn from_internal_verbosity_to_string(e: InternalVerbosity) -> String {
    let mut string = "".to_owned();
    if e.is_silent() {
        string += " ";
        string += "-q";
    }
    if e.log_level_filter() > LevelFilter::Error {
        let n = e.log_level_filter() as usize;
        string += " ";
        string += "-";
        string += &"u".repeat(n - 1);
    }
    string
}
fn from_verbosity_to_string(e: Verbosity) -> String {
    let mut string = "".to_owned();
    if e.log_level_filter() > LevelFilter::Error {
        let n = e.log_level_filter() as usize;
        string += " ";
        string += "-";
        string += &"v".repeat(n - 1);
    }
    string
}

#[cfg(test)]
mod display {
    // Structs
    use crate::types::{
        Attach, Cli, DisplayCommands, Init, Logs, LogsCommands, Pipeline, Shell, Toggle, Trigger,
    };
    use crate::types::{Commands, DetachableCommands, PostCommands, PreCommands};
    use crate::types::{InternalVerbosity, Verbosity};

    // Test Cli struct to bash string convertion.
    #[test]
    fn pipeline_args() {
        // Define a cli struct
        let cli = Cli {
            commands: Commands::PostCommands(PostCommands::DetachableCommands(
                DetachableCommands::Run(Pipeline {
                    name: Some("test".to_owned()),
                    trigger: Trigger {
                        flag: Some("pre-push".to_owned()),
                    },
                }),
            )),
            attach: Some(String::from(&Attach::False)),
            raw: None,
            config: None,
            // Set verbosity to default level (Error)
            internal_verbose: InternalVerbosity::new(0, 0),
            verbose: Verbosity::new(0, 0),
        };
        let result = format!("{}", cli);
        println!("\n{}", result);
        assert_eq!(result, "run \"test\" --flag pre-push");
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
            attach: Some(String::from(&Attach::False)),
            raw: None,
            config: None,
            internal_verbose: InternalVerbosity::new(0, 0),
            verbose: Verbosity::new(0, 0),
        };
        let result = format!("{}", cli);
        println!("\n{}", result);
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
            attach: Some(String::from(&Attach::False)),
            raw: None,
            config: None,
            // fn new(verbose: u8, quiet: u8) -> Self
            internal_verbose: InternalVerbosity::new(2, 0),
            verbose: Verbosity::new(0, 0),
        };
        let result = format!("{}", cli);
        println!("\n{}", result);
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
            attach: Some(String::from(&Attach::False)),
            raw: None,
            config: None,
            // fn new(verbose: u8, quiet: u8) -> Self
            internal_verbose: InternalVerbosity::new(0, 0),
            verbose: Verbosity::new(2, 0),
        };
        let result = format!("{}", cli);
        println!("\n{}", result);
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
            attach: Some(String::from(&Attach::False)),
            raw: None,
            config: Some("test.pipelight.ts".to_owned()),
            // fn new(verbose: u8, quiet: u8) -> Self
            internal_verbose: InternalVerbosity::new(0, 0),
            verbose: Verbosity::new(0, 0),
        };
        let result = format!("{}", cli);
        println!("\n{}", result);
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
            attach: Some(String::from(&Attach::False)),
            raw: Some(vec!["--host".to_owned(), "linode".to_owned()]),
            config: None,
            // fn new(verbose: u8, quiet: u8) -> Self
            internal_verbose: InternalVerbosity::new(0, 0),
            verbose: Verbosity::new(0, 0),
        };
        let result = format!("{}", cli);
        println!("\n{}", result);
        assert_eq!(result, "ls -- --host linode");
    }
}
