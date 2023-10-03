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
    /**
    Find unreported aborted pipelines and update logs
    */
    pub fn sanitize(&mut self) -> Result<Self> {
        let mut pipelines: Vec<Pipeline> = vec![];
        if let Some(e) = self.pipelines.clone() {
            pipelines = Filters::filter_by_status(e, Some(Status::Running))?;
            for mut pipeline in pipelines.clone() {
                if pipeline.is_running().is_err() {
                    pipeline.set_status(Some(Status::Aborted));
                    pipeline.log();
                }
            }
        }
        Ok(self.to_owned())
    }
    /**
    Delete every logs but the ones from running pipelines
    */
    pub fn clean(&mut self) -> Result<()> {
        let pipelines: Vec<Pipeline> = self.hydrate()?.sanitize()?.get()?;
        for pipeline in pipelines {
            if pipeline.is_running().is_err() {
                pipeline.clean()?;
            }
        }
        Ok(())
    }
}
