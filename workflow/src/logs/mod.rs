// Struct
use crate::pipeline::Filters;
use crate::types::{Logs, Node, Pipeline};
// Getters
use crate::Getters;
mod getters;
// Tests
mod test;
// Traits
use exec::{Statuable, Status};
// Error Handling
use miette::Result;

impl Logs {
    /**
    Find unreported aborted pipelines and update logs
    */
    pub fn sanitize(&mut self) -> Result<Self> {
        self.pipelines.as_mut().map(|e| {
            e.iter_mut().map(|pipeline| {
                if pipeline.get_status() == Some(Status::Running) && !pipeline.is_running().unwrap()
                {
                    pipeline.set_status(Some(Status::Aborted));
                    pipeline.log();
                }
            })
        });
        Ok(self.to_owned())
    }
    /**
    Delete every logs but the ones from running pipelines
    */
    pub fn clean() -> Result<()> {
        let pipelines = Logs::get()?;
        for pipeline in pipelines {
            if pipeline.get_status() != Some(Status::Running) {
                pipeline.clean()?;
            }
        }
        Ok(())
    }
}
