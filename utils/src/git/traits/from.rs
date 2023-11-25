use crate::git::types::Special::*;
use crate::git::types::{Flag, Hook, Special};
use convert_case::{Case, Casing};
use log::error;
use std::fmt;
use std::process::exit;

impl fmt::Display for Hook {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl From<&String> for Special {
    fn from(action: &String) -> Special {
        // let cased: &str = &action.to_case(Case::Snake);
        let cased: &str = &action.to_case(Case::Kebab);
        match cased {
            "manual" => Manual,
            "watch" => Watch,
            _ => {
                let message = format!("The special flag {} is not known", cased);
                error!("{}", message);
                exit(1);
            }
        }
    }
}
impl From<&Special> for String {
    fn from(action: &Special) -> String {
        serde_plain::to_string::<Special>(action).unwrap()
    }
}

impl From<&String> for Hook {
    fn from(action: &String) -> Hook {
        // let cased: &str = &action.to_case(Case::Snake);
        let cased: &str = &action.to_case(Case::Kebab);
        serde_plain::from_str(cased).unwrap()
    }
}
impl From<&Hook> for String {
    fn from(action: &Hook) -> String {
        serde_plain::to_string::<Hook>(action).unwrap()
    }
}
impl From<&String> for Flag {
    fn from(action: &String) -> Flag {
        let cased: &str = &action.to_case(Case::Kebab);
        match cased {
            "manual" => Flag::Special(Special::Manual),
            "watch" => Flag::Special(Special::Watch),
            "blank" => Flag::Special(Special::Blank),
            _ => Flag::Hook(Hook::from(action)),
        }
    }
}
impl From<&Flag> for String {
    fn from(action: &Flag) -> String {
        match action {
            Flag::Special(special) => String::from(special),
            Flag::Hook(hook) => String::from(hook),
        }
    }
}
