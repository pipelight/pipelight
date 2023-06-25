use crate::types::{Config, Logs, Pipeline, Trigger};

// File storage
use std::fs;
use std::path::Path;

// Logger
use log::warn;
use utils::logger::logger;

// Date and Time
use chrono::{DateTime, Local};

// Error Handling
use miette::{miette, Diagnostic, Error, IntoDiagnostic, NamedSource, Report, Result, SourceSpan};
use thiserror::Error;
// use std::error::Error;

// Import global config
use super::default::CONFIG;

pub trait Getters<T> {
    /// Return every instances of the struct.
    fn get() -> Result<Vec<T>>;
    /// Return an instance of the struct.
    fn get_by_name(name: &str) -> Result<T>;
}

impl Config {
    pub fn get() -> Result<Self> {
        unsafe {
            if *CONFIG == Config::default() {
                let message = "Tried to get a Config that hasn't been initialized yet";
                warn!("{}", message);
                return Err(Error::msg(message));
            } else {
                let ptr = (*CONFIG).clone().to_owned();
                return Ok(ptr);
            }
        }
    }
}

impl Getters<Pipeline> for Logs {
    fn get() -> Result<Vec<Pipeline>> {
        let dir = &logger.lock().unwrap().directory;
        let message = "No logs to display.";
        // Safe exit if no log folder
        if !Path::new(dir).exists() {
            return Err(Error::msg(message));
        } else {
            let paths = fs::read_dir(dir).unwrap();
            let n = paths.count();
            if n == 0 {
                return Err(Error::msg(message));
            } else {
                let paths = fs::read_dir(dir).unwrap();
                let mut pipelines = vec![];
                for path in paths {
                    let dir_entry = path.into_diagnostic()?;
                    let json = utils::read_last_line(&dir_entry.path())?;
                    let pipeline = serde_json::from_str::<Pipeline>(&json).into_diagnostic()?;
                    pipelines.push(pipeline);
                }
                // pipelines = Logs::sanitize(pipelines)?;
                pipelines.sort_by(|a, b| {
                    let a_date = a
                        .clone()
                        .event
                        .unwrap()
                        .date
                        .parse::<DateTime<Local>>()
                        .unwrap();

                    let b_date = &b
                        .clone()
                        .event
                        .unwrap()
                        .date
                        .parse::<DateTime<Local>>()
                        .unwrap();
                    return a_date.cmp(b_date);
                });
                Ok(pipelines)
            }
        }
    }
    fn get_by_name(name: &str) -> Result<Pipeline> {
        let pipelines = Logs::get()?;
        let pipeline;
        let mut pipelines = pipelines
            .iter()
            .filter(|p| &p.name == name)
            .cloned()
            .collect::<Vec<Pipeline>>();
        if !pipelines.is_empty() {
            pipelines.sort_by_key(|e| e.clone().event.unwrap().date);
            pipeline = pipelines.pop().unwrap();
            Ok(pipeline)
        } else {
            let message = format!("Couldn't find a pipeline named {:?}, in logs", name);
            return Err(Error::msg(message));
        }
    }
}
impl Logs {
    pub fn get_many_by_name(name: &str) -> Result<Vec<Pipeline>> {
        let pipelines = Logs::get()?;
        let mut pipelines = pipelines
            .iter()
            .filter(|p| &p.name == name)
            .cloned()
            .collect::<Vec<Pipeline>>();
        if !pipelines.is_empty() {
            pipelines.sort_by_key(|e| e.clone().event.unwrap().date);
            pipelines.sort_by(|a, b| {
                let a_date = a
                    .clone()
                    .event
                    .unwrap()
                    .date
                    .parse::<DateTime<Local>>()
                    .unwrap();

                let b_date = &b
                    .clone()
                    .event
                    .unwrap()
                    .date
                    .parse::<DateTime<Local>>()
                    .unwrap();
                return a_date.cmp(b_date);
            });
            Ok(pipelines)
        } else {
            let message = format!("Couldn't find a pipeline named {:?}, in logs", name);
            return Err(Error::msg(message));
        }
    }
    pub fn get_many_by_sid(sid: &u32) -> Result<Vec<Pipeline>> {
        let pipelines = Logs::get()?;
        let mut pipelines = pipelines
            .iter()
            .filter(|p| {
                if p.event.clone().unwrap().sid.is_some() {
                    let p_sid = p.event.clone().unwrap().sid.unwrap();
                    return &p_sid == sid;
                } else {
                    return false;
                }
            })
            .cloned()
            .collect::<Vec<Pipeline>>();
        if !pipelines.is_empty() {
            pipelines.sort_by(|a, b| {
                let a_date = a
                    .clone()
                    .event
                    .unwrap()
                    .date
                    .parse::<DateTime<Local>>()
                    .unwrap();

                let b_date = &b
                    .clone()
                    .event
                    .unwrap()
                    .date
                    .parse::<DateTime<Local>>()
                    .unwrap();
                return a_date.cmp(b_date);
            });
            Ok(pipelines)
        } else {
            let message = format!("Couldn't find a pipeline with sid {:?}, in logs", sid);
            return Err(Error::msg(message));
        }
    }
}
impl Getters<Pipeline> for Pipeline {
    fn get() -> Result<Vec<Pipeline>> {
        let config = Config::get()?;
        let optional = config.pipelines;
        match optional {
            Some(p) => return Ok(p),
            None => {
                let message = "Couldn't retrieve pipelines";
                return Err(Error::msg(message));
            }
        };
    }
    fn get_by_name(name: &str) -> Result<Pipeline> {
        let pipelines = Pipeline::get()?;
        let optional = pipelines.iter().filter(|p| p.name == name).next();
        match optional {
            Some(res) => return Ok(res.to_owned()),
            None => {
                let message = format!("Couldn't find pipeline {:?}", name);
                warn!("{}", message);
                return Err(Error::msg(message));
            }
        };
    }
}

impl Getters<Trigger> for Trigger {
    fn get() -> Result<Vec<Trigger>> {
        let pipelines = Pipeline::get()?;
        let mut triggers = pipelines
            .iter()
            .map(|p| p.triggers.clone().unwrap_or_default())
            .collect::<Vec<Vec<Trigger>>>()
            .into_iter()
            .flatten()
            .collect::<Vec<Trigger>>();
        triggers.sort();
        triggers.dedup();
        Ok(triggers)
    }
    fn get_by_name(name: &str) -> Result<Trigger> {
        let triggers = Trigger::get();
        let binding = triggers?;
        let trigger = binding.iter().next().unwrap();
        Ok(trigger.to_owned())
    }
}
