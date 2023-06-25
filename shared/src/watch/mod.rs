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

use std::path::Path;
use std::process::exit;
use std::thread;
use utils::{
    git::{Git, Hook},
    logger::Logger,
};

// Error Handling
use miette::{miette, Diagnostic, Error, IntoDiagnostic, NamedSource, Report, Result, SourceSpan};
use thiserror::Error;

// Globals
use super::cli::CLI;

pub fn watch_bin(attach: bool) -> Result<()> {
    trace!("Create detached subprocess");
    let bin = "pipelight";

    let mut args;
    unsafe {
        args = (*CLI).clone();
    }

    #[cfg(debug_assertions)]
    let command = format!("cargo run --bin {} {} --attach", &bin, &args);

    #[cfg(not(debug_assertions))]
    let command = format!("{} {} --attach", &bin, &args);

    match attach {
        true => {
            // Lauch attach thread
            watch_in_thread(attach)?;
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
pub fn watch(attach: bool) -> Result<()> {
    let config = Config::get()?;
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
                run::run_bin(pipeline.clone().name, attach);

                // let origin = env::current_dir().unwrap();
                // println!("{:?}", origin);
                // println!("{:?}", env);
            }
        }
    }
    Ok(())
}

/// Launch attached thread
pub fn watch_in_thread(attach: bool) -> Result<()> {
    let thread = thread::spawn(move || watch(attach).unwrap());
    thread.join().unwrap();
    Ok(())
}
