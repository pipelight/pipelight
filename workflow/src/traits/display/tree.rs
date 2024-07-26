// Caracteres
use super::characters::Characters;
// Colors
pub use colored::control::set_override;
use colored::{ColoredString, Colorize};
// Structs
use crate::types::Node;
use pipelight_utils::exec::Status;
use log::LevelFilter;
use regex::Regex;
use std::fmt;
// Globals
use pipelight_utils::globals::LOGGER;
// Date utilities
use pipelight_utils::dates::convert::*;

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
    /// Add a leaf to prefix T from inside a vec of T of nth element
    pub fn leaf(&self, prefix: String, index: usize, length: usize) -> ColoredString {
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

            if self.duration.is_some()
                && LOGGER.lock().unwrap().pipelines.level >= LevelFilter::Error
            {
                let human_duration = std_duration_to_human_duration(
                    iso8601_to_std_duration(self.duration.as_ref().unwrap()).unwrap(),
                )
                .unwrap();
                let pretty = format!(" ({})", human_duration);
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
                    if child.level <= LOGGER.lock().unwrap().pipelines.level {
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

impl fmt::Display for Node {
    fn fmt(&self, _f: &mut fmt::Formatter) -> fmt::Result {
        // set_override(true);
        let prefix = "".to_owned();
        self.clone().display(prefix);
        Ok(())
    }
}
