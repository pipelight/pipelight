// import tests
mod test;

use crate::interface::types::{
    Cli, Commands, DisplayCommands, Logs, LogsCommands, Pipeline, Shell, Trigger,
};

use clap_verbosity_flag::Verbosity;
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

fn from_verbosity_to_string(e: Verbosity) -> String {
    let mut string = "".to_owned();
    if e.is_silent() {
        string += " ";
        string += "-q";
    }

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
            Commands::Run(pipeline) => format!("run{}", pipeline),
            Commands::Stop(pipeline) => format!("stop{}", pipeline),
            Commands::Trigger(trigger) => format!("trigger{}", trigger),
            Commands::Logs(logs) => format!("logs{}", logs),
            Commands::Inspect(pipeline) => format!("inspect{}", pipeline),
            Commands::Ls(list) => format!("ls{}", list),
            Commands::Watch(_) => "watch".to_owned(),
            Commands::Completion(shell) => format!("completion{}", shell),
        };
        write!(f, "{}", string)
    }
}
