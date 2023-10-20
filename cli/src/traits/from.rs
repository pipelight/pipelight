use crate::types::ColoredOutput;
use convert_case::{Case, Casing};
// Structs
use crate::types::{
    Cli, DisplayCommands, Init, Logs, LogsCommands, Pipeline, Shell, Toggle, Trigger,
};
use crate::types::{Commands, DetachableCommands, PostCommands, PreCommands};
use crate::types::{InternalVerbosity, Verbosity};
// Traits
use clap::FromArgMatches;

// Error Handling
use log::LevelFilter;
use miette::Result;

impl From<&String> for ColoredOutput {
    fn from(option: &String) -> ColoredOutput {
        let cased: &str = &option.to_case(Case::Kebab);
        serde_plain::from_str(cased).unwrap()
    }
}
impl From<&ColoredOutput> for String {
    fn from(option: &ColoredOutput) -> String {
        serde_plain::to_string::<ColoredOutput>(option).unwrap()
    }
}
impl From<&Cli> for String {
    fn from(e: &Cli) -> String {
        let mut string = format!("{}", e.commands);

        if e.config.is_some() {
            string += " ";
            string += &format!("--config {}", e.config.clone().unwrap());
        }
        if e.raw.is_some() {
            string += " ";
            string += &format!("-- {}", e.raw.clone().unwrap().join(" "));
        }
        string += &from_verbosity_to_string(e.verbose.clone());
        string += &from_internal_verbosity_to_string(e.internal_verbose.clone());

        if e.attach {
            string += " ";
            string += "--attach";
        }
        string
    }
}

impl From<&Pipeline> for String {
    fn from(e: &Pipeline) -> String {
        let mut string = "".to_owned();

        if e.name.is_some() {
            string += " ";
            string += &e.name.clone().unwrap();
        }
        if e.trigger.flag.is_some() {
            string += " ";
            string += "--flag";
            string += " ";
            string += &e.trigger.flag.clone().unwrap();
        }
        string
    }
}

impl From<&DisplayCommands> for String {
    fn from(e: &DisplayCommands) -> String {
        let mut string = "".to_owned();
        if e.name.is_some() {
            string += " ";
            string += &e.name.clone().unwrap();
        }
        if e.json {
            string += " ";
            string += "--json";
        }
        string
    }
}
impl From<&Shell> for String {
    fn from(e: &Shell) -> String {
        let mut string = "".to_owned();
        string += " ";
        string += &e.name;
        string
    }
}

impl From<&Init> for String {
    fn from(e: &Init) -> String {
        let mut string = "".to_owned();
        if let Some(template) = e.template.clone() {
            string += &template;
        }
        if let Some(file) = e.file.clone() {
            string += &file;
        }
        string
    }
}
impl From<&Logs> for String {
    fn from(e: &Logs) -> String {
        let mut string = "".to_owned();
        if let Some(commands) = e.commands.clone() {
            match commands {
                LogsCommands::Rm => {
                    string += " ";
                    string += "rm";
                }
            }
            string += &format!("{}", &e.display);
        }
        string
    }
}

impl From<&Toggle> for String {
    fn from(e: &Toggle) -> String {
        let mut string = "".to_owned();
        if e.enable {
            string += " ";
            string += "enable";
        }
        if e.disable {
            string += " ";
            string += "disable";
        }
        string
    }
}

impl From<&Trigger> for String {
    fn from(e: &Trigger) -> String {
        let mut string = "".to_owned();
        if e.flag.is_some() {
            string += " ";
            string += "--flag";
            string += " ";
            string += &e.flag.clone().unwrap();
        }
        string
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

impl From<&Commands> for String {
    fn from(e: &Commands) -> String {
        match e {
            Commands::PreCommands(pre_commands) => match pre_commands {
                PreCommands::Init(_) => "init".to_owned(),
                PreCommands::Hooks(toggle) => format!("hooks{}", toggle),
                PreCommands::Completion(shell) => format!("completion{}", shell),
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
        }
    }
}
pub fn string_to_command(e: &str) -> Result<Commands> {
    let os_str: Vec<&str> = e.split(' ').collect();
    let cli = Cli::build()?;
    let matches = cli.get_matches_from(os_str);
    let args = Cli::from_arg_matches(&matches)
        .map_err(|err| err.exit())
        .unwrap();
    let command: Commands = args.commands;
    Ok(command)
}
