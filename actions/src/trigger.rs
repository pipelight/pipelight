// Traits
use workflow::traits::Getters;
// Struct
use workflow::Pipeline;
// Error Handling
use miette::Result;
// Parallelism
use rayon::prelude::*;

// Should set global flag here ??
// Action::Trigger(flag) => {
/**
Filter pipeline by trigger and run
*/
pub fn launch() -> Result<()> {
    let mut pipelines = Pipeline::get()?;
    pipelines.par_iter_mut().for_each(|pipeline| {
        if pipeline.is_triggerable().is_ok() {
            pipeline.run().unwrap();
        }
    });
    Ok(())
}
