use crate::types::{Config, Logs, Pipeline, Trigger};
use log::warn;
use std::fs;
use std::path::Path;
use utils::logger::logger;

// Error Handling
use miette::{miette, Diagnostic, Error, IntoDiagnostic, NamedSource, Report, Result, SourceSpan};
use thiserror::Error;
// use std::error::Error;

pub trait Getters<T> {
    /// Return every instances of the struct.
    fn get() -> Result<Vec<T>>;
    /// Return an instance of the struct.
    fn get_by_name(name: &str) -> Result<T>;
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
            Ok(pipelines)
        } else {
            let message = format!("Couldn't find a pipeline named {:?}, in logs", name);
            return Err(Error::msg(message));
        }
    }
}
impl Getters<Pipeline> for Pipeline {
    fn get() -> Result<Vec<Pipeline>> {
        let config = Config::new(None)?;
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
