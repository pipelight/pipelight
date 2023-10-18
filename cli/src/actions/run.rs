// Struct
use exec::Status;
use workflow::{Getters, Node, Pipeline, Trigger};
// Error Handling
use log::debug;
use miette::{Error, Result};
use workflow::error::IsError;

pub fn launch(name: &str) -> Result<()> {
    // Guard
    let mut pipeline = Pipeline::get_by_name(&name)?;
    if pipeline.is_triggerable()? {
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
    } else {
        let mut string = "".to_owned();
        if let Some(triggers) = pipeline.triggers {
            // let actions = triggers.iter().map(|e| e.get_action());
            for trigger in triggers {
                string += &format!("{}\n", trigger);
            }
        }
        let mut hint = "".to_owned();
        hint += "Checkout to an authorize git branch or use an authorize action:\n";
        hint += &string;

        let message = "Can not trigger the pipeline in this environment";
        Err(IsError::new(&message, &hint)?.into())
    }
}
