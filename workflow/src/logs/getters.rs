// Structs
use crate::pipeline::Filters;
use crate::types::{Logs, Pipeline};
// Trait
use crate::traits::Getters;
// Date and Time
use chrono::{DateTime, Local};
// Error Handling
use miette::{Error, IntoDiagnostic, Result};
// Global vars
use once_cell::sync::Lazy;
use std::sync::{Arc, Mutex};

pub static LOGS: Lazy<Arc<Mutex<Option<Vec<Pipeline>>>>> = Lazy::new(|| Arc::new(Mutex::new(None)));

impl Logs {
    /**
    Read logs and store them into a global variable.
    Sorted by ascending date by default.
    */
    fn hydrate() -> Result<()> {
        let global_logs = LOGS.lock().unwrap().clone();
        if global_logs.is_none() {
            let logs: Vec<String> = cast::Logs::read(".pipelight/logs/")?;
            let mut pipelines: Vec<Pipeline> = vec![];
            for json in logs {
                let pipeline = serde_json::from_str::<Pipeline>(&json).into_diagnostic()?;
                pipelines.push(pipeline);
            }
            pipelines = Filters::sort_by_date_asc(pipelines)?;
            *LOGS.lock().unwrap() = Some(pipelines.clone());
        }
        Ok(())
    }
}

// Basic getters
impl Getters<Pipeline> for Logs {
    fn get() -> Result<Vec<Pipeline>> {
        Logs::hydrate()?;
        let logs = LOGS.lock().unwrap().clone().unwrap();
        Ok(logs)
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

// More getters
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
}
