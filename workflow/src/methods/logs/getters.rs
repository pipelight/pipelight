// use crate::globals::PORTAL;
use crate::methods::pipeline::filters::Filters;
use crate::traits::Getters;
use crate::types::{Logs, Pipeline};
use cast;

// Date and Time
use chrono::{DateTime, Local};

// Error Handling
use miette::{Error, IntoDiagnostic, Result};

// Global vars
use once_cell::sync::Lazy;

pub static mut LOGS: Lazy<Option<Vec<Pipeline>>> = Lazy::new(|| None);

impl Logs {
    // Read logs and store them into a global var
    fn hydrate() -> Result<()> {
        // let portal;
        let global_logs;
        unsafe {
            // portal = (*PORTAL).clone();
            global_logs = (*LOGS).clone();
        };
        if global_logs.is_none() {
            let logs: Vec<String> = cast::Logs::read(&format!(
                // "{}/.pipelight/logs/",
                ".pipelight/logs/" // ,portal.target.directory_path.unwrap()
            ))?;
            let mut pipelines: Vec<Pipeline> = vec![];
            for json in logs {
                let pipeline = serde_json::from_str::<Pipeline>(&json).into_diagnostic()?;
                pipelines.push(pipeline);
            }
            // Sort by date ascending
            pipelines = Filters::sort_by_date_asc(pipelines)?;
            unsafe {
                *LOGS = Some(pipelines.clone());
            }
        }
        Ok(())
    }
}

// Basic getters
impl Getters<Pipeline> for Logs {
    fn get() -> Result<Vec<Pipeline>> {
        Logs::hydrate()?;
        let logs;
        unsafe { logs = (*LOGS).clone().unwrap() }
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
    pub fn _get_many_by_sid(sid: &u32) -> Result<Vec<Pipeline>> {
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
