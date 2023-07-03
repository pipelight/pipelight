use super::detach;
use super::run;
use log::{debug, trace};
use pipeline::{Config, Trigger};
use utils::git::Flag;

use std::thread;

// Error Handling
use miette::Result;

/// Function to be called from the cli.
/// Either spawn detached new processes or spawn attached threads
/// to run the triggerable pipelines
pub fn launch(attach: bool, flag: Option<String>) -> Result<()> {
    trace!("Create detached subprocess");
    // Run or Fork
    match attach {
        true => {
            // Lauch in attached thread
            trigger_in_thread(attach, flag)?;
        }
        false => detach()?,
    }
    Ok(())
}

/// Filter pipeline by trigger and run
pub fn trigger(attach: bool, flag: Option<String>) -> Result<()> {
    let config = Config::get()?;
    let mut env = Trigger::env()?;

    if flag.is_some() {
        env.set_action(Some(Flag::from(&flag.clone().unwrap())));
    }

    if config.pipelines.is_none() {
        let message = "No pipeline found";
        debug!("{}", message);
        return Ok(());
    }
    for pipeline in &config.pipelines.unwrap() {
        if pipeline.clone().triggers.is_none() {
            let message = format!("No triggers defined for pipeline: {:?}", &pipeline.name);
            debug!("{}", message)
        } else if pipeline.is_triggerable()? {
            run::launch(pipeline.clone().name, attach, flag.clone())?;
        }
    }
    Ok(())
}

/// Launch attached thread
pub fn trigger_in_thread(attach: bool, flag: Option<String>) -> Result<()> {
    let thread = thread::spawn(move || {
        // Action
        trigger(attach, flag).unwrap();
    });
    thread.join().unwrap();
    Ok(())
}
