use crate::types::Special::*;
use crate::types::{Flag, Hook, Special};
use convert_case::{Case, Casing};
use log::error;
use std::fmt;
use std::process::exit;
// Trait - Enum iteration workaround
use strum::IntoEnumIterator;

impl fmt::Display for Special {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
impl From<&Special> for String {
    fn from(action: &Special) -> String {
        serde_plain::to_string::<Special>(action).unwrap()
    }
}
impl From<&String> for Special {
    fn from(action: &String) -> Special {
        // let cased: &str = &action.to_case(Case::Snake);
        let cased: &str = &action.to_case(Case::Kebab);
        match cased {
            "manual" => Manual,
            "blank" => Blank,
            "watch" => Watch,
            _ => {
                let message = format!("The special flag {} is not known", cased);
                error!("{}", message);
                exit(1);
            }
        }
    }
}

impl fmt::Display for Hook {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
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
        let cased: &String = &action.to_case(Case::Kebab);

        let specials: Vec<String> = Special::iter().map(|x| String::from(&x)).collect();
        let git_hooks: Vec<String> = Hook::iter().map(|x| String::from(&x)).collect();

        if specials.contains(cased) {
            Flag::Special(Special::from(cased))
        } else if git_hooks.contains(cased) {
            Flag::Hook(Hook::from(cased))
        } else {
            let message = format!("The special flag {} is not known", cased);
            error!("{}", message);
            panic!("{}", message);
        }
    }
}
impl fmt::Display for Flag {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
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
