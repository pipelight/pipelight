#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_must_use)]
use exec::Exec;
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

pub fn trigger_bin() -> Result<(), Box<dyn Error>> {
    trace!("Create detached subprocess");
    let bin = "pipelight-trigger";
    // let command = format!("cargo run --bin {} {}", bin, pipeline_name);
    let command = format!("{}", bin);
    Exec::new().detached(&command)?;
    Ok(())
}
/// Filter pipeline by trigger and run
pub fn trigger() -> Result<(), Box<dyn Error>> {
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
