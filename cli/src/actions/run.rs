// Struct
use exec::Status;
use workflow::{Getters, Node, Pipeline};
// Error Handling
use log::error;
use miette::{Error, Result};

pub fn launch(name: &str) -> Result<()> {
    // Guard
    let mut pipeline = Pipeline::get_by_name(&name)?;
    if pipeline.is_triggerable().is_ok() {
        // Action
        pipeline.run()?;
        // Return pipeline log
        println!("{}", Node::from(&pipeline));
        match pipeline.status {
            Some(Status::Succeeded) => return Ok(()),
            Some(Status::Failed) => {
                let message = "Pipeline status: Failed";
                return Err(Error::msg(message));
            }
            _ => return Ok(()),
        };
    }
    Ok(())
}
