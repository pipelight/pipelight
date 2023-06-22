#![allow(unused_variables)]
// #![allow(unused_imports)]
#![allow(unused_must_use)]
#[allow(dead_code)]
// use performance::*;

// Logger
use log::error;

// Command Line
use shared::cli;
// use shared::cli::types::Cli;

// use exec::Exec;
use std::process::exit;

// Error Handling
use miette::Result;

fn main() {
    handler().unwrap_or_else(|e| {
        error!("{}", e);
        exit(1)
    })
}
fn handler() -> Result<()> {
    // let args = cli::get_raw()?;
    // if args.is_some() {
    // } else {
    //     cli::get_args(None)?;
    // }
    Ok(())
}
