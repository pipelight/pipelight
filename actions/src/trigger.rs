// Detach
use crate::utils::should_detach;
// Traits
use workflow::traits::Getters;
// Struct
use workflow::Pipeline;
// Error Handling
use miette::Result;
// Parallelism
use rayon::prelude::*;

pub fn launch() -> Result<()> {
    match should_detach()? {
        false => action()?,
        true => {}
    };
    Ok(())
}

/// Filter pipeline by trigger and run
pub fn action() -> Result<()> {
    let mut pipelines = Pipeline::get()?;
    pipelines.par_iter_mut().for_each(|pipeline| {
        if pipeline.is_triggerable().is_ok() {
            pipeline.run().unwrap();
        }
    });
    Ok(())
}
