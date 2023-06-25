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

// CLI
use super::cli::types::{Cli, Commands};
use clap::{Command, Parser};

// Globals
use super::cli::CLI;

/// To be called from the cli.
/// Either spawn detached new processes or spawn attached threads
/// to run the triggerable pipelines
pub fn trigger_bin(attach: bool, flag: Option<String>) -> Result<()> {
    trace!("Create detached subprocess");

    match attach {
        true => {
            // Lauch in attached thread
            trigger_in_thread(attach, flag)?;
        }
        false => {
            // Run a detached subprocess
            trace!("Create detached subprocess");
            let bin = "pipelight";
            let mut args;
            unsafe {
                args = (*CLI).clone();
            }
            args.attach = true;

            #[cfg(debug_assertions)]
            let command = format!("cargo run --bin {} {}", &bin, &args);

            #[cfg(not(debug_assertions))]
            let command = format!("{} {}", &bin, &args);

            // Lauch detached process
            // trace!("Create detached subprocess");
            Exec::new().detached(&command)?;
        }
    }
    Ok(())
}

/// Filter pipeline by trigger and run
pub fn trigger(attach: bool, flag: Option<String>) -> Result<()> {
    let config = Config::get()?;
    let mut env = Trigger::env()?;

    if flag.is_some() {
        env.set_action(flag);
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
pub fn trigger_in_thread(attach: bool, flag: Option<String>) -> Result<()> {
    let thread = thread::spawn(move || trigger(attach, flag).unwrap());
    thread.join().unwrap();
    Ok(())
}
