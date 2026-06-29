// Structs
use crate::pipeline::Filters;
use crate::types::{Logs, Pipeline};
use pipelight_git::Git;
// Trait
use crate::traits::Getters;
// Global vars
use crate::globals::LOGS;
// Error Handling
use log::warn;
use miette::{Error, IntoDiagnostic, Result};
use pipelight_error::PipelightError;

impl Logs {
    /// Read logs and store them into a global variable.
    /// Sorted by ascending date by default.
    pub fn hydrate(&mut self) -> Result<Self, PipelightError> {
        // Get global
        if LOGS.lock().unwrap().clone().is_none() {
            // Read log files
            let json_logs: Vec<String> = cast::Logs::read(".pipelight/logs")?;
            let mut pipelines: Vec<Pipeline> = vec![];
            for json in json_logs {
                match serde_json::from_str::<Pipeline>(&json) {
                    Ok(v) => {
                        pipelines.push(v);
                    }
                    Err(e) => {
                        // Ignore corrupted lines.
                        warn!("Stripped corrupted log files.");
                    }
                };
            }
            pipelines = Filters::sort_by_date_asc(pipelines)?;
            // Set global
            *LOGS.lock().unwrap() = Some(pipelines);
        }
        self.pipelines = LOGS.lock().unwrap().clone();
        Ok(self.to_owned())
    }
}

// Basic getters
impl Getters<Pipeline> for Logs {
    fn get() -> Result<Vec<Pipeline>> {
        let logs = Logs::default().hydrate()?.sanitize()?;
        match logs.pipelines {
            Some(mut pipelines) => {
                pipelines = Filters::sort_by_date_asc(pipelines)?;
                Ok(pipelines)
            }
            None => Err(Error::msg("Couldn't get a pipeline log from log files")),
        }
    }
    fn get_by_name(name: &str) -> Result<Pipeline> {
        let mut pipelines = Logs::get()?;
        pipelines = pipelines
            .iter()
            .filter(|p| p.name == name)
            .cloned()
            .collect::<Vec<Pipeline>>();
        match pipelines.pop() {
            None => {
                let message = format!("Couldn't find a pipeline named {:?}, in logs", name);
                Err(Error::msg(message))
            }
            Some(p) => Ok(p),
        }
    }
}

// More getters
impl Logs {
    pub fn get_many_by_name(name: &str) -> Result<Vec<Pipeline>> {
        let mut pipelines = Logs::get()?;
        pipelines = pipelines
            .iter()
            .filter(|p| p.name == *name)
            .cloned()
            .collect::<Vec<Pipeline>>();
        match pipelines.is_empty() {
            false => Ok(pipelines),
            true => {
                let message = format!("Couldn't find a pipeline named {:?}, in logs", name);
                Err(Error::msg(message))
            }
        }
    }
}
