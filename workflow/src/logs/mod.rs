// Struct
use crate::types::Logs;
// Getters
use crate::Getters;
mod getters;
// Tests
mod test;
// Traits
use exec::{Statuable, Status};
// IterMut
use rayon::prelude::*;
// Error Handling
use miette::Result;
// Global vars
use crate::globals::LOGS;

impl Logs {
    /**
    Find unreported aborted pipelines and update logs
    */
    pub fn sanitize(&mut self) -> Result<Self> {
        if let Some(mut pipelines) = self.pipelines.clone() {
            pipelines.par_iter_mut().for_each(|pipeline| {
                if pipeline.get_status() == Some(Status::Running) && !pipeline.is_running().unwrap()
                {
                    pipeline.set_status(Some(Status::Aborted));
                    pipeline.log();
                }
            });
            *LOGS.lock().unwrap() = Some(pipelines);
            self.pipelines = LOGS.lock().unwrap().clone();
        }
        Ok(self.to_owned())
    }
    /**
    Delete every logs but the ones from running pipelines
    */
    pub fn clean() -> Result<()> {
        let pipelines = Logs::get()?;
        for pipeline in pipelines {
            // Guard
            if pipeline.get_status() != Some(Status::Running) {
                pipeline.clean()?;
            }
        }
        Ok(())
    }
}
