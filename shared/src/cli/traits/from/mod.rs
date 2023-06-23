// import tests
mod test;

use crate::cli::types::{
    Cli, Commands, DisplayCommands, Logs, LogsCommands, Pipeline, Trigger, Verbosity, Watch,
};
// use crate::cli::verbosity::ErrorLevel;
use log::LevelFilter;

use std::fmt;

impl fmt::Display for Cli {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut string = format!("{}", self.commands);
        string += &format!("{}", self.verbose);
        write!(f, "{}", string)
    }
}

impl fmt::Display for Pipeline {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut string = "".to_owned();
        if self.name.is_some() {
            string += &self.name.clone().unwrap();
        }
        if self.trigger.flag.is_some() {
            string += " ";
            string += "--flag";
            string += " ";
            string += &self.trigger.flag.clone().unwrap();
        }
        if self.trigger.attach {
            string += " ";
            string += "--attach";
        }
        write!(f, "{}", string)
    }
}
impl fmt::Display for DisplayCommands {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut string = "".to_owned();
        if self.name.is_some() {
            string += &self.name.clone().unwrap();
        }
        if self.json {
            string += " ";
            string += "--json";
        }
        write!(f, "{}", string)
    }
}
impl fmt::Display for Logs {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut string = "".to_owned();
        if self.commands.is_some() {
            match self.commands.clone().unwrap() {
                LogsCommands::Rm => {
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
        if self.attach {
            string += " ";
            string += "--attach";
        }
        write!(f, "{}", string)
    }
}

impl fmt::Display for Verbosity {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut string = "".to_owned();
        if self.is_silent() {
            string += " ";
            string += "-q";
        }

        println!("log_level = {}", self.log_level_filter());
        println!("log_level = {:#?}", self);

        if self.log_level_filter() > LevelFilter::Error {
            let n = self.log_level_filter() as usize;
            string += " ";
            string += "-";
            string += &"v".repeat(n);
        }
        write!(f, "{}", string)
    }
}

impl fmt::Display for Watch {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut string = "".to_owned();
        if self.attach {
            string += " ";
            string += "--attach";
        }
        write!(f, "{}", string)
    }
}

impl fmt::Display for Commands {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut string;
        match self {
            Commands::Run(pipeline) => {
                string = format!("run {}", pipeline);
            }
            Commands::Raw(raw) => {
                string = "raw".to_owned();
                string += " ";
                string += &raw.string;
            }
            Commands::Stop(pipeline) => {
                string = format!("stop {}", pipeline);
            }
            Commands::Trigger(trigger) => {
                string = format!("trigger {}", trigger);
            }
            Commands::Logs(logs) => {
                string = format!("logs {}", logs);
            }
            Commands::Inspect(_) => string = "inspect".to_owned(),
            Commands::Ls(_) => string = "ls".to_owned(),
            Commands::Watch(_) => string = "watch".to_owned(),
        }
        write!(f, "{}", string)
    }
}
