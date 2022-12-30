#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_must_use)]
use crate::git::Git;
use crate::types::logs::{PipelineLog, PipelineStatus, StepLog};
use crate::types::{Config, Pipeline};
use log::LevelFilter::{Debug, Trace};
use log::{debug, error, info, trace, warn};
#[allow(dead_code)]
use project_root::get_project_root;
use std::env;
use std::env::current_dir;
use std::error::Error;
use std::path::Path;
use std::process::exit;

/// Launch attached subprocess
pub fn trigger() -> Result<(), Box<dyn Error>> {
    get_event();

    let config = Config::new()?;
    let git = Git::new();

    // Retrieve env
    // branch and triggering action
    let branch = git.branch();

    for pipeline in config.pipelines {
        if pipeline.triggers.is_none() {
            let message = format!("No triggers defined for pipeline: {:?}", &pipeline.name);
        }
        for trigger in pipeline.triggers.unwrap() {
            println!("{:?}", trigger.to_tuples()?);
            println!("{:?}", branch);
            // println!("{:?}", hook);

            // if (branch == trigger.branch)
        }
    }

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
fn get_event() -> Result<(), Box<dyn Error>> {
    let mut mode = "";
    let root = current_dir()?;
    let path_string = root.display().to_string();
    println!("{:?}", root);
    if path_string.contains("/.git/hooks/") {
        let hook = root
            .parent()
            .unwrap()
            .file_stem()
            .unwrap()
            .to_str()
            .unwrap();
        mode = hook;
    } else {
        mode = "manual";
    }
    println!("{:?}", &mode);
    Ok(())
}
