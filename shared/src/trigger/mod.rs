#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unused_must_use)]
use crate::run;
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
use std::thread;
use utils::{
    git::{Git, Hook},
    logger::Logger,
};

pub fn trigger_bin(attach: bool, args: Option<Vec<String>>) -> Result<(), Box<dyn Error>> {
    trace!("Create detached subprocess");
    let bin = "pipelight-trigger";

    #[cfg(debug_assertions)]
    let mut command = format!("cargo run --bin {}", bin);

    #[cfg(not(debug_assertions))]
    let mut command = format!("{}", bin);

    if args.is_some() {
        command = format!("{} {}", command, args.unwrap().join(" "))
    }

    match attach {
        true => {
            // Lauch attach thread
            trigger_in_thread(attach)?;
        }
        false => {
            // Lauch detached process
            // trace!("Create detached subprocess");
            Exec::new().detached(&command)?;
        }
    }
    Ok(())
}

/// Filter pipeline by trigger and run
pub fn trigger(attach: bool) -> Result<(), Box<dyn Error>> {
    let config = Config::new(None)?;
    let env = Trigger::env()?;
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
            if pipeline.is_triggerable()? {
                run::run_bin(pipeline.clone().name, attach, None);

                // let origin = env::current_dir().unwrap();
                // println!("{:?}", origin);
                // println!("{:?}", env);
            }
        }
    }
    Ok(())
}

/// Launch attached thread
pub fn trigger_in_thread(attach: bool) -> Result<(), Box<dyn Error>> {
    let thread = thread::spawn(move || trigger(attach).unwrap());
    thread.join().unwrap();
    Ok(())
}
