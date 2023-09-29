// Tests
mod test;
// Structs
use crate::types::{
    Cli, Commands, DisplayCommands, Init, InternalVerbosity, Logs, LogsCommands, Pipeline,
    PostCommands, PreCommands, Shell, Toggle, Trigger, Verbosity,
};

use log::LevelFilter;
use std::fmt;

impl fmt::Display for Cli {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut string = format!("{}", self.commands);

        if self.config.is_some() {
            string += " ";
            string += &format!("--config {}", self.config.clone().unwrap());
        }
        if self.raw.is_some() {
            string += " ";
            string += &format!("-- {}", self.raw.clone().unwrap().join(" "));
        }
        string += &from_verbosity_to_string(self.verbose.clone());
        string += &from_internal_verbosity_to_string(self.internal_verbose.clone());

        if self.attach {
            string += " ";
            string += "--attach";
        }
        write!(f, "{}", string)
    }
}

impl fmt::Display for Pipeline {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut string = "".to_owned();

        if self.name.is_some() {
            string += " ";
            string += &self.name.clone().unwrap();
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
        if self.enable {
            string += " ";
            string += "enable";
        }
        if self.disable {
            string += " ";
            string += "disable";
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

impl fmt::Display for Commands {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let string = match self {
            Commands::PreCommands(pre_commands) => match pre_commands {
                PreCommands::Init(_) => "init".to_owned(),
                PreCommands::Hooks(toggle) => format!("hooks{}", toggle),
                PreCommands::Completion(shell) => format!("completion{}", shell),
            },
            Commands::PostCommands(post_commands) => match post_commands {
                PostCommands::Run(pipeline) => format!("run{}", pipeline),
                PostCommands::Stop(pipeline) => format!("stop{}", pipeline),
                PostCommands::Trigger(trigger) => format!("trigger{}", trigger),
                PostCommands::Logs(logs) => format!("logs{}", logs),
                PostCommands::Inspect(pipeline) => format!("inspect{}", pipeline),
                PostCommands::Ls(list) => format!("ls{}", list),
                PostCommands::Watch(_) => "watch".to_owned(),
            },
        };
        write!(f, "{}", string)
    }
}
