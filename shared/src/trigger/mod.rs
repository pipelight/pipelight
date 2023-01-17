#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_must_use)]
use log::{debug, error, info, trace, warn};
use pipeline::types::{Config, Pipeline, Trigger};
#[allow(dead_code)]
use project_root::get_project_root;
use std::env;
use std::env::current_dir;
use std::error::Error;
use std::path::Path;
use std::process::exit;
use utils::{
    git::{Git, Hook},
    logger::Logger,
};

/// Filter pipeline by trigger and run
pub fn trigger() -> Result<(), Box<dyn Error>> {
    // Go to git root folder
    let config = Config::new();
    let env = Trigger::env()?;
    Git::new().teleport();
    if config.pipelines.is_none() {
        let message = "No pipeline found";
        debug!("{}", message);
        return Ok(());
    }
    for pipeline in &config.pipelines.unwrap() {
        if pipeline.clone().triggers.is_none() {
            let message = format!("No triggers defined for pipeline: {:?}", &pipeline.name);
            debug!("{}", message)
        } else {
            if pipeline.clone().triggers.unwrap().contains(&env) {
                pipeline.clone().run()
            }
        }
    }
    Ok(())
}
