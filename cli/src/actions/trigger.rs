// Struct
use crate::services::types::{Action, Service};
use crate::types::{Commands, DetachableCommands, Pipeline, PostCommands, Trigger};
use workflow;
// Traits
use crate::services::traits::FgBg;
use workflow::Getters;
// IterMut
use rayon::prelude::*;
// Globals
use crate::globals::CLI;
// Error Handling
use miette::Result;

pub fn launch(trigger: &Trigger) -> Result<()> {
    let mut pipelines = workflow::Pipeline::get()?;
    pipelines.par_iter_mut().for_each(|pipeline| {
        // Guard
        if pipeline.is_triggerable_strict().unwrap() {
            let mut args = CLI.lock().unwrap().clone();
            args.commands = Commands::PostCommands(PostCommands::DetachableCommands(
                DetachableCommands::Run(Pipeline {
                    trigger: trigger.to_owned(),
                    name: Some(pipeline.name.clone()),
                }),
            ));
            Service::new(Action::Run, Some(args))
                .unwrap()
                .should_detach()
                .unwrap();
        }
    });
    Ok(())
}
