#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_must_use)]
use crate::logger::{debug, error, info, set_logger, trace, warn};
use crate::types::logs::{PipelineLog, PipelineStatus, StepLog};
use crate::types::{Config, Pipeline};
use git2;
use log::LevelFilter::{Debug, Trace};
#[allow(dead_code)]
use project_root::get_project_root;
use std::env;
use std::env::current_dir;
use std::error::Error;
use std::path::Path;
use std::process::exit;

///  Detect git repo
pub fn is_git() -> Result<bool, Box<dyn Error>> {
    let root = current_dir()?;
    git2::Repository::discover(root)?;
    Ok(true)
}

/// Launch attached subprocess
pub fn trigger() -> Result<(), Box<dyn Error>> {
    set_logger(Trace);
    get_event();

    let config = Config::get()?;

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
