// Detach
use crate::utils::should_detach;
// Struct
use exec::Status;
use workflow::{Getters, Node, Pipeline};
// Error Handling
use miette::{Error, Result};

/**
Run the pipeline.
Detach it if needed.
and return fancy log to stdout.
*/
pub fn launch(pipeline_name: &str) -> Result<()> {
    // Guard
    let mut pipeline = Pipeline::get_by_name(&pipeline_name)?;
    pipeline.is_triggerable()?;
    should_detach()?;

    // Action
    pipeline.run()?;

    // Return pipeline log
    println!("{}", Node::from(&pipeline));
    match pipeline.status {
        Some(Status::Succeeded) => Ok(()),
        Some(Status::Failed) => {
            let message = "Pipeline status: Failed";
            Err(Error::msg(message))
        }
        _ => Ok(()),
    }
}
