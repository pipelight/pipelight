// Structs
use crate::types::Pipeline;
use pipelight_exec::Status;
use uuid::Uuid;
// Date and Time
use chrono::{DateTime, Local};
// Collections
use std::collections::HashMap;
// Error Handling
use log::warn;
use miette::{Error, Result};

pub struct Filters;
impl Filters {
    /**
    Keep only one pipeline among those which have the same name.
    */
    pub fn dedup(pipelines: Vec<Pipeline>) -> Result<Vec<Pipeline>> {
        let mut pipelines = pipelines;
        let init_length = &pipelines.len();
        pipelines.sort_by_key(|p| p.clone().name);
        pipelines.dedup_by_key(|p| p.clone().name);

        let end_length = &pipelines.len();
        if init_length != end_length {
            let message = "Removed pipelines with identical names";
            warn!("{}", message)
        }
        Ok(pipelines)
    }
    /**
     * Sort pipelines by ascending date.
     * The more recent pipeline is the last.
     * The oldes pipeline is the first.
     */
    pub fn sort_by_date_asc(pipelines: Vec<Pipeline>) -> Result<Vec<Pipeline>> {
        let mut pipelines = pipelines;
        if !pipelines.is_empty() {
            // Sort by date ascending
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
        }
        Ok(pipelines)
    }
    /**
     * Sort pipelines by descending date.
     * The more recent pipeline is the first.
     * The oldes pipeline is the last.
     */
    pub fn sort_by_date_desc(pipelines: Vec<Pipeline>) -> Result<Vec<Pipeline>> {
        let mut pipelines = pipelines;
        if !pipelines.is_empty() {
            // Sort by date descending
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
                a_date.cmp(b_date).reverse()
            });
        }
        Ok(pipelines)
    }
    /**
    Return the pipelines that have the provided status
    */
    pub fn filter_by_status(
        pipelines: Vec<Pipeline>,
        status: Option<Status>,
    ) -> Result<Vec<Pipeline>> {
        let mut pipelines = pipelines;
        pipelines.retain(|e| e.status == status);
        Ok(pipelines)
    }
    pub fn has_watch_flag(pipelines: Vec<Pipeline>) -> Result<()> {
        for pipeline in pipelines.clone() {
            if pipeline.is_watchable().is_ok() {
                return Ok(());
            }
        }
        let message = "no watchable pipelines";
        Err(Error::msg(message))
    }
    pub fn to_hashmap(pipelines: Vec<Pipeline>) -> HashMap<Uuid, Pipeline> {
        let map: HashMap<_, _> = pipelines.iter().map(|e| (e.uuid, e.to_owned())).collect();
        map
    }
    pub fn filter_by_name(pipelines: Vec<Pipeline>, name: &str) -> Result<Vec<Pipeline>> {
        let mut pipelines = pipelines;
        pipelines.retain(|e| e.name == name);
        Ok(pipelines)
    }
}
