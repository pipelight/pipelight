// Traits
use workflow::traits::Getters;
// Struct
use crate::types::Action;
use workflow::Pipeline;
// Error Handling
use miette::Result;
// Parallelism
use rayon::prelude::*;

pub struct Triggerer;

impl Action {
    pub fn start(&self) -> Result<()> {
        match self {
            Action::Trigger() => {
                // Should set global flag here ??
                // Action::Trigger(flag) => {
                /**
                Filter pipeline by trigger and run
                */
                let mut pipelines = Pipeline::get()?;
                pipelines.par_iter_mut().for_each(|pipeline| {
                    if pipeline.is_triggerable().is_ok() {
                        pipeline.run().unwrap();
                    }
                });
            }
        }
        Ok(())
    }
}
