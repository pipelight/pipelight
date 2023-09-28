use crate::types::{Logs, Node, Pipeline};

// Getters
use crate::Getters;
mod getters;

// Tests
mod test;

use exec::{Statuable, Status};

// Error Handling
use miette::Result;

use crate::pipeline::Filters;

impl Logs {
    // Pretty print logs from json log file
    pub fn sanitize(pipelines: Vec<Pipeline>) -> Result<Vec<Pipeline>> {
        let mut pipelines = pipelines;
        pipelines = Filters::filter_by_status(pipelines, Some(Status::Running))?;
        for mut pipeline in pipelines.clone() {
            if pipeline.is_running().is_err() {
                pipeline.set_status(Some(Status::Aborted));
                println!("{}", Node::from(&pipeline));
            }
        }
        Ok(pipelines)
    }
    pub fn clean() -> Result<()> {
        let _pipelines = Self::get()?;
        Ok(())
    }
}
