use crate::types::{Logs, Pipeline};

// File storage
use std::fs;
use std::path::Path;

// Logger
use log::warn;
use utils::logger::logger;

// Date and Time
use chrono::{DateTime, Local};

// Error Handling
use miette::{Error, IntoDiagnostic, Result};

use super::Getters;

impl Getters<Pipeline> for Logs {
    fn get() -> Result<Vec<Pipeline>> {
        let dir = &logger.lock().unwrap().pipelines.directory;
        let message = "No logs to display.";
        // Safe exit if no log folder
        if !Path::new(dir).exists() {
            Err(Error::msg(message))
        } else {
            let paths = fs::read_dir(dir).unwrap();
            let n = paths.count();
            if n == 0 {
                Err(Error::msg(message))
            } else {
                let paths = fs::read_dir(dir).unwrap();
                let mut pipelines = vec![];
                for path in paths {
                    let dir_entry = path.into_diagnostic()?;
                    let json = utils::read_last_line(&dir_entry.path())?;
                    let pipeline = serde_json::from_str::<Pipeline>(&json);
                    if pipeline.is_err() {
                        warn!("Striping corrupted log")
                    } else {
                        pipelines.push(pipeline.into_diagnostic()?);
                    }
                }
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
                    a_date.cmp(b_date)
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
            .filter(|p| p.name == *name)
            .cloned()
            .collect::<Vec<Pipeline>>();
        if !pipelines.is_empty() {
            pipelines.sort_by_key(|e| e.clone().event.unwrap().date);
            pipeline = pipelines.pop().unwrap();
            Ok(pipeline)
        } else {
            let message = format!("Couldn't find a pipeline named {:?}, in logs", name);
            Err(Error::msg(message))
        }
    }
}
impl Logs {
    pub fn get_many_by_name(name: &str) -> Result<Vec<Pipeline>> {
        let pipelines = Logs::get()?;
        let mut pipelines = pipelines
            .iter()
            .filter(|p| p.name == *name)
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
                a_date.cmp(b_date)
            });
            Ok(pipelines)
        } else {
            let message = format!("Couldn't find a pipeline named {:?}, in logs", name);
            Err(Error::msg(message))
        }
    }
    pub fn get_many_by_sid(sid: &u32) -> Result<Vec<Pipeline>> {
        let pipelines = Logs::get()?;
        let mut pipelines = pipelines
            .iter()
            .filter(|p| {
                if p.event.clone().unwrap().sid.is_some() {
                    let p_sid = p.event.clone().unwrap().sid.unwrap();
                    &p_sid == sid
                } else {
                    false
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
                a_date.cmp(b_date)
            });
            Ok(pipelines)
        } else {
            let message = format!("Couldn't find a pipeline with sid {:?}, in logs", sid);
            Err(Error::msg(message))
        }
    }
}
