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
use utils::{
    git::{Git, Hook},
    logger::Logger,
};

/// Filter pipeline by trigger and run
pub fn trigger() -> Result<(), Box<dyn Error>> {
    // Go to git root folder
    let git = Git::new();
    let config = Config::new();

    for pipeline in &config.pipelines.unwrap() {
        if pipeline.triggers.is_none() {
            let message = format!("No triggers defined for pipeline: {:?}", &pipeline.name);
        } else {
            for trigger in &pipeline.clone().triggers.unwrap() {
                let tuples = Trigger::flatten(trigger);
                if tuples.contains(&Trigger::env().unwrap()) {
                    let mut pipeline = Pipeline::from(pipeline);
                    pipeline.run()
                }
            }
        }
    }
    Ok(())
}
