#![allow(unused_variables)]
#![allow(unused_must_use)]
use log::error;
use pipeline::types::Config;
use shared::trigger::trigger;
use std::env;

// Error Handling
use miette::Result;
use std::process::exit;

fn main() {
    handler().unwrap_or_else(|e| {
        error!("{}", e);
        exit(1)
    })
}

/// Launch detached subprocess
fn handler() -> Result<()> {
    run()?;
    Ok(())
}

// Get command line args and triggers pipelines
pub fn run() -> Result<()> {
    let mut attach: Option<&String> = None;

    let mut args = env::args().collect::<Vec<String>>();
    args.remove(0);

    let binding = args.clone();
    if args.clone().first().is_some() {
        attach = binding.first();
        args.remove(0);
    }

    Config::new(Some(args.clone()));

    if attach.is_some() {
        if attach.unwrap() == "--attach" {
            trigger(true)?;
        }
    } else {
        // Run pipelines in detached process
        trigger(false)?;
    }
    Ok(())
}
