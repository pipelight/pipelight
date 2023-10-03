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

// Usage of a global variable to avoid reading the same file multiple times.
pub static LOGS: Lazy<Arc<Mutex<Option<Vec<Pipeline>>>>> = Lazy::new(|| Arc::new(Mutex::new(None)));

impl Logs {
    /**
    Read logs and store them into a global variable.
    Sorted by ascending date by default.
    */
    pub fn hydrate(&mut self) -> Result<Self> {
        // Get global
        let global_logs = LOGS.lock().unwrap().clone();

        if global_logs.is_none() {
            // Read log files
            let json_logs: Vec<String> = cast::Logs::read(".pipelight/logs/")?;

            let mut pipelines: Vec<Pipeline> = vec![];
            for json in json_logs {
                let pipeline = serde_json::from_str::<Pipeline>(&json).into_diagnostic()?;
                pipelines.push(pipeline);
            }
            pipelines = Filters::sort_by_date_asc(pipelines)?;
            self.pipelines = Some(pipelines);

            // Set global
            *LOGS.lock().unwrap() = self.pipelines.clone();
        }
        Ok(self.to_owned())
    }
}

// Basic getters
impl Logs {
    pub fn get(&self) -> Result<Vec<Pipeline>> {
        let mut pipelines: Vec<Pipeline> = vec![];
        if let Some(e) = self.pipelines.clone() {
            pipelines = Filters::sort_by_date_asc(e)?;
        }
        Ok(pipelines)
    }
    pub fn get_by_name(&self, name: &str) -> Result<Pipeline> {
        if let Some(pipelines) = self.pipelines.clone() {
            let mut pipelines = pipelines
                .iter()
                .filter(|p| p.name == *name)
                .cloned()
                .collect::<Vec<Pipeline>>();
            pipelines = Filters::sort_by_date_asc(pipelines)?;
            let pipeline = pipelines.pop().unwrap();
            Ok(pipeline)
        } else {
            let message = format!("Couldn't find a pipeline named {:?}, in logs", name);
            Err(Error::msg(message))
        }
    }
}

// More getters
impl Logs {
    pub fn get_many_by_name(&self, name: &str) -> Result<Vec<Pipeline>> {
        if let Some(pipelines) = self.pipelines.clone() {
            let mut pipelines = pipelines
                .iter()
                .filter(|p| p.name == *name)
                .cloned()
                .collect::<Vec<Pipeline>>();
            pipelines = Filters::sort_by_date_asc(pipelines)?;
            Ok(pipelines)
        } else {
            let message = format!("Couldn't find a pipeline named {:?}, in logs", name);
            Err(Error::msg(message))
        }
    }
}
