// Struct
use crate::services::types::{Action, Service};
use exec::Status;
use workflow::{Getters, Node, Pipeline};
// Globals
use crate::globals::CLI;
// Traits
use crate::services::traits::FgBg;
// Error Handling
use miette::{Error, Result};
use workflow::error::IsError;

pub fn launch(name: &str) -> Result<()> {
    let mut pipeline = Pipeline::get_by_name(name)?;

    // Guard
    if pipeline.is_triggerable()? {
        let args = CLI.lock().unwrap().clone();

        match args.attach {
            None => {
                Service::new(Action::RunLoose, Some(args))?.should_detach()?;
                Ok(())
            }
            Some(false) => {
                Service::new(Action::RunLoose, Some(args))?.should_detach()?;
                Ok(())
            }
            Some(true) => {
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
        }
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
        Err(IsError::new(message, &hint)?.into())
    }
}
