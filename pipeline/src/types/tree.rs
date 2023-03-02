use crate::types::characters::Characters;
use crate::types::{Command, Event, Node, Parallel, Pipeline, Step, StepOrParallel, Trigger};
use chrono::Utc;
use chrono::{DateTime, Duration, Local};
use colored::{ColoredString, Colorize};
use exec::types::{Statuable, Status};
use log::LevelFilter;
use log::{debug, error, info, warn};
use regex::Regex;
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
            let mut value = self.value.clone().unwrap();
            // Remove extra spaces
            let big_spaces: Regex = Regex::new(r"\s\s+").unwrap();
            value = big_spaces.replace_all(&value, "\n").to_string();
            value = value.replace("\n", &format!("\n{prefix:}", prefix = prefix.white()));

            if self.duration.is_some() {
                if logger.lock().unwrap().level >= LevelFilter::Info {
                    let duration = format_duration(self.duration.unwrap()).unwrap();
                    let pretty = format!(" ({})", duration);
                    value.push_str(&format!("{}", pretty.white()));
                }
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
    let mut res: String;
    let computed = Duration::from_std(duration).unwrap();
    let m: f64 = computed.num_minutes() as f64;
    let mut s: f64 = computed.num_seconds() as f64;
    let mut ms: f64 = computed.num_milliseconds() as f64;
    res = "processing".to_owned();
    if m > 0 as f64 {
        res = format!("{:.2}m", m);
    } else {
        if s > 0 as f64 {
            s = (ms / 1000 as f64) as f64;
            res = format!("{:.2}s", s);
        } else {
            if ms > 0 as f64 {
                let ns = computed.num_nanoseconds();
                if ns.is_some() {
                    ms = (ns.unwrap() as f64 / (1000 * 1000) as f64) as f64;
                    res = format!("{:.2}ms", ms);
                }
            }
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
