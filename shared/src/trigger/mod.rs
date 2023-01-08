#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_must_use)]
use log::{debug, error, info, trace, warn};
use pipeline::{
    cast::Config,
    types::{Pipeline, Trigger},
};
#[allow(dead_code)]
use project_root::get_project_root;
use std::env;
use std::env::current_dir;
use std::error::Error;
use std::path::Path;
use std::process::exit;
use utils::git::Hook;
use utils::{
    git::{Git, GitHook},
    logger::Logs,
};

/// Launch attached subprocess
pub fn trigger() -> Result<(), Box<dyn Error>> {
    get_event();

    // Retrieve action
    let args = env::args().collect::<Vec<String>>();
    let action: Hook = Hook::from_str(&args[1]);

    // Retrieve branch
    let git = Git::new();
    let branch = git.get_branch().unwrap();

    let env = Trigger {
        action: Some(action),
        branch: Some(branch),
    };

    let config = Config::new()?;

    for pipeline in &config.pipelines.unwrap() {
        if pipeline.triggers.is_none() {
            let message = format!("No triggers defined for pipeline: {:?}", &pipeline.name);
        } else {
            for trigger in &pipeline.clone().triggers.unwrap() {
                let tuples = Trigger::flatten(trigger);
                if tuples.contains(&env) {
                    let mut pipeline = Pipeline::from(pipeline);
                }
            }
        }
    }
    Ok(())
}
fn get_event() -> Result<(), Box<dyn Error>> {
    let mut mode = "";
    let root = current_dir()?;
    let path_string = root.display().to_string();
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
    debug!("mode: {:?}", &mode);
    Ok(())
}
