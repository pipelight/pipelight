use crate::types::characters::Characters;
use crate::types::{Command, Event, Node, Parallel, Pipeline, Step, StepOrParallel, Trigger};
use chrono::Utc;
use chrono::{DateTime, Duration, Local};
use colored::{ColoredString, Colorize};
use exec::types::{Statuable, Status};
use log::LevelFilter;
use log::{debug, error, info, warn};
use std::error::Error;
use std::fmt;
use utils::logger::logger;

static INDENT: &str = "  ";

fn add_level(prefix: String) -> String {
    let leaf: String = format!("{}{INDENT:}", Characters::unicode().vbar, INDENT = INDENT);
    let mut prefix = prefix;
    prefix.push_str(&leaf);
    return prefix.to_owned();
}
fn add_level_phantom(prefix: String) -> String {
    let leaf: String = format!("{}{INDENT:}", " ".to_owned());
    let mut prefix = prefix;
    prefix.push_str(&leaf);
    return prefix.to_owned();
}
impl Node {
    /// Add a leaf to prefix T from inside [T] of nth element
    pub fn leaf(&self, prefix: String, index: usize, length: usize) -> ColoredString {
        let leaf: String;
        if index == length {
            leaf = format!(
                "{prefix:}{}\n{prefix:}{}{}",
                Characters::unicode().vbar,
                Characters::unicode().lbot,
                Characters::unicode().hbar,
            );
        } else {
            leaf = format!(
                "{prefix:}{}\n{prefix:}{}{}",
                Characters::unicode().vbar,
                Characters::unicode().lcross,
                Characters::unicode().hbar,
            );
        }
        return leaf.white();
    }
    /// Display Node and color based on Node.status
    fn display(&self, prefix: String) {
        // Display node value
        if self.value.is_some() {
            let mut value = self
                .value
                .clone()
                .unwrap()
                .replace("\n", &format!("\n{prefix:}", prefix = prefix.white()));
            if self.duration.is_some() {
                let duration = format_duration(self.duration.unwrap()).unwrap();
                let pretty = format!(" ({})", duration);
                value.push_str(&format!("{}", pretty.white()));
            }
            if self.level <= LevelFilter::Error {
                print!("{}\n", &value);
            } else {
                match self.status {
                    Some(Status::Started) => print!("{}\n", &value),
                    Some(Status::Running) => print!("{}\n", &value.green()),
                    Some(Status::Succeeded) => print!("{}\n", &value.blue()),
                    Some(Status::Failed) => print!("{}\n", &value.red()),
                    Some(Status::Aborted) => print!("{}\n", &value.yellow()),
                    None => print!("{}\n", &value.white()),
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

pub fn format_duration(duration: std::time::Duration) -> Result<String, Box<dyn Error>> {
    let res: String;
    let computed = Duration::from_std(duration).unwrap();

    let s = computed.num_seconds();
    let ms = computed.num_milliseconds();
    let m = computed.num_minutes();
    if s != 0 {
        res = format!("{}s", s);
    } else {
        if ms != 0 {
            res = format!("{}ms", ms);
        } else {
            res = format!("{}m", m);
        }
    }
    Ok(res)
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let prefix = "".to_owned();
        self.clone().display(prefix);
        Ok(())
    }
}
