use crate::methods::{iso8601_to_std_duration, std_duration_to_iso8601};
use crate::types::{Duration, Node};

// Caracteres
mod characters;
use characters::Characters;

// From - Convert types to Node
mod from;

// Colors
pub use colored::control::set_override;
use colored::{ColoredString, Colorize};

use chrono::Utc;
use chrono::{DateTime, Local};
use exec::{Statuable, Status};
use log::LevelFilter;
use log::{debug, error, info, warn};
use regex::Regex;
use std::fmt;
use utils::logger::logger;

// Error Handling
use miette::{IntoDiagnostic, Result};

static INDENT: &str = "  ";

fn add_level(prefix: String) -> String {
    let leaf: String = format!("{}{INDENT:}", Characters::unicode().vbar, INDENT = INDENT);
    let mut prefix = prefix;
    prefix.push_str(&leaf);
    prefix.to_owned()
}
fn add_level_phantom(prefix: String) -> String {
    let leaf: String = format!("{}{INDENT:}", " ".to_owned());
    let mut prefix = prefix;
    prefix.push_str(&leaf);
    prefix.to_owned()
}
impl Node {
    /// Add a leaf to prefix T from inside [T] of nth element
    pub fn leaf(&self, prefix: String, index: usize, length: usize) -> ColoredString {
        let leaf: String;
        let leaf: String = if index == length {
            format!(
                "{prefix:}{}\n{prefix:}{}{}",
                Characters::unicode().vbar,
                Characters::unicode().lbot,
                Characters::unicode().hbar,
            )
        } else {
            format!(
                "{prefix:}{}\n{prefix:}{}{}",
                Characters::unicode().vbar,
                Characters::unicode().lcross,
                Characters::unicode().hbar,
            )
        };
        leaf.white()
    }
    /// Display Node and color based on Node.status
    fn display(&self, prefix: String) {
        // Display node value
        if self.value.is_some() {
            let mut value = self.value.clone().unwrap();
            // Remove extra spaces
            let big_spaces: Regex = Regex::new(r"\s\s+").unwrap();
            value = big_spaces.replace_all(&value, "\n").to_string();
            value = value.replace('\n', &format!("\n{prefix:}", prefix = prefix.white()));

            if self.duration.is_some() && logger.lock().unwrap().level >= LevelFilter::Error {
                let duration = format_duration(
                    iso8601_to_std_duration(self.duration.clone().unwrap()).unwrap(),
                )
                .unwrap();
                let pretty = format!(" ({})", duration);
                value.push_str(&format!("{}", pretty.white()));
            }
            if self.level <= LevelFilter::Error {
                println!("{}", &value);
            } else {
                match self.status {
                    Some(Status::Started) => println!("{}", &value),
                    Some(Status::Running) => println!("{}", &value.green()),
                    Some(Status::Succeeded) => println!("{}", &value.blue()),
                    Some(Status::Failed) => println!("{}", &value.red()),
                    Some(Status::Aborted) => println!("{}", &value.yellow()),
                    None => println!("{}", &value.white()),
                }
            }
            // Iterate over childs
            if self.children.is_some() {
                let length = self.children.clone().unwrap().len() - 1;
                for (index, child) in &mut self.children.clone().unwrap().iter().enumerate() {
                    if child.level <= logger.lock().unwrap().level {
                        print!("{}", &child.leaf(prefix.clone(), index, length));
                        if index == length {
                            let prefix = add_level_phantom(prefix.clone());
                            child.display(prefix.clone());
                        } else {
                            let prefix = add_level(prefix.clone());
                            child.display(prefix.clone());
                        }
                    }
                }
            }
        }
    }
}

pub fn format_duration(duration: std::time::Duration) -> Result<String> {
    let computed = chrono::Duration::from_std(duration).unwrap();
    let mut res: String = "".to_owned();
    // Res
    let minutes = computed.num_minutes();
    let seconds = computed.num_seconds() - minutes * 60;
    let milliseconds = computed.num_milliseconds() - minutes * 60 * 1000 - seconds * 1000;

    if minutes > 0 {
        res = format!("{}{}m", res, minutes);
    }
    if seconds > 0 {
        res = format!("{}{}s", res, seconds);
    }
    if milliseconds > 0 && minutes <= 0 {
        res = format!("{}{}ms", res, milliseconds);
    }
    Ok(res)
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // set_override(true);
        let prefix = "".to_owned();
        self.clone().display(prefix);
        Ok(())
    }
}
