// Error Handling
use log::warn;
use miette::{Result};
//Types
use crate::types::{Pipeline};
use exec::Status;
// Date and Time
use chrono::{DateTime, Local};

pub struct Filters;

impl Filters {
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
    pub fn sort_by_date_asc(pipelines: Vec<Pipeline>) -> Result<Vec<Pipeline>> {
        let mut pipelines = pipelines;
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
        Ok(pipelines)
    }
    pub fn sort_by_date_desc(pipelines: Vec<Pipeline>) -> Result<Vec<Pipeline>> {
        let mut pipelines = pipelines;
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
        Ok(pipelines)
    }
    pub fn filter_by_status(
        pipelines: Vec<Pipeline>,
        status: Option<Status>,
    ) -> Result<Vec<Pipeline>> {
        let mut pipelines = pipelines;
        pipelines = pipelines
            .into_iter()
            .filter(|e| e.status == status)
            .collect();
        Ok(pipelines)
    }
}
