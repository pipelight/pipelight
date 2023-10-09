// Struct
use exec::Status;
use workflow::{Getters, Node, Pipeline};
// Error Handling
use miette::{Error, Result};

pub fn launch(pipeline_name: &str) -> Result<()> {
    // Guard
    let mut pipeline = Pipeline::get_by_name(&pipeline_name)?;
    if pipeline.is_triggerable().is_ok() {
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
        };
    }
    Ok(())
}
