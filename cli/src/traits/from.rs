use crate::types::ColoredOutput;
use convert_case::{Case, Casing};
// Structs
use crate::types::{
    Cli, DisplayCommands, Init, Logs, Pipeline, Shell, Toggle, Trigger,
};
use crate::types::{Commands};


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
        format!("{}", &e)
    }
}
impl From<&Commands> for String {
    fn from(e: &Commands) -> String {
        format!("{}", &e)
    }
}

impl From<&Pipeline> for String {
    fn from(e: &Pipeline) -> String {
        format!("{}", &e)
    }
}

impl From<&DisplayCommands> for String {
    fn from(e: &DisplayCommands) -> String {
        format!("{}", &e)
    }
}
impl From<&Shell> for String {
    fn from(e: &Shell) -> String {
        format!("{}", &e)
    }
}

impl From<&Init> for String {
    fn from(e: &Init) -> String {
        format!("{}", &e)
    }
}
impl From<&Logs> for String {
    fn from(e: &Logs) -> String {
        format!("{}", &e)
    }
}

impl From<&Toggle> for String {
    fn from(e: &Toggle) -> String {
        format!("{}", &e)
    }
}

impl From<&Trigger> for String {
    fn from(e: &Trigger) -> String {
        format!("{}", &e)
    }
}
