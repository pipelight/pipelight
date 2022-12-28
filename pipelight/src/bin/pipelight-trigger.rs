#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_must_use)]
use log::LevelFilter::{Debug, Trace};
#[allow(dead_code)]
use shared::config::{get_config, get_pipeline, is_git};
use shared::logger::{debug, error, info, set_logger, trace, warn};
use shared::types::config::Pipeline;
use shared::types::logs::{PipelineLog, PipelineStatus, StepLog};
use std::env;
use std::error::Error;
use std::path::Path;
use std::process::exit;

fn main() {
    handler().unwrap_or_else(|a| exit(1))
}
/// Launch attached subprocess
fn handler() -> Result<(), Box<dyn Error>> {
    set_logger(Trace);
    get_triggering_event();

    let config = get_config()?;

    for pipeline in &config.pipelines {
        if pipeline.triggers.is_none() {
            let message = format!("No triggers defined for pipeline: {:?}", &pipeline.name);
        }
        for trigger in &pipeline.triggers {
            println!("{:?}", trigger);
        }
    }

    // Check there is a git folder
    if !is_git()? {
        let message = "Couldn't detect git repository";
        debug!("{}", message);
    }

    // Retrieve env branch and triggering file (manual or git_hook)
    // get_branch()
    // get_triggering_event()

    // Retrieve triggers
    // get_triggers()
    //
    // Compare
    // if triggers.action.is_none(){
    //
    // }
    // if git_hook is in triggers.actions
    //     if branch is in triggers.branches
    //         run()
    //
    Ok(())
}
fn get_triggering_event() {
    let file_name = file!();
    let path = Path::new(file_name);
    let root = &path.parent().unwrap();
    println!("{:?}", root);
}
