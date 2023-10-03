use super::run;
// Detach
use crate::utils::detach;
// Traits
use log::{debug, trace};
use utils::git::Flag;
use workflow::traits::Getters;
// Struct
use workflow::{Pipeline, Trigger};
// Error Handling
use miette::Result;
// Parallelism
use rayon::prelude::*;

/// Filter pipeline by trigger and run
pub fn trigger() -> Result<()> {
    let mut pipelines = Pipeline::get()?;
    pipelines.par_iter_mut().for_each(|pipeline| {
        if pipeline.is_triggerable().is_ok() {
            pipeline.run().unwrap();
        }
    });
    Ok(())
}
